use super::_entities::user_business_informations::{ActiveModel, Entity, Model};
use crate::models::user_business_informations;
use crate::validators::business_information::{validate_rpps_number, validate_siret_number};
use loco_rs::model::ModelResult;
use sea_orm::{entity::prelude::*, ActiveValue};
use serde::Deserialize;

pub type UserBusinessInformations = Entity;

#[derive(Deserialize)]
pub struct CreateBusinessInfomation {
  pub rpps_number: String,
  pub adeli_number: Option<String>,
  pub siret_number: String,
}

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
  async fn before_save<C>(self, _db: &C, insert: bool) -> std::result::Result<Self, DbErr>
  where
    C: ConnectionTrait,
  {
    let mut this = self;

    // if this.rpps_number matches an ActiveValue::Set pattern (i.e when the values changes),
    // it extracts the inner value and bind it to a variable we call rpps
    if let ActiveValue::Set(ref rpps) = this.rpps_number {
      if !validate_rpps_number(rpps) {
        return Err(DbErr::Custom("RPPS_number_not_valid".to_string()));
      }
    }

    if let ActiveValue::Set(ref siret) = this.siret_number {
      if !validate_siret_number(siret) {
        return Err(DbErr::Custom("SIRET_number_not_valid".to_string()));
      }
    }

    if !insert && this.updated_at.is_unchanged() {
      this.updated_at = ActiveValue::Set(chrono::Utc::now().into());
    }

    Ok(this)
  }
}

// implement your read-oriented logic here
impl Model {}

// implement your write-oriented logic here
impl ActiveModel {
  pub async fn create<T: ConnectionTrait>(
    db: &T,
    params: &CreateBusinessInfomation,
    concerned_user_id: &i32,
  ) -> ModelResult<Model, DbErr> {
    Ok(
      user_business_informations::ActiveModel {
        user_id: ActiveValue::Set(*concerned_user_id),
        rpps_number: ActiveValue::Set(params.rpps_number.clone()),
        siret_number: ActiveValue::Set(params.siret_number.clone()),
        adeli_number: ActiveValue::Set(params.adeli_number.clone()),
        ..Default::default()
      }
      .insert(db)
      .await?,
    )
  }
}

// implement your custom finders, selectors oriented logic here
impl Entity {}
