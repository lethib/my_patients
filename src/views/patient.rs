use crate::models::{_entities::practitioner_offices, patients};
use practitioner_offices::Model as PractitionerOffice;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct OfficeIdAndName {
  id: i32,
  name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PatientResponse {
  id: i32,
  pub first_name: String,
  pub last_name: String,
  pub email: Option<String>,
  pub ssn: String,
  pub address_line_1: String,
  pub address_zip_code: String,
  pub address_city: String,
  pub address_country: String,
  office: Option<OfficeIdAndName>,
}

impl PatientResponse {
  #[must_use]
  pub fn new(
    patient: &Option<patients::Model>,
    office: Option<&PractitionerOffice>,
  ) -> Option<Self> {
    patient.as_ref().map(|patient| Self {
      id: patient.id.clone(),
      first_name: patient.first_name.clone(),
      last_name: patient.last_name.clone(),
      email: (patient.email != "default@mail.com").then(|| patient.email.clone()),
      ssn: patient
        .decrypt_ssn()
        .unwrap_or_else(|_| "Unable to decrypt".to_string()),
      address_line_1: patient.address_line_1.clone(),
      address_zip_code: patient.address_zip_code.clone(),
      address_city: patient.address_city.clone(),
      address_country: patient.address_country.clone(),
      office: office.map(|office| OfficeIdAndName {
        id: office.id,
        name: office.name.clone(),
      }),
    })
  }

  #[must_use]
  pub fn from_model(patient: &patients::Model, office: &PractitionerOffice) -> Self {
    Self {
      id: patient.id.clone(),
      first_name: patient.first_name.clone(),
      last_name: patient.last_name.clone(),
      email: (patient.email != "default@mail.com").then(|| patient.email.clone()),
      ssn: patient
        .decrypt_ssn()
        .unwrap_or_else(|_| "Unable to decrypt".to_string()),
      address_line_1: patient.address_line_1.clone(),
      address_zip_code: patient.address_zip_code.clone(),
      address_city: patient.address_city.clone(),
      address_country: patient.address_country.clone(),
      office: Some(OfficeIdAndName {
        id: office.id,
        name: office.name.clone(),
      }),
    }
  }
}
