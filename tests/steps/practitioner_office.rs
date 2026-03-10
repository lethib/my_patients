use axum::http::StatusCode;
use cucumber::{given, then, when};
use opencab::models::{
  _entities::{practitioner_offices, user_practitioner_offices},
  practitioner_offices::PractitionerOfficeParams,
  user_practitioner_offices::CreateLinkParams,
};
use sea_orm::ActiveValue::Set;
use sea_orm::{
  prelude::Decimal, ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter,
};
use std::str::FromStr;

use crate::{factories::user::UserFactory, AppWorld};

#[given("a practitioner exists for office tests")]
async fn practitioner_exists(world: &mut AppWorld) {
  world.practitioner_office.user = Some(UserFactory::new().create(&world.db).await);
}

#[given(expr = "an office {string} linked to the practitioner with a revenue share of {int}")]
async fn office_linked_to_practitioner(world: &mut AppWorld, name: String, revenue_share: i64) {
  let user = world.practitioner_office.user.as_ref().unwrap();
  let params = PractitionerOfficeParams {
    name: name.clone(),
    address_line_1: "1 rue de la Paix".to_string(),
    address_zip_code: "75001".to_string(),
    address_city: "Paris".to_string(),
  };
  let office = practitioner_offices::ActiveModel::create(&world.db, &params)
    .await
    .unwrap();

  let revenue_share_percentage = Decimal::from_str(&revenue_share.to_string()).unwrap();
  user_practitioner_offices::ActiveModel::create(
    &world.db,
    &CreateLinkParams {
      user_id: user.id,
      practitioner_office_id: office.id,
      revenue_share_percentage,
    },
  )
  .await
  .unwrap();

  world.practitioner_office.office = Some(office);
}

#[when(expr = "I create an office {string} with a revenue share of {int}")]
async fn create_office(world: &mut AppWorld, name: String, revenue_share: i64) {
  let user = world.practitioner_office.user.as_ref().unwrap();
  let params = PractitionerOfficeParams {
    name,
    address_line_1: "1 rue de la Paix".to_string(),
    address_zip_code: "75001".to_string(),
    address_city: "Paris".to_string(),
  };
  let office = practitioner_offices::ActiveModel::create(&world.db, &params)
    .await
    .unwrap();

  let revenue_share_percentage = Decimal::from_str(&revenue_share.to_string()).unwrap();
  user_practitioner_offices::ActiveModel::create(
    &world.db,
    &CreateLinkParams {
      user_id: user.id,
      practitioner_office_id: office.id,
      revenue_share_percentage,
    },
  )
  .await
  .unwrap();

  world.practitioner_office.office = Some(office);
}

#[when("I try to create an office with an invalid zip code")]
async fn try_create_office_invalid_zip(world: &mut AppWorld) {
  let params = PractitionerOfficeParams {
    name: "Cabinet Test".to_string(),
    address_line_1: "1 rue de la Paix".to_string(),
    address_zip_code: "INVALID".to_string(),
    address_city: "Paris".to_string(),
  };
  let result = practitioner_offices::ActiveModel::create(&world.db, &params).await;
  world.practitioner_office.last_error = result.err();
}

#[when(expr = "I update the office revenue share to {int}")]
async fn update_office_revenue_share(world: &mut AppWorld, revenue_share: i64) {
  let user = world.practitioner_office.user.as_ref().unwrap();
  let office = world.practitioner_office.office.as_ref().unwrap();

  let mut link = user_practitioner_offices::Entity::find()
    .filter(user_practitioner_offices::Column::PractitionerOfficeId.eq(office.id))
    .filter(user_practitioner_offices::Column::UserId.eq(user.id))
    .one(&world.db)
    .await
    .unwrap()
    .unwrap()
    .into_active_model();

  link.revenue_share_percentage = Set(Decimal::from_str(&revenue_share.to_string()).unwrap());
  link.update(&world.db).await.unwrap();
}

#[when(expr = "I update the office name to {string}")]
async fn update_office_name(world: &mut AppWorld, new_name: String) {
  let office = world.practitioner_office.office.as_ref().unwrap();
  let params = PractitionerOfficeParams {
    name: new_name,
    address_line_1: office.address_line_1.clone(),
    address_zip_code: office.address_zip_code.clone(),
    address_city: office.address_city.clone(),
  };
  let mut active = office.clone().into_active_model();
  active.name = Set(params.name.trim().to_string());
  let updated = active.update(&world.db).await.unwrap();
  world.practitioner_office.office = Some(updated);
}

#[then(expr = "the office is linked to the practitioner with a revenue share of {int}")]
async fn office_linked_with_revenue_share(world: &mut AppWorld, expected: i64) {
  let user = world.practitioner_office.user.as_ref().unwrap();
  let office = world.practitioner_office.office.as_ref().unwrap();

  let link = user_practitioner_offices::Entity::find()
    .filter(user_practitioner_offices::Column::PractitionerOfficeId.eq(office.id))
    .filter(user_practitioner_offices::Column::UserId.eq(user.id))
    .one(&world.db)
    .await
    .unwrap()
    .expect("link should exist");

  assert_eq!(
    link.revenue_share_percentage,
    Decimal::from_str(&expected.to_string()).unwrap()
  );
}

#[then(expr = "the office revenue share is {int}")]
async fn office_revenue_share_is(world: &mut AppWorld, expected: i64) {
  let user = world.practitioner_office.user.as_ref().unwrap();
  let office = world.practitioner_office.office.as_ref().unwrap();

  let link = user_practitioner_offices::Entity::find()
    .filter(user_practitioner_offices::Column::PractitionerOfficeId.eq(office.id))
    .filter(user_practitioner_offices::Column::UserId.eq(user.id))
    .one(&world.db)
    .await
    .unwrap()
    .expect("link should exist");

  assert_eq!(
    link.revenue_share_percentage,
    Decimal::from_str(&expected.to_string()).unwrap()
  );
}

#[then(expr = "the office name is {string}")]
fn office_name_is(world: &mut AppWorld, expected: String) {
  let office = world.practitioner_office.office.as_ref().unwrap();
  assert_eq!(office.name, expected);
}

#[then("the office creation fails")]
fn office_creation_fails(world: &mut AppWorld) {
  let err = world
    .practitioner_office
    .last_error
    .as_ref()
    .expect("expected an error but none was recorded");
  assert_eq!(
    err.code,
    StatusCode::UNPROCESSABLE_ENTITY,
    "expected UNPROCESSABLE_ENTITY, got: {:?}",
    err
  );
}
