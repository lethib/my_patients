use crate::initializers::get_services;
use crate::models::{
  my_errors::MyErrors,
  patients::{self, CreatePatientParams, Model as PatientModel},
};

pub async fn create(patient_params: &CreatePatientParams) -> Result<PatientModel, MyErrors> {
  let services = get_services();
  let created_patient = patients::ActiveModel::create(&services.db, patient_params).await?;
  Ok(created_patient)
}
