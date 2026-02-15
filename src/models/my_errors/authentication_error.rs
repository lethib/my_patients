use crate::models::my_errors::MyErrors;
use axum::http::StatusCode;

pub enum AuthenticationError {
  InvalidCredentials,
  MissingToken,
  InvalidToken,
  InvalidClaims,
  AccessKeyNotVerified,
  AccessDenied(Option<String>),
}

impl From<AuthenticationError> for MyErrors {
  fn from(err: AuthenticationError) -> Self {
    match err {
      AuthenticationError::InvalidCredentials => MyErrors {
        code: StatusCode::UNAUTHORIZED,
        msg: "invalid_credentials".to_string(),
      },
      AuthenticationError::MissingToken => MyErrors {
        code: StatusCode::UNAUTHORIZED,
        msg: "missing_token".to_string(),
      },
      AuthenticationError::InvalidToken => MyErrors {
        code: StatusCode::UNAUTHORIZED,
        msg: "invalid_token".to_string(),
      },
      AuthenticationError::InvalidClaims => MyErrors {
        code: StatusCode::UNAUTHORIZED,
        msg: "invalid_claims".to_string(),
      },
      AuthenticationError::AccessKeyNotVerified => MyErrors {
        code: StatusCode::UNAUTHORIZED,
        msg: "access_key_not_verified".to_string(),
      },
      AuthenticationError::AccessDenied(to) => MyErrors {
        code: StatusCode::UNAUTHORIZED,
        msg: format!("access_denied_to_{}", to.unwrap_or("resource".to_string())),
      },
    }
  }
}
