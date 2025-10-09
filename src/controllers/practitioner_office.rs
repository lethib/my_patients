use axum::{debug_handler, extract::State, middleware, response::Response, routing::post, Json};
use loco_rs::{
  app::AppContext,
  prelude::{format, Routes},
};

use crate::{
  middlewares::current_user::{current_user_middleware, CurrentUser},
  models::{my_errors::MyErrors, practitioner_offices::PractitionerOfficeParams},
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

pub fn routes(ctx: &AppContext) -> Routes {
  Routes::new()
    .prefix("/api/practitioner_office")
    .add("/create", post(create))
    .layer(middleware::from_fn_with_state(
      ctx.clone(),
      current_user_middleware,
    ))
}
