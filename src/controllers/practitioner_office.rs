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
use sea_orm::{
  ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, IntoActiveModel, ModelTrait,
  QueryFilter,
};

use crate::{
  middlewares::current_user::{current_user_middleware, CurrentUser},
  models::{
    _entities::{practitioner_offices, user_practitioner_offices},
    my_errors::{application_error::ApplicationError, unexpected_error::UnexpectedError, MyErrors},
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
  let office = practitioner_offices::Entity::find_by_id(office_id)
    .inner_join(user_practitioner_offices::Entity)
    .filter(user_practitioner_offices::Column::UserId.eq(ctx.current_user().0.id))
    .one(&ctx.db)
    .await?
    .ok_or(ApplicationError::NOT_FOUND())?;

  let mut office = office.clone().into_active_model();
  office.name = Set(params.name.trim().to_string());
  office.address_line_1 = Set(params.address_line_1.trim().to_string());
  office.address_zip_code = Set(params.address_zip_code.trim().to_string());
  office.address_city = Set(params.address_city.trim().to_string());

  office.update(&ctx.db).await?;

  Ok(format::json(serde_json::json!({ "success": true }))?)
}

async fn destroy(
  State(ctx): State<AppContext>,
  Path(office_id): Path<i32>,
) -> Result<Response, MyErrors> {
  let office = practitioner_offices::Entity::find_by_id(office_id)
    .inner_join(user_practitioner_offices::Entity)
    .filter(user_practitioner_offices::Column::UserId.eq(ctx.current_user().0.id))
    .one(&ctx.db)
    .await?
    .ok_or(ApplicationError::NOT_FOUND())?;

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
