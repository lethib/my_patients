use crate::{
  app_state::{AppState, CurrentUserExt},
  models::{my_errors::MyErrors, user_business_informations::CreateBusinessInfomation},
  services,
  views::practitioner_office::PractitionerOffice,
};
use axum::{debug_handler, extract::State, Json};

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
