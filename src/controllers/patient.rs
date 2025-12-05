use axum::{
  debug_handler,
  extract::{Path, Query, State},
  Json,
};
use base64::Engine;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SearchBySSNParams {
  pub ssn: String,
}

#[derive(Deserialize)]
pub struct SearchParams {
  pub q: String,
  pub page: Option<u64>,
}

use crate::{
  app_state::{AppState, CurrentUserExt},
  models::{
    _entities::patients,
    my_errors::{application_error::ApplicationError, MyErrors},
    patients::{CreatePatientParams, Model},
  },
  services::{self, invoice::GenerateInvoiceParams},
  views::patient::PatientResponse,
};

#[debug_handler]
pub async fn create(
  State(_state): State<AppState>,
  CurrentUserExt(user, _): CurrentUserExt,
  Json(create_patient_params): Json<CreatePatientParams>,
) -> Result<Json<serde_json::Value>, MyErrors> {
  services::patients::create(&create_patient_params, &user).await?;

  Ok(Json(serde_json::json!({ "success": true })))
}

#[debug_handler]
pub async fn update(
  State(state): State<AppState>,
  CurrentUserExt(user, _): CurrentUserExt,
  Path(patient_id): Path<i32>,
  Json(patient_params): Json<CreatePatientParams>,
) -> Result<Json<serde_json::Value>, MyErrors> {
  let patient = patients::Entity::find_by_id(patient_id)
    .filter(patients::Column::UserId.eq(user.id))
    .one(&state.db)
    .await?
    .ok_or(ApplicationError::NOT_FOUND())?;

  services::patients::update(&patient, &patient_params).await?;

  Ok(Json(serde_json::json!({ "success": true })))
}

#[debug_handler]
pub async fn search_by_ssn(
  State(state): State<AppState>,
  Query(params): Query<SearchBySSNParams>,
) -> Result<Json<Vec<PatientResponse>>, MyErrors> {
  let found_patients = Model::search_by_ssn(&state.db, &params.ssn).await?;

  let serialized_patients: Vec<PatientResponse> = found_patients
    .iter()
    .map(|patient| PatientResponse::new(patient))
    .collect();

  Ok(Json(serialized_patients))
}

#[debug_handler]
pub async fn search(
  State(_state): State<AppState>,
  CurrentUserExt(user, _): CurrentUserExt,
  Query(params): Query<SearchParams>,
) -> Result<Json<serde_json::Value>, MyErrors> {
  let page = params.page.unwrap_or(1);

  let query = if params.q.trim().is_empty() {
    ""
  } else {
    &params.q
  };

  let (patients, total_pages) = services::patients::search_paginated(query, page, &user).await?;

  let patient_responses: Vec<PatientResponse> = patients
    .iter()
    .map(|p| PatientResponse::from_model(&p))
    .collect();

  Ok(Json(serde_json::json!({
    "paginated_data": patient_responses,
    "pagination": {
      "page": page,
      "per_page": 10,
      "total_pages": total_pages,
      "has_more": page < total_pages
    }
  })))
}

#[debug_handler]
pub async fn generate_invoice(
  State(state): State<AppState>,
  CurrentUserExt(user, _): CurrentUserExt,
  Path(patient_id): Path<i32>,
  Json(params): Json<GenerateInvoiceParams>,
) -> Result<Json<serde_json::Value>, MyErrors> {
  let invoice_generated =
    services::invoice::generate_patient_invoice(&patient_id, &params, &user).await?;

  if params.should_be_sent_by_email {
    services::invoice::send_invoice(&state, &invoice_generated, &user).await?;
  }

  Ok(Json(serde_json::json!({
    "pdf_data": base64::prelude::BASE64_STANDARD.encode(&invoice_generated.pdf_data),
    "filename": invoice_generated.filename
  })))
}
