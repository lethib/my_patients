use crate::{
  app_state::{AppState, CurrentUserExt},
  auth::jwt::JwtService,
  models::{
    _entities::users,
    my_errors::{
      application_error::ApplicationError, authentication_error::AuthenticationError,
      unexpected_error::UnexpectedError, MyErrors,
    },
    users::{LoginParams, RegisterParams},
  },
  services,
  views::auth::{CurrentResponse, LoginResponse},
};
use axum::{debug_handler, extract::State, http::StatusCode, Json};
use sea_orm::IntoActiveModel;
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

  if !user.is_access_key_verified {
    return Err(MyErrors {
      code: StatusCode::SEE_OTHER,
      msg: "access_key_needs_to_be_verified".to_string(),
    });
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

#[derive(Deserialize)]
pub struct CheckAccessKeyParams {
  access_key: String,
  user_email: String,
}

#[debug_handler]
pub async fn check_access_key(
  State(state): State<AppState>,
  Json(params): Json<CheckAccessKeyParams>,
) -> Result<Json<serde_json::Value>, MyErrors> {
  let user = users::Model::find_by_email(&state.db, &params.user_email)
    .await
    .map_err(|_| UnexpectedError::SHOULD_NOT_HAPPEN())?;

  if services::user::check_access_key(&user, params.access_key) {
    users::ActiveModel::enable_access(&mut user.clone().into_active_model(), &state.db).await?;

    let jwt_service = JwtService::new(&state.config.jwt.secret);
    let token = jwt_service
      .generate_token(&user.pid.to_string(), state.config.jwt.expiration)
      .map_err(|error| UnexpectedError::new(error.to_string()))?;

    return Ok(Json(serde_json::json!({ "token": token })));
  }

  Err(ApplicationError::new("access_key_not_recognized".into()))
}
