use serde::{Deserialize, Serialize};

use crate::{
  models::_entities::{user_business_informations, users},
  views::user::BusinessInformation,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginResponse {
  pub token: String,
  pub pid: String,
  pub name: String,
}

impl LoginResponse {
  #[must_use]
  pub fn new(user: &users::Model, token: &String) -> Self {
    Self {
      token: token.to_string(),
      pid: user.pid.to_string(),
      name: user.name.clone(),
    }
  }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CurrentResponse {
  pub pid: String,
  pub name: String,
  pub email: String,
  pub business_information: Option<BusinessInformation>,
}

impl CurrentResponse {
  #[must_use]
  pub fn new(user: &(users::Model, Option<user_business_informations::Model>)) -> Self {
    Self {
      pid: user.0.pid.to_string(),
      name: user.0.name.clone(),
      email: user.0.email.clone(),
      business_information: user
        .1
        .as_ref()
        .map(|business_information| BusinessInformation::new(&business_information)),
    }
  }
}
