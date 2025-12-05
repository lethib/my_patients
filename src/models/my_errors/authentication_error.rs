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

  #[allow(non_snake_case, dead_code)]
  pub fn INVALID_AUTH_TOKEN() -> MyErrors {
    MyErrors {
      code: StatusCode::UNAUTHORIZED,
      msg: "invalid_auth_token".to_string(),
    }
  }

  #[allow(dead_code)]
  pub fn new(msg: String) -> MyErrors {
    MyErrors {
      code: StatusCode::UNAUTHORIZED,
      msg,
    }
  }
}
