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

#[derive(Deserialize)]
struct SearchParams {
  q: String,
  page: Option<u64>,
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
  services::patients::create(&create_patient_params, &ctx.current_user().0).await?;

  Ok(format::json(serde_json::json!({ "success": true }))?)
}

#[debug_handler]
async fn search_by_ssn(
  State(ctx): State<AppContext>,
  Query(params): Query<SearchBySSNParams>,
) -> Result<Response, MyErrors> {
  let found_patient = Model::search_by_ssn(&ctx.db, &params.ssn).await?;

  Ok(format::json(PatientResponse::new(&found_patient))?)
}

#[debug_handler]
async fn search(
  State(ctx): State<AppContext>,
  Query(params): Query<SearchParams>,
) -> Result<Response, MyErrors> {
  let page = params.page.unwrap_or(1);

  let query = if params.q.trim().is_empty() {
    ""
  } else {
    &params.q
  };

  let (patients, total_pages) =
    services::patients::search_paginated(query, page, &ctx.current_user().0).await?;

  let patient_responses: Vec<PatientResponse> =
    patients.iter().map(PatientResponse::from_model).collect();

  Ok(format::json(serde_json::json!({
    "paginated_data": patient_responses,
    "pagination": {
      "page": page,
      "per_page": 10,
      "total_pages": total_pages,
      "has_more": page < total_pages
    }
  }))?)
}

pub fn routes(ctx: &AppContext) -> Routes {
  Routes::new()
    .prefix("/api/patient")
    .add("/save", post(save))
    .add("/_search_by_ssn", get(search_by_ssn))
    .add("/_search", get(search))
    .layer(middleware::from_fn_with_state(
      ctx.clone(),
      current_user_middleware,
    ))
}
