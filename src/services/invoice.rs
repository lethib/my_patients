use crate::{
  initializers::get_services,
  models::{
    _entities::{patient_users, patients, practitioner_offices, users},
    my_errors::{application_error::ApplicationError, MyErrors},
  },
  workers::{
    self,
    invoice_generator::InvoiceGeneratorArgs,
    mailer::{args::EmailArgs, attachment::EmailAttachment, worker::EmailWorker},
  },
};
use loco_rs::prelude::*;
use sea_orm::{QuerySelect, RelationTrait};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct GenerateInvoiceParams {
  pub amount: String,
  pub invoice_date: String,
}

pub struct GenerateInvoiceResponse {
  pub pdf_data: Vec<u8>,
  pub filename: String,
  patient_email: String,
  invoice_date: chrono::NaiveDate,
}

pub async fn send_invoice(
  ctx: &AppContext,
  generated_invoice: &GenerateInvoiceResponse,
  current_user: &users::Model,
) -> Result<(), MyErrors> {
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
    format!("Votre consulation du {}", invoice_date),
    format!(
      "Vous trouverez ci-joint votre facture pour la consultation du {}",
      invoice_date
    ),
  )
  .with_attachment(attachment)
  .with_reply_to(current_user.email.to_string());

  EmailWorker::perform_later(ctx, args).await?;

  Ok(())
}

pub async fn generate_patient_invoice(
  patient_id: &i32,
  params: &GenerateInvoiceParams,
  current_user: &users::Model,
) -> Result<GenerateInvoiceResponse, MyErrors> {
  let services = get_services();

  let (patient, practitioner_office) = patients::Entity::find_by_id(*patient_id)
    .inner_join(patient_users::Entity)
    .join(
      sea_orm::JoinType::InnerJoin,
      patient_users::Relation::PractitionerOffices.def(),
    )
    .filter(patient_users::Column::UserId.eq(current_user.id))
    .select_also(practitioner_offices::Entity)
    .one(&services.db)
    .await?
    .ok_or_else(|| ApplicationError::UNPROCESSABLE_ENTITY.to_my_error())?;

  let practitioner_office =
    practitioner_office.ok_or_else(|| ApplicationError::UNPROCESSABLE_ENTITY.to_my_error())?;

  let invoice_date = chrono::NaiveDate::parse_from_str(&params.invoice_date, "%Y-%m-%d")?;

  let filename = format!(
    "{} {} Note d'honoraires - {} {} {}.pdf",
    current_user.first_name,
    current_user.last_name.to_uppercase(),
    &patient.last_name,
    &patient.first_name,
    invoice_date.format("%d_%m_%Y")
  );

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
