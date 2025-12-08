use crate::models::my_errors::MyErrors;
use axum::http::StatusCode;
pub struct AuthenticationError {}

impl AuthenticationError {
  #[allow(non_snake_case)]
  pub fn INVALID_CREDENTIALS() -> MyErrors {
    MyErrors {
      code: StatusCode::UNAUTHORIZED,
      msg: "invalid_credentials".to_string(),
    }
  }
}
