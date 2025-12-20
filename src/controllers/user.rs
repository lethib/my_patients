use crate::{
  app_state::{AppState, CurrentUserExt},
  models::{my_errors::MyErrors, user_business_informations::CreateBusinessInfomation},
  services,
  views::practitioner_office::PractitionerOffice,
};
use axum::{
  debug_handler,
  extract::{Multipart, State},
  Json,
};

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
) -> Result<Json<serde_json::Value>, MyErrors> {
  let mut signature_data: Option<Vec<u8>> = None;
  let mut filename: Option<String> = None;
  let mut content_type: Option<String> = None;

  while let Some(field) = multipart.next_field().await.map_err(|_| MyErrors {
    code: axum::http::StatusCode::BAD_REQUEST,
    msg: "Invalid multipart data".to_string(),
  })? {
    let field_name = field.name().unwrap_or("").to_string();

    if field_name == "signature" {
      filename = field.file_name().map(|f| f.to_string());
      content_type = field.content_type().map(|ct| ct.to_string());

      tracing::info!("filename={:?}, content_type={:?}", filename, content_type);

      let data = field.bytes().await.map_err(|_| MyErrors {
        code: axum::http::StatusCode::BAD_REQUEST,
        msg: "Failed to read file data".to_string(),
      })?;

      signature_data = Some(data.to_vec());
    }
  }

  let signature_data = signature_data.ok_or_else(|| MyErrors {
    code: axum::http::StatusCode::BAD_REQUEST,
    msg: "No signature file provided".to_string(),
  })?;

  let filename = filename.unwrap_or_else(|| format!("signature_{}.png", user.id));
  let content_type = content_type.unwrap_or_else(|| "image/png".to_string());

  services::invoice::upload_signature_for_user(&user, &signature_data, &filename, &content_type)
    .await?;

  Ok(Json(serde_json::json!({
    "success": true,
    "message": "Signature uploaded successfully",
    "filename": filename
  })))
}
