use crate::{
  app_state::{AppState, CurrentUserExt},
  models::{
    my_errors::{application_error::ApplicationError, MyErrors},
    user_business_informations::CreateBusinessInfomation,
  },
  services,
  views::practitioner_office::PractitionerOffice,
};
use axum::{
  debug_handler,
  extract::{Multipart, State},
  http::status,
  Json,
};
use image::{imageops::FilterType, ImageFormat};

#[debug_handler]
pub async fn save_business_info(
  State(_state): State<AppState>,
  CurrentUserExt(user, _): CurrentUserExt,
  Json(business_information): Json<CreateBusinessInfomation>,
) -> Result<Json<serde_json::Value>, MyErrors> {
  services::user::save_business_information(&business_information, &user).await?;

  Ok(Json(serde_json::json!({ "success": true })))
}

#[debug_handler]
pub async fn my_offices(
  State(state): State<AppState>,
  CurrentUserExt(user, _): CurrentUserExt,
) -> Result<Json<Vec<PractitionerOffice>>, MyErrors> {
  let my_offices = user.get_my_offices(&state.db).await?;

  let serialized_offices: Vec<PractitionerOffice> = my_offices
    .iter()
    .map(|office| PractitionerOffice::new(office))
    .collect();

  Ok(Json(serialized_offices))
}

#[debug_handler]
pub async fn upload_signature(
  State(_state): State<AppState>,
  CurrentUserExt(user, _): CurrentUserExt,
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
    &user.first_name.to_lowercase(),
    &user.last_name.to_lowercase(),
    &user.id.to_string()
  );

  let storage_service = services::storage::StorageService::new()?;
  storage_service
    .upload_signature(&png_bytes, &filename, "image/png")
    .await?;

  Ok(status::StatusCode::NO_CONTENT)
}
