use serde::{Deserialize, Serialize};

use crate::models::_entities::practitioner_offices;

#[derive(Debug, Deserialize, Serialize)]
pub struct PractitionerOffice {
  id: i32,
  pub name: String,
}

impl PractitionerOffice {
  pub fn new(office: &practitioner_offices::Model) -> Self {
    Self {
      id: office.id,
      name: office.name.clone(),
    }
  }
}
