use axum::{
  debug_handler,
  extract::{Path, State},
  Json,
};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, EntityTrait, IntoActiveModel, ModelTrait};

use crate::{
  app_state::AppState,
  auth::statement::AuthStatement,
  middleware::auth::AuthenticatedUser,
  models::{
    _entities::practitioner_offices,
    my_errors::{application_error::ApplicationError, MyErrors},
    practitioner_offices::PractitionerOfficeParams,
  },
  services,
};

#[debug_handler]
pub async fn create(
  State(_state): State<AppState>,
  AuthenticatedUser(current_user, _): AuthenticatedUser,
  Json(params): Json<PractitionerOfficeParams>,
) -> Result<Json<serde_json::Value>, MyErrors> {
  services::practitioner_office::create(&params, &current_user).await?;

  Ok(Json(serde_json::json!({ "success": true })))
}

#[debug_handler]
pub async fn update(
  State(state): State<AppState>,
  authorize: AuthStatement,
  Path(office_id): Path<i32>,
  Json(params): Json<PractitionerOfficeParams>,
) -> Result<Json<serde_json::Value>, MyErrors> {
  let office = practitioner_offices::Entity::find_by_id(office_id)
    .one(&state.db)
    .await?
    .ok_or(ApplicationError::NOT_FOUND())?;

  authorize.is_owning_resource(&office).await.run_complete()?;

  let mut office = office.clone().into_active_model();
  office.name = Set(params.name.trim().to_string());
  office.address_line_1 = Set(params.address_line_1.trim().to_string());
  office.address_zip_code = Set(params.address_zip_code.trim().to_string());
  office.address_city = Set(params.address_city.trim().to_string());

  office.update(&state.db).await?;

  Ok(Json(serde_json::json!({ "success": true })))
}

#[debug_handler]
pub async fn destroy(
  State(state): State<AppState>,
  authorize: AuthStatement,
  Path(office_id): Path<i32>,
) -> Result<Json<serde_json::Value>, MyErrors> {
  let office = practitioner_offices::Entity::find_by_id(office_id)
    .one(&state.db)
    .await?
    .ok_or(ApplicationError::NOT_FOUND())?;

  authorize.is_owning_resource(&office).await.run_complete()?;

  office.clone().delete(&state.db).await?;

  Ok(Json(serde_json::json!({ "success": true })))
}
