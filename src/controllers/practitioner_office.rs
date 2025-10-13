use axum::{
  debug_handler,
  extract::{Path, State},
  middleware,
  response::Response,
  routing::{delete, post, put},
  Json,
};
use loco_rs::{
  app::AppContext,
  prelude::{format, Routes},
};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, IntoActiveModel, ModelTrait};

use crate::{
  middlewares::current_user::{current_user_middleware, CurrentUser},
  models::{
    my_errors::{unexpected_error::UnexpectedError, MyErrors},
    practitioner_offices::PractitionerOfficeParams,
  },
  services,
};

#[debug_handler]
async fn create(
  State(ctx): State<AppContext>,
  Json(params): Json<PractitionerOfficeParams>,
) -> Result<Response, MyErrors> {
  services::practitioner_office::create(&params, &ctx.current_user().0).await?;

  Ok(format::json(serde_json::json!({ "success": true }))?)
}

async fn update(
  State(ctx): State<AppContext>,
  Path(office_id): Path<i32>,
  Json(params): Json<PractitionerOfficeParams>,
) -> Result<Response, MyErrors> {
  let user_offices = ctx.current_user().0.get_my_offices(&ctx.db).await?;

  let office = user_offices
    .iter()
    .find(|&office| office.id == office_id)
    .ok_or_else(|| UnexpectedError::SHOULD_NOT_HAPPEN.to_my_error())?;

  let mut office = office.clone().into_active_model();
  office.name = Set(params.name.clone());
  office.address_line_1 = Set(params.address_line_1.clone());
  office.address_zip_code = Set(params.address_zip_code.clone());
  office.address_city = Set(params.address_city.clone());

  office.update(&ctx.db).await?;

  Ok(format::json(serde_json::json!({ "success": true }))?)
}

async fn destroy(
  State(ctx): State<AppContext>,
  Path(office_id): Path<i32>,
) -> Result<Response, MyErrors> {
  let user_offices = ctx.current_user().0.get_my_offices(&ctx.db).await?;

  let office = user_offices
    .iter()
    .find(|&office| office.id == office_id)
    .ok_or_else(|| UnexpectedError::SHOULD_NOT_HAPPEN.to_my_error())?;

  office.clone().delete(&ctx.db).await?;

  Ok(format::json(serde_json::json!({ "success": true }))?)
}

pub fn routes(ctx: &AppContext) -> Routes {
  Routes::new()
    .prefix("/api/practitioner_office")
    .add("/create", post(create))
    .add("/{office_id}", put(update))
    .add("/{office_id}", delete(destroy))
    .layer(middleware::from_fn_with_state(
      ctx.clone(),
      current_user_middleware,
    ))
}
