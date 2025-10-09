use sea_orm::TransactionTrait;

use crate::{
  initializers::get_services,
  models::{
    _entities::{practitioner_offices, user_practitioner_offices},
    my_errors::MyErrors,
    practitioner_offices::PractitionerOfficeParams,
    user_practitioner_offices::CreateLinkParams,
    users::users,
  },
};

pub async fn create(
  params: &PractitionerOfficeParams,
  linked_practitioner: &users::Model,
) -> Result<(), MyErrors> {
  let services = get_services();

  let db_transaction = services.db.begin().await?;

  let created_practitioner_office =
    practitioner_offices::ActiveModel::create(&db_transaction, params).await?;

  user_practitioner_offices::ActiveModel::create(
    &db_transaction,
    &CreateLinkParams {
      user_id: linked_practitioner.id,
      practitioner_office_id: created_practitioner_office.id,
    },
  )
  .await?;

  db_transaction.commit().await?;

  Ok(())
}
