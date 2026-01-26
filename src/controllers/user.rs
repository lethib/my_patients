use crate::{
  app_state::AppState,
  middleware::auth::AuthenticatedUser,
  models::{
    _entities::prelude::UserBusinessInformations,
    my_errors::{application_error::ApplicationError, unexpected_error::UnexpectedError, MyErrors},
    user_business_informations::CreateBusinessInformation,
  },
  services::{self, storage::StorageService},
  views::practitioner_office::PractitionerOffice,
};
use axum::{
  debug_handler,
  extract::{Multipart, State},
  http::status,
  Json,
};
use image::{imageops::FilterType, ImageFormat};
use sea_orm::{ActiveModelTrait, ActiveValue, IntoActiveModel, ModelTrait};

#[debug_handler]
pub async fn save_business_info(
  State(_state): State<AppState>,
  AuthenticatedUser(current_user, _): AuthenticatedUser,
  Json(business_information): Json<CreateBusinessInformation>,
) -> Result<Json<serde_json::Value>, MyErrors> {
  services::user::save_business_information(&business_information, &current_user).await?;

  Ok(Json(serde_json::json!({ "success": true })))
}

#[debug_handler]
pub async fn my_offices(
  State(state): State<AppState>,
  AuthenticatedUser(current_user, _): AuthenticatedUser,
) -> Result<Json<Vec<PractitionerOffice>>, MyErrors> {
  let my_offices = current_user.get_my_offices(&state.db).await?;

  let serialized_offices: Vec<PractitionerOffice> = my_offices
    .iter()
    .map(|office| PractitionerOffice::new(office))
    .collect();

  Ok(Json(serialized_offices))
}

#[debug_handler]
pub async fn get_signature_url(
  State(_state): State<AppState>,
  AuthenticatedUser(_current_user, user_bi): AuthenticatedUser,
) -> Result<String, MyErrors> {
  let storage = StorageService::new()?;
  let signature_filename = user_bi
    .ok_or(UnexpectedError::SHOULD_NOT_HAPPEN())?
    .signature_file_name
    .ok_or(UnexpectedError::SHOULD_NOT_HAPPEN())?;

  Ok(storage.signature_url(&signature_filename))
}

#[debug_handler]
pub async fn upload_signature(
  State(state): State<AppState>,
  AuthenticatedUser(current_user, _): AuthenticatedUser,
  mut multipart: Multipart,
) -> Result<status::StatusCode, MyErrors> {
  let field = multipart
    .next_field()
    .await
    .map_err(|_| ApplicationError::BAD_REQUEST())?
    .ok_or(ApplicationError::BAD_REQUEST())?;

  let field_name = field.name().ok_or(ApplicationError::BAD_REQUEST())?;
  if field_name != "signature" {
    return Err(ApplicationError::BAD_REQUEST());
  }

  let signature_data = field
    .bytes()
    .await
    .map_err(|_| ApplicationError::UNPROCESSABLE_ENTITY())?;

  let img = image::load_from_memory(&signature_data).map_err(|e| {
    tracing::error!("Failed to load image: {}", e);
    ApplicationError::UNPROCESSABLE_ENTITY()
  })?;

  let resized = img.resize_exact(314, 156, FilterType::Lanczos3);

  let mut png_bytes: Vec<u8> = Vec::new();
  resized
    .write_to(&mut std::io::Cursor::new(&mut png_bytes), ImageFormat::Png)
    .map_err(|e| {
      tracing::error!("Failed to encode image: {}", e);
      ApplicationError::UNPROCESSABLE_ENTITY()
    })?;

  let filename = format!(
    "{}_{}_{}",
    &current_user.first_name.to_lowercase(),
    &current_user.last_name.to_lowercase(),
    &current_user.id.to_string()
  );

  let storage_service = services::storage::StorageService::new()?;
  storage_service
    .upload_signature(&png_bytes, &filename, "image/png")
    .await?;

  let mut business_information = current_user
    .find_related(UserBusinessInformations)
    .one(&state.db)
    .await?
    .ok_or(ApplicationError::UNPROCESSABLE_ENTITY())?
    .into_active_model();

  business_information.signature_file_name = ActiveValue::Set(Some(filename));
  business_information.update(&state.db).await?;

  Ok(status::StatusCode::NO_CONTENT)
}
