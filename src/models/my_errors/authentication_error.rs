use crate::models::my_errors::{MyErrors, ToErr};

#[allow(non_camel_case_types)]
pub enum AuthenticationError {
  INVALID_CREDENTIALS,
  INVALID_AUTH_TOKEN,
  new(String),
}

impl AuthenticationError {
  pub fn to_my_error(self) -> MyErrors {
    match self {
      AuthenticationError::INVALID_CREDENTIALS => MyErrors {
        code: axum::http::StatusCode::UNAUTHORIZED,
        msg: "invalid_credentials".into(),
      },
      AuthenticationError::INVALID_AUTH_TOKEN => MyErrors {
        code: axum::http::StatusCode::UNAUTHORIZED,
        msg: "invalid_authentication_token".into(),
      },
      AuthenticationError::new(msg) => MyErrors {
        code: axum::http::StatusCode::UNAUTHORIZED,
        msg,
      },
    }
  }
}

impl ToErr for AuthenticationError {
  fn to_err<_T>(self) -> Result<_T, MyErrors> {
    Err(self.to_my_error())
  }
}
