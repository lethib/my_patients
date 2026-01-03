use crate::{
  app_state::{AppState, CurrentUserExt},
  auth::jwt::{JwtService, TOKEN_TYPE_AUTH},
  models::{
    _entities::users,
    my_errors::{authentication_error::AuthenticationError, MyErrors},
  },
};
use axum::{
  extract::{Request, State},
  middleware::Next,
  response::Response,
};

pub async fn auth_middleware(
  State(state): State<AppState>,
  mut request: Request,
  next: Next,
) -> Result<Response, MyErrors> {
  let auth_header = request
    .headers()
    .get("Authorization")
    .and_then(|h| h.to_str().ok())
    .ok_or_else(|| {
      tracing::error!("Authorization header not found");
      AuthenticationError::MISSING_TOKEN()
    })?;

  let token = auth_header.strip_prefix("Bearer ").ok_or_else(|| {
    tracing::error!("No Bearer token found in Authorization header");
    AuthenticationError::INVALID_TOKEN()
  })?;

  let jwt_service = JwtService::new(&state.config.jwt.secret);
  let claims = jwt_service.validate_token(token).map_err(|e| {
    tracing::error!("JWT validation failed: {}", e);
    AuthenticationError::INVALID_TOKEN()
  })?;

  if claims.token_type != TOKEN_TYPE_AUTH {
    tracing::error!(
      "Invalid token type: expected 'auth', got '{}'",
      claims.token_type
    );
    return Err(AuthenticationError::INVALID_TOKEN());
  }

  let user_result = users::Model::find_by_pid(&state.db, &claims.pid)
    .await
    .map_err(|e| {
      tracing::error!("Failed to load user from database: {:?}", e);
      AuthenticationError::INVALID_CLAIMS()
    })?;

  if !user_result.0.is_access_key_verified {
    return Err(AuthenticationError::ACCESS_KEY_NOT_VERIFIED());
  }

  // Insert user into request extensions
  request
    .extensions_mut()
    .insert(CurrentUserExt(user_result.0, user_result.1));

  Ok(next.run(request).await)
}
