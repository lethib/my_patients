use axum::{
  debug_handler,
  extract::{Path, Query, State},
  middleware,
  response::Response,
  routing::{get, post},
  Json,
};
use base64::Engine;
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
  services::{self, invoice::GenerateInvoiceParams},
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

  Ok(format::json(PatientResponse::new(&found_patient, None))?)
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

  let patient_responses: Vec<PatientResponse> = patients
    .iter()
    .map(|p| PatientResponse::from_model(&p.0, &p.1.name))
    .collect();

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

#[debug_handler]
async fn generate_invoice(
  State(ctx): State<AppContext>,
  Path(patient_id): Path<i32>,
  Json(params): Json<GenerateInvoiceParams>,
) -> Result<Response, MyErrors> {
  tracing::info!("{:?}", params);
  let invoice_generated =
    services::invoice::generate_patient_invoice(patient_id, &params, &ctx.current_user().0).await?;

  Ok(format::json(serde_json::json!({
    "pdf_data": base64::prelude::BASE64_STANDARD.encode(&invoice_generated.pdf_data),
    "filename": invoice_generated.filename
  }))?)
}

pub fn routes(ctx: &AppContext) -> Routes {
  Routes::new()
    .prefix("/api/patient")
    .add("/save", post(save))
    .add("/_search_by_ssn", get(search_by_ssn))
    .add("/_search", get(search))
    .add("/{patient_id}/_generate_invoice", post(generate_invoice))
    .layer(middleware::from_fn_with_state(
      ctx.clone(),
      current_user_middleware,
    ))
}
