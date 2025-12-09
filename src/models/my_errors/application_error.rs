use crate::models::my_errors::MyErrors;
use axum::http::StatusCode;

pub struct ApplicationError {}

impl ApplicationError {
  #[allow(non_snake_case)]
  pub fn UNPROCESSABLE_ENTITY() -> MyErrors {
    MyErrors {
      code: StatusCode::UNPROCESSABLE_ENTITY,
      msg: "unprocessable_entity".into(),
    }
  }

  #[allow(non_snake_case)]
  pub fn NOT_FOUND() -> MyErrors {
    MyErrors {
      code: StatusCode::NOT_FOUND,
      msg: "resource_not_found".into(),
    }
  }

  pub fn new(msg: String) -> MyErrors {
    MyErrors {
      code: StatusCode::NOT_FOUND,
      msg,
    }
  }
}
