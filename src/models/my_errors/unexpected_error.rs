use crate::models::my_errors::MyErrors;
use axum::http::StatusCode;

pub struct UnexpectedError {}

impl UnexpectedError {
  #[allow(non_snake_case)]
  pub fn SHOULD_NOT_HAPPEN() -> MyErrors {
    MyErrors {
      code: StatusCode::INTERNAL_SERVER_ERROR,
      msg: "should_not_happen".into(),
    }
  }

  pub fn new(msg: String) -> MyErrors {
    MyErrors {
      code: StatusCode::INTERNAL_SERVER_ERROR,
      msg: msg,
    }
  }
}
