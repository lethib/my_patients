use crate::{
  models::{_entities::patients, my_errors::MyErrors},
  services::crypto::Crypto,
};

pub use super::_entities::patients::{ActiveModel, Entity, Model};
use loco_rs::model::ModelResult;
use sea_orm::{entity::prelude::*, ActiveValue};
use serde::{Deserialize, Serialize};
pub type Patients = Entity;

#[derive(Debug, Deserialize, Serialize)]
pub struct CreatePatientParams {
  name: String,
  ssn: String,
}

// Encryption utilities for SSN
impl Model {
  fn encrypt_ssn(ssn: &str) -> Result<String, MyErrors> {
    Crypto::encrypt(ssn)
  }

  pub fn decrypt_ssn(&self) -> Result<String, MyErrors> {
    Crypto::decrypt(&self.ssn)
  }
}

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
  async fn before_save<C>(self, _db: &C, insert: bool) -> std::result::Result<Self, DbErr>
  where
    C: ConnectionTrait,
  {
    let mut this = self;
    if !insert && this.updated_at.is_unchanged() {
      this.updated_at = sea_orm::ActiveValue::Set(chrono::Utc::now().into());
      Ok(this)
    } else {
      this.pid = ActiveValue::Set(Uuid::new_v4());
      Ok(this)
    }
  }
}

// implement your read-oriented logic here
impl Model {}

// implement your write-oriented logic here
impl ActiveModel {
  pub async fn create(
    db: &DatabaseConnection,
    params: &CreatePatientParams,
  ) -> ModelResult<Model, MyErrors> {
    let ssn_encrypted = Model::encrypt_ssn(&params.ssn)?;

    let patient = patients::ActiveModel {
      name: ActiveValue::Set(params.name.clone()),
      ssn: ActiveValue::Set(ssn_encrypted),
      ..Default::default()
    }
    .insert(db)
    .await?;

    return Ok(patient);
  }
}

// implement your custom finders, selectors oriented logic here
impl Entity {}
