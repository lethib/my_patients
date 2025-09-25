use crate::initializers::get_services;
use crate::models::_entities::{patient_users, patients, practitioner_offices};
use crate::models::my_errors::unexpected_error::UnexpectedError;
use crate::models::patient_users::CreateLinkParams;
use crate::models::{
  my_errors::MyErrors,
  patients::{CreatePatientParams, Model as PatientModel},
  users,
};
use sea_orm::{ColumnTrait, Condition, QueryFilter, QuerySelect, RelationTrait};
use sea_orm::{EntityTrait, JoinType, PaginatorTrait, QueryOrder, TransactionTrait};

pub async fn create(
  patient_params: &CreatePatientParams,
  linked_to_user: &users::Model,
) -> Result<PatientModel, MyErrors> {
  let services = get_services();

  let existing_patient = PatientModel::search_by_ssn(&services.db, &patient_params.ssn).await?;

  let db_transaction = services.db.begin().await?;

  let created_patient = match existing_patient {
    Some(patient) => patient,
    None => patients::ActiveModel::create(&db_transaction, patient_params).await?,
  };

  patient_users::ActiveModel::create(
    &db_transaction,
    &CreateLinkParams {
      user_id: linked_to_user.id,
      patient_id: created_patient.id,
      practitioner_office_id: patient_params.practitioner_office_id,
    },
  )
  .await?;

  db_transaction.commit().await?;

  Ok(created_patient)
}

pub async fn search_paginated(
  query: &str,
  page: u64,
  user: &users::Model,
) -> Result<(Vec<(PatientModel, practitioner_offices::Model)>, u64), MyErrors> {
  let db = &get_services().db;

  // Build search condition for first_name and last_name (case-insensitive)
  let search_condition = Condition::any()
    .add(sea_orm::sea_query::Expr::cust_with_values(
      "LOWER(first_name) LIKE LOWER($1)",
      [format!("%{}%", query)],
    ))
    .add(sea_orm::sea_query::Expr::cust_with_values(
      "LOWER(last_name) LIKE LOWER($1)",
      [format!("%{}%", query)],
    ));

  // Query patients that belong to the current user and match the search
  let paginator = patients::Entity::find()
    .inner_join(patient_users::Entity)
    .join(
      JoinType::InnerJoin,
      patient_users::Relation::PractitionerOffices.def(),
    )
    .filter(patient_users::Column::UserId.eq(user.id))
    .filter(search_condition)
    .select_also(practitioner_offices::Entity)
    .order_by_asc(patients::Column::FirstName)
    .order_by_asc(patients::Column::LastName)
    .paginate(db, 10);

  let total_pages = paginator.num_pages().await?;
  let patients_with_optional_offices = paginator.fetch_page(page - 1).await?; // SeaORM uses 0-based pagination

  let result: Result<Vec<(PatientModel, practitioner_offices::Model)>, MyErrors> =
    patients_with_optional_offices
      .into_iter()
      .map(|(patient, office_option)| {
        office_option.map(|office| (patient, office)).ok_or(
          UnexpectedError::new("Patient should always have a practitioner office".to_string())
            .to_my_error(),
        )
      })
      .collect();

  Ok((result?, total_pages))
}
