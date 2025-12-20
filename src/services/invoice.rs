use crate::{
  app_state::{AppState, WorkerJob},
  initializers::get_services,
  models::{
    _entities::{patients, practitioner_offices::Entity as PractitionerOffices, users},
    medical_appointments::{ActiveModel as MedicalAppointments, CreateMedicalAppointmentParams},
    my_errors::{application_error::ApplicationError, unexpected_error::UnexpectedError, MyErrors},
    patients as PatientModel,
  },
  workers::{
    self,
    invoice_generator::InvoiceGeneratorArgs,
    mailer::{args::EmailArgs, attachment::EmailAttachment},
  },
};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

// Supabase Storage helpers
async fn upload_to_supabase_storage(
  file_data: &[u8],
  filename: &str,
  bucket_name: &str,
  content_type: &str,
) -> Result<(), MyErrors> {
  let supabase_url =
    std::env::var("SUPABASE_URL").map_err(|_| UnexpectedError::SHOULD_NOT_HAPPEN())?;
  let supabase_key =
    std::env::var("SUPABASE_SERVICE_ROLE_KEY").map_err(|_| UnexpectedError::SHOULD_NOT_HAPPEN())?;

  let client = reqwest::Client::new();
  let url = format!(
    "{}/storage/v1/object/{}/{}",
    supabase_url, bucket_name, filename
  );

  let response = client
    .post(&url)
    .header("Authorization", format!("Bearer {}", supabase_key))
    .header("Content-Type", content_type)
    .body(file_data.to_vec())
    .send()
    .await
    .map_err(|e| {
      tracing::error!("Failed to send upload request: {}", e);
      UnexpectedError::SHOULD_NOT_HAPPEN()
    })?;

  if response.status().is_success() {
    Ok(())
  } else {
    let status = response.status();
    let error_text = response.text().await.unwrap_or_default();
    tracing::error!(
      "Failed to upload to Supabase storage ({}): {}",
      status,
      error_text
    );
    Err(UnexpectedError::SHOULD_NOT_HAPPEN().into())
  }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GenerateInvoiceParams {
  pub amount: String,
  pub invoice_date: String,
  pub should_be_sent_by_email: bool,
  pub practitioner_office_id: i32,
}

pub struct GenerateInvoiceResponse {
  pub pdf_data: Vec<u8>,
  pub filename: String,
  patient_email: String,
  invoice_date: chrono::NaiveDate,
}

pub async fn upload_signature_for_user(
  _user: &users::Model,
  signature_data: &[u8],
  filename: &str,
  content_type: &str,
) -> Result<(), MyErrors> {
  let bucket_name =
    std::env::var("SUPABASE_SIGNATURE_BUCKET").map_err(|_| UnexpectedError::SHOULD_NOT_HAPPEN())?;

  upload_to_supabase_storage(signature_data, filename, &bucket_name, content_type).await
}

pub async fn send_invoice(
  state: &AppState,
  generated_invoice: &GenerateInvoiceResponse,
  current_user: &users::Model,
) -> Result<(), MyErrors> {
  if generated_invoice.patient_email == PatientModel::DEFAULT_EMAIL {
    return Err(ApplicationError::UNPROCESSABLE_ENTITY());
  }

  let attachment = EmailAttachment::from_bytes(
    generated_invoice.filename.to_string(),
    "application/pdf".to_string(),
    &generated_invoice.pdf_data,
  );

  let invoice_date = generated_invoice
    .invoice_date
    .format("%d/%m/%Y")
    .to_string();

  let args = EmailArgs::new_text(
    generated_invoice.patient_email.clone(),
    format!("Note d'honoraires {}", invoice_date),
    format!(
      "Vous trouverez ci-joint votre facture pour la consultation du {}\n\n{} {}\nOSTEOPATHE D.O.\n{}",
      invoice_date, current_user.last_name, current_user.first_name, current_user.phone_number
    ),
  )
  .with_attachment(attachment)
  .with_reply_to(current_user.email.to_string());

  // Enqueue email job via worker channel
  state
    .worker_transmitter
    .send(WorkerJob::Email(args))
    .await
    .map_err(|_| UnexpectedError::SHOULD_NOT_HAPPEN())?;

  Ok(())
}

pub async fn generate_patient_invoice(
  patient_id: &i32,
  params: &GenerateInvoiceParams,
  current_user: &users::Model,
) -> Result<GenerateInvoiceResponse, MyErrors> {
  let services = get_services();

  let patient = patients::Entity::find_by_id(*patient_id)
    .filter(patients::Column::UserId.eq(current_user.id))
    .one(&services.db)
    .await?
    .ok_or(ApplicationError::NOT_FOUND())?;

  let invoice_date = chrono::NaiveDate::parse_from_str(&params.invoice_date, "%Y-%m-%d")?;

  let filename = format!(
    "{} {} Note d'honoraires - {} {} {}.pdf",
    current_user.first_name,
    current_user.last_name.to_uppercase(),
    &patient.last_name,
    &patient.first_name,
    invoice_date.format("%d_%m_%Y")
  );

  let medical_appointment_params = CreateMedicalAppointmentParams {
    user_id: current_user.id,
    patient_id: *patient_id,
    practitioner_office_id: params.practitioner_office_id,
    date: invoice_date,
  };

  let created_medical_appointment =
    MedicalAppointments::create(&services.db, &medical_appointment_params).await?;

  let practitioner_office =
    PractitionerOffices::find_by_id(created_medical_appointment.practitioner_office_id)
      .one(&services.db)
      .await?
      .ok_or(UnexpectedError::SHOULD_NOT_HAPPEN())?;

  let args = InvoiceGeneratorArgs {
    patient: patient.clone(),
    user: current_user.clone(),
    amount: params.amount.clone(),
    invoice_date,
    practitioner_office,
  };

  let pdf_data = workers::invoice_generator::generate_invoice_pdf(&services.db, &args).await?;

  Ok(GenerateInvoiceResponse {
    pdf_data,
    filename,
    patient_email: patient.email,
    invoice_date,
  })
}
