use axum::{
  debug_handler,
  extract::{Path, Query, State},
  middleware,
  response::Response,
  routing::{get, post, put},
  Json,
};
use base64::Engine;
use loco_rs::{
  app::AppContext,
  prelude::{format, Routes},
};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
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
    _entities::{patient_users, patients},
    my_errors::{unexpected_error::UnexpectedError, MyErrors},
    patients::{CreatePatientParams, Model},
  },
  services::{self, invoice::GenerateInvoiceParams},
  views::patient::PatientResponse,
};

#[debug_handler]
async fn create(
  State(ctx): State<AppContext>,
  Json(create_patient_params): Json<CreatePatientParams>,
) -> Result<Response, MyErrors> {
  services::patients::create(&create_patient_params, &ctx.current_user().0).await?;

  Ok(format::json(serde_json::json!({ "success": true }))?)
}

#[debug_handler]
async fn update(
  State(ctx): State<AppContext>,
  Path(patient_id): Path<i32>,
  Json(patient_params): Json<CreatePatientParams>,
) -> Result<Response, MyErrors> {
  let patient = patients::Entity::find_by_id(patient_id)
    .inner_join(patient_users::Entity)
    .filter(patient_users::Column::UserId.eq(ctx.current_user().0.id))
    .one(&ctx.db)
    .await?
    .ok_or_else(|| UnexpectedError::SHOULD_NOT_HAPPEN.to_my_error())?;

  patients::ActiveModel::update(&ctx.db, patient.id, &patient_params).await?;

  // TODO_TM: let's not forget to delete and create association record

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
    .map(|p| PatientResponse::from_model(&p.0, &p.1))
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
    .add("/create", post(create))
    .add("/{patient_id}", put(update))
    .add("/_search_by_ssn", get(search_by_ssn))
    .add("/_search", get(search))
    .add("/{patient_id}/_generate_invoice", post(generate_invoice))
    .layer(middleware::from_fn_with_state(
      ctx.clone(),
      current_user_middleware,
    ))
}
