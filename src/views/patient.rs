use crate::models::patients;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PatientResponse {
  pub first_name: Option<String>,
  pub last_name: Option<String>,
  pub ssn: Option<String>,
}

impl PatientResponse {
  #[must_use]
  pub fn new(patient: &Option<patients::Model>) -> Self {
    match patient {
      Some(patients) => Self {
        first_name: Some(patients.first_name.clone()),
        last_name: Some(patients.last_name.clone()),
        ssn: patients.decrypt_ssn().ok(),
      },
      None => Self {
        first_name: None,
        last_name: None,
        ssn: None,
      },
    }
  }

  #[must_use]
  pub fn from_model(patient: &patients::Model) -> Self {
    Self {
      first_name: Some(patient.first_name.clone()),
      last_name: Some(patient.last_name.clone()),
      ssn: patient.decrypt_ssn().ok(),
    }
  }
}
