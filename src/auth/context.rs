use crate::{
  app_state::AppState,
  auth::{
    jwt::{JwtService, TOKEN_TYPE_AUTH},
    statement::AuthStatement,
  },
  models::{
    _entities::user_business_informations,
    my_errors::{authentication_error::AuthenticationError, MyErrors},
    users,
  },
};

pub struct AuthContext {
  pub current_user: Option<(users::Model, Option<user_business_informations::Model>)>,
  authorized: bool,
  complete: bool,
  error: Option<MyErrors>,
}

impl AuthContext {
  pub async fn new(auth_header: Option<&str>, state: &AppState) -> Self {
    let (current_user, error) = match auth_header {
      Some(header) => Self::validate_auth_header(header, state).await,
      None => (None, None),
    };

    Self {
      current_user,
      authorized: false,
      complete: false,
      error,
    }
  }

  pub fn authorize(self) -> AuthStatement {
    AuthStatement::new(self)
  }

  pub(super) fn authorized(&mut self) {
    self.panic_if_already_completed();
    self.authorized = true;
  }

  pub(super) fn not_authorized(&mut self, error: Option<MyErrors>) {
    self.panic_if_already_completed();
    self.authorized = false;

    if self.error.is_none() {
      self.error = error;
    }
  }

  pub(super) fn complete(&mut self) -> Result<(), MyErrors> {
    self.panic_if_already_completed();
    self.complete = true;

    if !self.authorized {
      match self.error.take() {
        Some(error) => return Err(error),
        None => return Err(AuthenticationError::ACCESS_DENIED(None)),
      }
    }

    Ok(())
  }

  pub(super) async fn validate_auth_header(
    auth_header: &str,
    state: &AppState,
  ) -> (
    Option<(users::Model, Option<user_business_informations::Model>)>,
    Option<MyErrors>,
  ) {
    let token = match auth_header.strip_prefix("Bearer ") {
      Some(t) => t,
      None => return (None, Some(AuthenticationError::MISSING_TOKEN())),
    };

    let jwt_service = JwtService::new(&state.config.jwt.secret);
    let claims = match jwt_service.validate_token(token) {
      Ok(data) => data,
      Err(_) => return (None, Some(AuthenticationError::INVALID_TOKEN())),
    };

    if claims.token_type != TOKEN_TYPE_AUTH {
      return (None, Some(AuthenticationError::INVALID_TOKEN()));
    }

    let user_result = match users::Model::find_by_pid(&state.db, &claims.pid).await {
      Ok(user) => user,
      Err(_) => return (None, Some(AuthenticationError::INVALID_CLAIMS())),
    };

    if !user_result.0.is_access_key_verified {
      return (
        Some(user_result),
        Some(AuthenticationError::ACCESS_KEY_NOT_VERIFIED()),
      );
    }

    (Some(user_result), None)
  }

  fn panic_if_already_completed(&self) {
    if self.complete {
      panic!("auth_context_already_completed")
    }
  }
}
