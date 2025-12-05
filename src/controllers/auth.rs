use crate::{
  app_state::{AppState, CurrentUserExt},
  auth::jwt::JwtService,
  models::{
    _entities::users,
    my_errors::{authentication_error::AuthenticationError, MyErrors},
    users::{LoginParams, RegisterParams},
  },
  views::auth::{CurrentResponse, LoginResponse},
};
use axum::{debug_handler, extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ForgotParams {
  pub email: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResetParams {
  pub token: String,
  pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MagicLinkParams {
  pub email: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResendVerificationParams {
  pub email: String,
}

/// Register function creates a new user with the given parameters
#[debug_handler]
pub async fn register(
  State(state): State<AppState>,
  Json(params): Json<RegisterParams>,
) -> Result<Json<()>, MyErrors> {
  users::Model::create_with_password(&state.db, &params).await?;
  Ok(Json(()))
}

/// In case the user forgot his password, this endpoint generates a forgot token
/// and sends email to the user. In case the email is not found in our DB, we are
/// returning a valid request for security reasons (not exposing users DB list).
#[debug_handler]
pub async fn forgot(
  State(state): State<AppState>,
  Json(params): Json<ForgotParams>,
) -> Result<Json<()>, MyErrors> {
  let Ok(_user) = users::Model::find_by_email(&state.db, &params.email).await else {
    // we don't want to expose our users' emails. if the email is invalid we still
    // return success to the caller
    return Ok(Json(()));
  };

  // TODO: implement the forgot password logic

  Ok(Json(()))
}

/// Reset user password by the given parameters
#[debug_handler]
pub async fn reset(
  State(_state): State<AppState>,
  Json(_params): Json<ResetParams>,
) -> Result<Json<()>, MyErrors> {
  return Err(MyErrors {
    code: StatusCode::BAD_REQUEST,
    msg: "not implemented".to_string(),
  });
}

/// Creates a user login and returns a token
#[debug_handler]
pub async fn login(
  State(state): State<AppState>,
  Json(params): Json<LoginParams>,
) -> Result<Json<LoginResponse>, MyErrors> {
  let user = users::Model::find_by_email(&state.db, &params.email)
    .await
    .map_err(|_| AuthenticationError::INVALID_CREDENTIALS())?;

  let valid = user.verify_password(&params.password);

  if !valid {
    return Err(AuthenticationError::INVALID_CREDENTIALS());
  }

  let jwt_service = JwtService::new(&state.config.jwt.secret);
  let token = jwt_service
    .generate_token(&user.pid.to_string(), state.config.jwt.expiration)
    .map_err(|_| MyErrors {
      code: StatusCode::UNAUTHORIZED,
      msg: "Failed to generate token".to_string(),
    })?;

  Ok(Json(LoginResponse::new(&user, &token)))
}

/// Get current authenticated user
#[debug_handler]
pub async fn me(
  CurrentUserExt(user, business_info): CurrentUserExt,
) -> Result<Json<CurrentResponse>, MyErrors> {
  Ok(Json(CurrentResponse::new(&(user, business_info))))
}
