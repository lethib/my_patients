use axum::{
  debug_handler,
  extract::{Path, State},
  http::status,
  Json,
};
use chrono::NaiveDate;
use sea_orm::{ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter};
use serde::Deserialize;

use crate::{
  app_state::AppState,
  auth::statement::AuthStatement,
  middleware::auth::AuthenticatedUser,
  models::{
    _entities::medical_appointments,
    medical_appointments::{CreateMedicalAppointmentParams, UpdateMedicalAppointmentParams},
    my_errors::{application_error::ApplicationError, MyErrors},
  },
};

#[derive(Debug, Deserialize)]
pub struct MedicalAppointmentPayload {
  date: String,
  practitioner_office_id: i32,
  price_in_cents: i32,
}

#[debug_handler]
pub async fn update(
  State(state): State<AppState>,
  authorize: AuthStatement,
  Path((patient_id, appointment_id)): Path<(i32, i32)>,
  Json(params): Json<MedicalAppointmentPayload>,
) -> Result<status::StatusCode, MyErrors> {
  let medical_appointment = medical_appointments::Entity::find_by_id(appointment_id)
    .filter(medical_appointments::Column::PatientId.eq(patient_id))
    .one(&state.db)
    .await?
    .ok_or(ApplicationError::NOT_FOUND())?;

  authorize
    .is_owning_resource(&medical_appointment)
    .await
    .run_complete()?;

  // Parse date string in YYYY-MM-DD format
  let appointment_date = NaiveDate::parse_from_str(&params.date, "%Y-%m-%d")?;

  let medical_appointments_params = UpdateMedicalAppointmentParams {
    date: appointment_date,
    practitioner_office_id: params.practitioner_office_id,
    price_in_cents: params.price_in_cents,
  };

  medical_appointment
    .into_active_model()
    .update(&state.db, &medical_appointments_params)
    .await?;

  Ok(status::StatusCode::NO_CONTENT)
}

#[debug_handler]
pub async fn create(
  State(state): State<AppState>,
  authorize: AuthStatement,
  AuthenticatedUser(current_user, _): AuthenticatedUser,
  Path(patient_id): Path<i32>,
  Json(params): Json<MedicalAppointmentPayload>,
) -> Result<status::StatusCode, MyErrors> {
  authorize.authenticated_user().run_complete()?;

  // Parse date string in YYYY-MM-DD format
  let appointment_date = NaiveDate::parse_from_str(&params.date, "%Y-%m-%d")?;

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
