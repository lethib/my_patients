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
    if insert {
      this.pid = ActiveValue::Set(Uuid::new_v4());
    } else if this.updated_at.is_unchanged() {
      this.updated_at = ActiveValue::Set(chrono::Utc::now().into())
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
    params: &CreatePatientParams,
  ) -> ModelResult<Model, MyErrors> {
    let ssn_encrypted = Model::encrypt_ssn(&params.ssn)?;

    return Ok(
      patients::ActiveModel {
        name: ActiveValue::Set(params.name.clone()),
        ssn: ActiveValue::Set(ssn_encrypted),
        ..Default::default()
      }
      .insert(db)
      .await?,
    );
  }
}

// implement your custom finders, selectors oriented logic here
impl Entity {}
