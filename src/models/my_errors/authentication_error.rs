use crate::models::my_errors::{MyErrors, ToErr};

#[allow(non_camel_case_types)]
pub enum AuthenticationError {
  INVALID_CREDENTIALS,
  new(String),
}

impl ToErr for AuthenticationError {
  fn to_err<_T>(self) -> Result<_T, MyErrors> {
    match self {
      AuthenticationError::INVALID_CREDENTIALS => Err(MyErrors {
        code: axum::http::StatusCode::UNAUTHORIZED,
        msg: "invalid_credentials".into(),
      }),
      AuthenticationError::new(msg) => Err(MyErrors {
        code: axum::http::StatusCode::UNAUTHORIZED,
        msg,
      }),
    }
  }
}
