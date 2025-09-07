use axum::{
  debug_handler,
  extract::{Query, State},
  middleware,
  response::Response,
  routing::{get, post},
  Json,
};
use loco_rs::{
  app::AppContext,
  prelude::{format, Routes},
};
use serde::Deserialize;

#[derive(Deserialize)]
struct SearchBySSNParams {
  ssn: String,
}

use crate::{
  middlewares::current_user::{current_user_middleware, CurrentUser},
  models::{
    my_errors::MyErrors,
    patients::{CreatePatientParams, Model},
  },
  services,
  views::patient::PatientResponse,
};

#[debug_handler]
async fn save(
  State(ctx): State<AppContext>,
  Json(create_patient_params): Json<CreatePatientParams>,
) -> Result<Response, MyErrors> {
  services::patients::create(&create_patient_params, &ctx.current_user()).await?;

  Ok(format::json(serde_json::json!({ "success": true }))?)
}

#[debug_handler]
async fn search_by_ssn(
  State(ctx): State<AppContext>,
  Query(params): Query<SearchBySSNParams>,
) -> Result<Response, MyErrors> {
  tracing::info!(params.ssn);
  let found_user = Model::search_by_ssn(&ctx.db, &params.ssn).await?;

  Ok(format::json(PatientResponse::new(&found_user))?)
}

pub fn routes(ctx: &AppContext) -> Routes {
  Routes::new()
    .prefix("/api/patient")
    .add("/save", post(save))
    .add("/_search_by_ssn", get(search_by_ssn))
    .layer(middleware::from_fn_with_state(
      ctx.clone(),
      current_user_middleware,
    ))
}
