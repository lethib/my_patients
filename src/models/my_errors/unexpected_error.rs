use crate::models::my_errors::{MyErrors, ToErr};

#[allow(non_camel_case_types)]
pub enum UnexpectedError {
  SHOULD_NOT_HAPPEN,
  new(String),
}

impl UnexpectedError {
  pub fn to_my_error(self) -> MyErrors {
    match self {
      UnexpectedError::SHOULD_NOT_HAPPEN => MyErrors {
        code: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        msg: "should_not_happen".into(),
      },
      UnexpectedError::new(msg) => MyErrors {
        code: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        msg: msg,
      },
    }
  }
}

impl ToErr for UnexpectedError {
  fn to_err<_T>(self) -> Result<_T, super::MyErrors> {
    Err(self.to_my_error())
  }
}
