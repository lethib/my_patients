use crate::{
  models::{
    _entities::patients,
    my_errors::{
      application_error::ApplicationError, unexpected_error::UnexpectedError, MyErrors, ToErr,
    },
  },
  services::crypto::Crypto,
  validators::address::is_address_valid,
};

pub use super::_entities::patients::{ActiveModel, Entity, Model};
use loco_rs::model::ModelResult;
use sea_orm::{entity::prelude::*, ActiveValue};
use serde::{Deserialize, Serialize};
pub type Patients = Entity;

#[derive(Debug, Deserialize, Serialize)]
pub struct CreatePatientParams {
  first_name: String,
  last_name: String,
  pub ssn: String,
  address_line_1: String,
  address_zip_code: String,
  address_city: String,
}

// Encryption utilities for SSN
// implement your read-oriented logic here
impl Model {
  fn encrypt_ssn(ssn: &str) -> Result<String, MyErrors> {
    Crypto::encrypt(ssn)
  }

  fn hash_ssn(ssn: &str) -> Result<String, MyErrors> {
    // Explicitly load .env file
    dotenv::dotenv().ok();
    let salt = std::env::var("SSN_SALT_KEY")
      .map_err(|_| UnexpectedError::SHOULD_NOT_HAPPEN.to_my_error())?;
    Crypto::hash(ssn, &salt)
  }

  pub fn decrypt_ssn(&self) -> Result<String, MyErrors> {
    Crypto::decrypt(&self.ssn)
  }

  pub async fn search_by_ssn<C: ConnectionTrait>(
    db: &C,
    ssn: &str,
  ) -> Result<Option<Self>, MyErrors> {
    let hashed_ssn = Self::hash_ssn(ssn)?;

    let patient = Entity::find()
      .filter(patients::Column::HashedSsn.eq(hashed_ssn))
      .one(db)
      .await
      .map_err(MyErrors::from)?;

    Ok(patient)
  }
}

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
  async fn before_save<C>(self, _db: &C, insert: bool) -> std::result::Result<Self, DbErr>
  where
    C: ConnectionTrait,
  {
    let mut this = self;
    if insert {
      this.pid = ActiveValue::Set(Uuid::new_v4());
    } else if this.updated_at.is_unchanged() {
      this.updated_at = ActiveValue::Set(chrono::Utc::now().into())
    }
    Ok(this)
  }
}

// implement your write-oriented logic here
impl ActiveModel {
  pub async fn create<T: ConnectionTrait>(
    db: &T,
    params: &CreatePatientParams,
  ) -> ModelResult<Model, MyErrors> {
    if !is_address_valid(&params.address_line_1, &params.address_zip_code) {
      return ApplicationError::UNPROCESSABLE_ENTITY.to_err();
    }

    let ssn_encrypted = Model::encrypt_ssn(&params.ssn)?;
    let ssn_hashed = Model::hash_ssn(&params.ssn)?;

    return Ok(
      patients::ActiveModel {
        first_name: ActiveValue::Set(params.first_name.clone()),
        last_name: ActiveValue::Set(params.last_name.clone()),
        ssn: ActiveValue::Set(ssn_encrypted),
        hashed_ssn: ActiveValue::Set(ssn_hashed),
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
