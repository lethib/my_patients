pub use super::_entities::practitioner_offices::{ActiveModel, Entity, Model};
use crate::{
  models::{
    _entities::practitioner_offices,
    my_errors::{application_error::ApplicationError, MyErrors, ToErr},
  },
  validators::address::is_address_valid,
};
use loco_rs::model::ModelResult;
use sea_orm::{entity::prelude::*, ActiveValue};
use serde::{Deserialize, Serialize};
pub type PractitionerOffices = Entity;

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
  async fn before_save<C>(self, _db: &C, insert: bool) -> std::result::Result<Self, DbErr>
  where
    C: ConnectionTrait,
  {
    if !insert && self.updated_at.is_unchanged() {
      let mut this = self;
      this.updated_at = sea_orm::ActiveValue::Set(chrono::Utc::now().into());
      Ok(this)
    } else {
      Ok(self)
    }
  }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PractitionerOfficeParams {
  pub name: String,
  pub address_line_1: String,
  pub address_zip_code: String,
  pub address_city: String,
}

// implement your read-oriented logic here
impl Model {}

// implement your write-oriented logic here
impl ActiveModel {
  pub async fn create<T: ConnectionTrait>(
    db: &T,
    params: &PractitionerOfficeParams,
  ) -> ModelResult<Model, MyErrors> {
    if !is_address_valid(&params.address_line_1, &params.address_zip_code) {
      return ApplicationError::UNPROCESSABLE_ENTITY.to_err();
    }

    return Ok(
      practitioner_offices::ActiveModel {
        name: ActiveValue::Set(params.name.clone()),
        address_line_1: ActiveValue::Set(params.address_line_1.clone()),
        address_zip_code: ActiveValue::Set(params.address_zip_code.clone()),
        address_city: ActiveValue::Set(params.address_city.clone()),
        address_country: ActiveValue::Set("FRANCE".to_string()),
        ..Default::default()
      }
      .insert(db)
      .await?,
    );
  }
}

// implement your custom finders, selectors oriented logic here
impl Entity {}
