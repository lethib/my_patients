use crate::models::my_errors::MyErrors;
use axum::http::StatusCode;

pub struct ApplicationError {}

impl ApplicationError {
  #[allow(non_snake_case)]
  pub fn UNPROCESSABLE_ENTITY() -> MyErrors {
    MyErrors {
      code: StatusCode::BAD_REQUEST,
      msg: "unprocessable_entity".into(),
    }
  }

  pub fn new(msg: String) -> MyErrors {
    MyErrors {
      code: StatusCode::BAD_REQUEST,
      msg,
    }
  }
}
