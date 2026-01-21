use axum::{
  debug_handler,
  extract::{Path, State},
  http::status,
  Json,
};
use chrono::DateTime;
use serde::Deserialize;

use crate::{
  app_state::{AppState, CurrentUserExt},
  models::{
    _entities::medical_appointments, medical_appointments::CreateMedicalAppointmentParams,
    my_errors::MyErrors,
  },
};

#[derive(Debug, Deserialize)]
pub struct CreateMedicalAppointmentPayload {
  date: String,
  practitioner_office_id: i32,
  price_in_cents: i32,
}

#[debug_handler]
pub async fn create(
  State(state): State<AppState>,
  CurrentUserExt(current_user, _): CurrentUserExt,
  Path(patient_id): Path<i32>,
  Json(params): Json<CreateMedicalAppointmentPayload>,
) -> Result<status::StatusCode, MyErrors> {
  let appointment_date = DateTime::parse_from_rfc3339(&params.date)?
    .naive_utc()
    .date();

  let medical_appointments_params = CreateMedicalAppointmentParams {
    date: appointment_date,
    practitioner_office_id: params.practitioner_office_id,
    price_in_cents: params.price_in_cents,
    user_id: current_user.id,
    patient_id: patient_id,
  };

  medical_appointments::ActiveModel::create(&state.db, &medical_appointments_params).await?;

  Ok(status::StatusCode::NO_CONTENT)
}
