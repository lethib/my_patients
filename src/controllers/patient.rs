use axum::{debug_handler, extract::State, middleware, response::Response, routing::post, Json};
use loco_rs::{
  app::AppContext,
  prelude::{format, Routes},
};

use crate::{
  middlewares::current_user::{current_user_middleware, CurrentUser},
  models::{my_errors::MyErrors, patients::CreatePatientParams},
  services,
};

#[debug_handler]
async fn save(
  State(ctx): State<AppContext>,
  Json(create_patient_params): Json<CreatePatientParams>,
) -> Result<Response, MyErrors> {
  services::patients::create(&create_patient_params, &ctx.current_user()).await?;

  Ok(format::json(serde_json::json!({ "success": true }))?)
}

pub fn routes(ctx: &AppContext) -> Routes {
  Routes::new()
    .prefix("/api/patient")
    .add("/save", post(save))
    .layer(middleware::from_fn_with_state(
      ctx.clone(),
      current_user_middleware,
    ))
}
