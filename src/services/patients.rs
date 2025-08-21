use sea_orm::TransactionTrait;

use crate::initializers::get_services;
use crate::models::patient_users::{self, CreateLinkParams};
use crate::models::{
  my_errors::MyErrors,
  patients::{self, CreatePatientParams, Model as PatientModel},
  users,
};

pub async fn create(
  patient_params: &CreatePatientParams,
  linked_to_user: &users::Model,
) -> Result<PatientModel, MyErrors> {
  let services = get_services();

  let db_transaction = services.db.begin().await?;

  let created_patient = patients::ActiveModel::create(&db_transaction, patient_params).await?;

  patient_users::ActiveModel::create(
    &db_transaction,
    &CreateLinkParams {
      user_id: linked_to_user.id,
      patient_id: created_patient.id,
    },
  )
  .await?;

  db_transaction.commit().await?;

  Ok(created_patient)
}
