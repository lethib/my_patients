use my_patients::models::users::{Model as UserModel, RegisterParams};
use sea_orm::DatabaseConnection;

pub struct UserFactory {
  email: String,
  password: String,
  first_name: String,
  last_name: String,
  phone_number: String,
}

impl Default for UserFactory {
  fn default() -> Self {
    Self {
      email: "doctor@test.com".to_string(),
      password: "password123".to_string(),
      first_name: "John".to_string(),
      last_name: "Doe".to_string(),
      phone_number: "0600000000".to_string(),
    }
  }
}

impl UserFactory {
  pub fn new() -> Self {
    Self::default()
  }

  pub async fn create(self, db: &DatabaseConnection) -> UserModel {
    UserModel::create_with_password(
      db,
      &RegisterParams {
        email: self.email,
        password: self.password,
        first_name: self.first_name,
        last_name: self.last_name,
        phone_number: self.phone_number,
      },
    )
    .await
    .unwrap()
  }
}
