use crate::models::my_errors::{MyErrors, ToErr};

#[allow(non_camel_case_types)]
pub enum ApplicationError {
  UNPROCESSABLE_ENTITY,
  new(String),
}

impl ApplicationError {
  pub fn to_my_error(self) -> MyErrors {
    match self {
      ApplicationError::UNPROCESSABLE_ENTITY => MyErrors {
        code: axum::http::StatusCode::BAD_REQUEST,
        msg: "unprocessable_entity".into(),
      },
      ApplicationError::new(msg) => MyErrors {
        code: axum::http::StatusCode::BAD_REQUEST,
        msg: msg,
      },
    }
  }
}

impl ToErr for ApplicationError {
  fn to_err<_T>(self) -> Result<_T, super::MyErrors> {
    Err(self.to_my_error())
  }
}
