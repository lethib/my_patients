use opencab::models::practitioner_offices::{
  ActiveModel as OfficeActiveModel, Model as OfficeModel, PractitionerOfficeParams,
};
use sea_orm::DatabaseConnection;

pub struct OfficeFactory {
  name: String,
  address_line_1: String,
  address_zip_code: String,
  address_city: String,
}

impl Default for OfficeFactory {
  fn default() -> Self {
    Self {
      name: "Cabinet Central".to_string(),
      address_line_1: "1 rue de la Paix".to_string(),
      address_zip_code: "75001".to_string(),
      address_city: "Paris".to_string(),
    }
  }
}

impl OfficeFactory {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn name(mut self, name: &str) -> Self {
    self.name = name.to_string();
    self
  }

  pub async fn create(self, db: &DatabaseConnection) -> OfficeModel {
    OfficeActiveModel::create(
      db,
      &PractitionerOfficeParams {
        name: self.name,
        address_line_1: self.address_line_1,
        address_zip_code: self.address_zip_code,
        address_city: self.address_city,
      },
    )
    .await
    .unwrap()
  }
}
