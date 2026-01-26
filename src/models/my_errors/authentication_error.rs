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

  #[allow(non_snake_case)]
  pub fn MISSING_TOKEN() -> MyErrors {
    MyErrors {
      code: StatusCode::UNAUTHORIZED,
      msg: "missing_token".to_string(),
    }
  }

  #[allow(non_snake_case)]
  pub fn INVALID_TOKEN() -> MyErrors {
    MyErrors {
      code: StatusCode::UNAUTHORIZED,
      msg: "invalid_token".to_string(),
    }
  }

  #[allow(non_snake_case)]
  pub fn INVALID_CLAIMS() -> MyErrors {
    MyErrors {
      code: StatusCode::UNAUTHORIZED,
      msg: "invalid_claims_inside_token".to_string(),
    }
  }

  #[allow(non_snake_case)]
  pub fn ACCESS_KEY_NOT_VERIFIED() -> MyErrors {
    MyErrors {
      code: StatusCode::UNAUTHORIZED,
      msg: "access_key_not_verified".to_string(),
    }
  }

  #[allow(non_snake_case)]
  pub fn ACCESS_DENIED(to: Option<String>) -> MyErrors {
    MyErrors {
      code: StatusCode::FORBIDDEN,
      msg: format!("access_denied_to_{}", to.unwrap_or("resource".to_string())),
    }
  }
}
