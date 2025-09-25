use crate::{
  middlewares::current_user::{current_user_middleware, CurrentUser},
  models::{my_errors::MyErrors, user_business_informations::CreateBusinessInfomation},
  services,
  views::practitioner_office::PractitionerOffice,
};
use axum::{
  debug_handler,
  extract::State,
  middleware,
  response::Response,
  routing::{get, post},
  Json,
};
use loco_rs::{
  app::AppContext,
  prelude::{format, Routes},
};

#[debug_handler]
async fn save_business_information(
  State(ctx): State<AppContext>,
  Json(business_information): Json<CreateBusinessInfomation>,
) -> Result<Response, MyErrors> {
  services::user::save_business_information(&business_information, &ctx.current_user().0).await?;

  Ok(format::json(serde_json::json!({ "success": true }))?)
}

#[debug_handler]
async fn my_offices(State(ctx): State<AppContext>) -> Result<Response, MyErrors> {
  let my_offices = ctx.current_user().0.get_my_offices(&ctx.db).await?;

  let serialized_offices: Vec<PractitionerOffice> = my_offices
    .iter()
    .map(|office| PractitionerOffice::new(office))
    .collect();

  Ok(format::json(serialized_offices)?)
}

pub fn routes(ctx: &AppContext) -> Routes {
  Routes::new()
    .prefix("/api/user")
    .add(
      "/_save_business_information",
      post(save_business_information),
    )
    .add("/my_offices", get(my_offices))
    .layer(middleware::from_fn_with_state(
      ctx.clone(),
      current_user_middleware,
    ))
}
