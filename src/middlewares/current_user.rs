use crate::models::{
  _entities::users,
  my_errors::{authentication_error::AuthenticationError, MyErrors},
};
use axum::{
  extract::{Request, State},
  middleware::Next,
  response::Response,
};
use loco_rs::{app::AppContext, auth, prelude::*};

pub async fn current_user_middleware(
  ctx: State<AppContext>,
  request: Request,
  next: Next,
) -> Result<Response, MyErrors> {
  let auth_header = request
    .headers()
    .get("Authorization")
    .and_then(|h| h.to_str().ok())
    .ok_or_else(|| {
      tracing::error!("Authorization header not found");
      AuthenticationError::INVALID_AUTH_TOKEN.to_my_error()
    })?;

  let token = auth_header.strip_prefix("Bearer ").ok_or_else(|| {
    tracing::error!("No Bearer token found in Authorization header");
    AuthenticationError::INVALID_AUTH_TOKEN.to_my_error()
  })?;

  let jwt_config = ctx
    .config
    .get_jwt_config()
    .map_err(|_| AuthenticationError::INVALID_AUTH_TOKEN.to_my_error())?;

  let claims = auth::jwt::JWT::new(&jwt_config.secret)
    .validate(token)
    .map_err(|_| AuthenticationError::INVALID_AUTH_TOKEN.to_my_error())?;

  let user = users::Model::find_by_pid(&ctx.db, &claims.claims.pid).await?;
  ctx.shared_store.insert(user);

  Ok(next.run(request).await)
}

pub trait CurrentUser {
  /// Get a reference to the current authenticated user from the shared store.
  /// Returns None if no user is authenticated or stored in the context.
  fn current_user(&self) -> loco_rs::app::RefGuard<'_, users::Model>;

  /// Get a clone of the current authenticated user from the shared store.
  /// Returns None if no user is authenticated or stored in the context.
  /// Note: This requires the User model to implement Clone.
  fn current_user_cloned(&self) -> users::Model;
}

impl CurrentUser for AppContext {
  fn current_user(&self) -> loco_rs::app::RefGuard<'_, users::Model> {
    self
      .shared_store
      .get_ref::<users::Model>()
      .expect("Current user not found in shared store")
  }

  fn current_user_cloned(&self) -> users::Model {
    self
      .shared_store
      .get::<users::Model>()
      .expect("Current user not found in shared store")
  }
}
