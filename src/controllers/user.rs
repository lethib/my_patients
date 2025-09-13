use crate::{
  middlewares::current_user::{current_user_middleware, CurrentUser},
  models::{my_errors::MyErrors, user_business_informations::CreateBusinessInfomation},
  services,
};
use axum::{debug_handler, extract::State, middleware, response::Response, routing::post, Json};
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

pub fn routes(ctx: &AppContext) -> Routes {
  Routes::new()
    .prefix("/api/user")
    .add(
      "/_save_business_information",
      post(save_business_information),
    )
    .layer(middleware::from_fn_with_state(
      ctx.clone(),
      current_user_middleware,
    ))
}
