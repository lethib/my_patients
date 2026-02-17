use crate::models::my_errors::MyErrors;
use axum::http::StatusCode;

pub enum ApplicationError {
  UnprocessableEntity,
  NotFound,
  BadRequest,
  #[allow(non_camel_case_types)]
  new(&'static str),
}

impl From<ApplicationError> for MyErrors {
  fn from(err: ApplicationError) -> Self {
    match err {
      ApplicationError::UnprocessableEntity => MyErrors {
        code: StatusCode::UNPROCESSABLE_ENTITY,
        msg: "unprocessable_entity".into(),
      },
      ApplicationError::NotFound => MyErrors {
        code: StatusCode::NOT_FOUND,
        msg: "resource_not_found".into(),
      },
      ApplicationError::BadRequest => MyErrors {
        code: StatusCode::BAD_REQUEST,
        msg: "bad_request".into(),
      },
      ApplicationError::new(msg) => MyErrors {
        code: StatusCode::BAD_REQUEST,
        msg: msg.to_string(),
      },
    }
  }
}
