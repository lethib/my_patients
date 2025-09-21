use crate::{
  initializers::get_services,
  models::{
    _entities::{patient_users, patients, users},
    my_errors::{application_error::ApplicationError, MyErrors},
  },
  workers::{self, invoice_generator::InvoiceGeneratorArgs},
};
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct GenerateInvoiceParams {
  pub amount: String,
}

pub struct GenerateInvoiceResponse {
  pub pdf_data: Vec<u8>,
  pub filename: String,
}

pub async fn generate_patient_invoice(
  patient_id: i32,
  params: &GenerateInvoiceParams,
  current_user: &users::Model,
) -> Result<GenerateInvoiceResponse, MyErrors> {
  let services = get_services();

  let patient = patients::Entity::find_by_id(patient_id)
    .inner_join(patient_users::Entity)
    .filter(patient_users::Column::UserId.eq(current_user.id))
    .one(&services.db)
    .await?
    .ok_or_else(|| ApplicationError::UNPROCESSABLE_ENTITY.to_my_error())?;

  let args = InvoiceGeneratorArgs {
    patient,
    user: current_user.clone(),
    amount: params.amount.clone(),
  };

  let pdf_data = workers::invoice_generator::generate_invoice_pdf(&services.db, &args).await?;

  // Return PDF as binary response with appropriate headers
  let filename = format!(
    "invoice_patient_{}_{}.pdf",
    patient_id,
    chrono::Utc::now().format("%Y%m%d_%H%M%S")
  );

  Ok(GenerateInvoiceResponse { pdf_data, filename })
}
