use crate::models::patients;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PatientResponse {
  id: i32,
  pub first_name: String,
  pub last_name: String,
  pub ssn: String,
  pub address_line_1: String,
  pub address_zip_code: String,
  pub address_city: String,
  pub address_country: String,
  pub office: Option<String>,
}

impl PatientResponse {
  #[must_use]
  pub fn new(patient: &Option<patients::Model>, office_name: Option<&String>) -> Option<Self> {
    patient.as_ref().map(|patients| Self {
      id: patients.id.clone(),
      first_name: patients.first_name.clone(),
      last_name: patients.last_name.clone(),
      ssn: patients
        .decrypt_ssn()
        .unwrap_or_else(|_| "Unable to decrypt".to_string()),
      address_line_1: patients.address_line_1.clone(),
      address_zip_code: patients.address_zip_code.clone(),
      address_city: patients.address_city.clone(),
      address_country: patients.address_country.clone(),
      office: office_name.cloned(),
    })
  }

  #[must_use]
  pub fn from_model(patient: &patients::Model, office_name: &String) -> Self {
    Self {
      id: patient.id.clone(),
      first_name: patient.first_name.clone(),
      last_name: patient.last_name.clone(),
      ssn: patient
        .decrypt_ssn()
        .unwrap_or_else(|_| "Unable to decrypt".to_string()),
      address_line_1: patient.address_line_1.clone(),
      address_zip_code: patient.address_zip_code.clone(),
      address_city: patient.address_city.clone(),
      address_country: patient.address_country.clone(),
      office: Some(office_name.clone()),
    }
  }
}
