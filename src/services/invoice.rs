use crate::{
  initializers::get_services,
  models::{
    _entities::{patient_users, patients, practitioner_offices, users},
    my_errors::{application_error::ApplicationError, MyErrors},
  },
  workers::{self, invoice_generator::InvoiceGeneratorArgs},
};
use loco_rs::prelude::*;
use sea_orm::{QuerySelect, RelationTrait};
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

  let (patient, practitioner_office) = patients::Entity::find_by_id(patient_id)
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

  let filename = format!(
    "{} {} Note d'honoraires - {} {} {}.pdf",
    current_user.first_name,
    current_user.last_name.to_uppercase(),
    &patient.last_name,
    &patient.first_name,
    chrono::Utc::now().format("%d_%m_%Y")
  );

  let args = InvoiceGeneratorArgs {
    patient,
    user: current_user.clone(),
    amount: params.amount.clone(),
    practitioner_office,
  };

  let pdf_data = workers::invoice_generator::generate_invoice_pdf(&services.db, &args).await?;

  Ok(GenerateInvoiceResponse { pdf_data, filename })
}
