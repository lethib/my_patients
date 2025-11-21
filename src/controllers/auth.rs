use crate::{
  middlewares::current_user::{current_user_middleware, CurrentUser},
  models::{
    _entities::users,
    my_errors::{authentication_error::AuthenticationError, MyErrors},
    users::{LoginParams, RegisterParams},
  },
  views::auth::{CurrentResponse, LoginResponse},
};
use axum::debug_handler;
use loco_rs::prelude::*;
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

/// Register function creates a new user with the given parameters and sends a
/// welcome email to the user
#[debug_handler]
async fn register(
  State(ctx): State<AppContext>,
  Json(params): Json<RegisterParams>,
) -> Result<Response, MyErrors> {
  users::Model::create_with_password(&ctx.db, &params).await?;
  Ok(format::json(())?)
}

/// In case the user forgot his password  this endpoints generate a forgot token
/// and send email to the user. In case the email not found in our DB, we are
/// returning a valid request for for security reasons (not exposing users DB
/// list).
#[debug_handler]
async fn forgot(
  State(ctx): State<AppContext>,
  Json(params): Json<ForgotParams>,
) -> Result<Response> {
  let Ok(_user) = users::Model::find_by_email(&ctx.db, &params.email).await else {
    // we don't want to expose our users email. if the email is invalid we still
    // returning success to the caller
    return format::json(());
  };

  // TODO: implement the forgot password logic

  format::json(())
}

/// reset user password by the given parameters
#[debug_handler]
async fn reset(
  State(_ctx): State<AppContext>,
  Json(_params): Json<ResetParams>,
) -> Result<Response> {
  return Err(loco_rs::errors::Error::BadRequest("not implemented".into()));
  // let Ok(user) = users::Model::find_by_email(&ctx.db, &params.token).await else {
  //     // we don't want to expose our users email. if the email is invalid we still
  //     // returning success to the caller
  //     tracing::info!("reset token not found");

  //     return format::json(());
  // };
  // user.into_active_model()
  //     .reset_password(&ctx.db, &params.password)
  //     .await?;

  // format::json(())
}

/// Creates a user login and returns a token
#[debug_handler]
async fn login(
  State(ctx): State<AppContext>,
  Json(params): Json<LoginParams>,
) -> Result<Response, MyErrors> {
  let user = users::Model::find_by_email(&ctx.db, &params.email)
    .await
    .map_err(|_| AuthenticationError::INVALID_CREDENTIALS())?;
  let valid = user.verify_password(&params.password);

  if !valid {
    return Err(AuthenticationError::INVALID_CREDENTIALS());
  }

  let jwt_secret = ctx.config.get_jwt_config()?;

  let token = user
    .generate_jwt(&jwt_secret.secret, jwt_secret.expiration)
    .or_else(|_| unauthorized("unauthorized!"))?;

  Ok(format::json(LoginResponse::new(&user, &token))?)
}

#[debug_handler]
async fn me(State(ctx): State<AppContext>) -> Result<Response, MyErrors> {
  Ok(format::json(CurrentResponse::new(&ctx.current_user()))?)
}

pub fn routes(ctx: &AppContext) -> Routes {
  Routes::new()
    .prefix("/api/auth")
    .add("/register", post(register))
    .add("/login", post(login))
    .add("/forgot", post(forgot))
    .add("/reset", post(reset))
    .add(
      "/me",
      get(me).layer(axum::middleware::from_fn_with_state(
        ctx.clone(),
        current_user_middleware,
      )),
    )
}
