use serde::{Deserialize, Serialize};

use crate::models::_entities::practitioner_offices;

#[derive(Debug, Deserialize, Serialize)]
pub struct PractitionerOffice {
  id: i32,
  pub name: String,
  pub address_line_1: String,
  pub address_zip_code: String,
  pub address_city: String,
}

impl PractitionerOffice {
  pub fn new(office: &practitioner_offices::Model) -> Self {
    Self {
      id: office.id,
      name: office.name.clone(),
      address_line_1: office.address_line_1.clone(),
      address_zip_code: office.address_zip_code.clone(),
      address_city: office.address_city.clone(),
    }
  }
}
