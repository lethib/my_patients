use crate::models::my_errors::MyErrors;

pub use super::_entities::patient_users::{ActiveModel, Entity, Model};
use loco_rs::model::ModelResult;
use sea_orm::{entity::prelude::*, ActiveValue};
pub type PatientUsers = Entity;

pub struct CreateLinkParams {
  pub user_id: i32,
  pub patient_id: i32,
  pub practitioner_office_id: i32,
}

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

// implement your read-oriented logic here
impl Model {}

// implement your write-oriented logic here
impl ActiveModel {
  pub async fn create<T: ConnectionTrait>(
    db: &T,
    params: &CreateLinkParams,
  ) -> ModelResult<Model, MyErrors> {
    return Ok(
      self::ActiveModel {
        user_id: ActiveValue::Set(params.user_id),
        patient_id: ActiveValue::Set(params.patient_id),
        practitioner_office_id: ActiveValue::Set(params.practitioner_office_id),
        ..Default::default()
      }
      .insert(db)
      .await?,
    );
  }
}

// implement your custom finders, selectors oriented logic here
impl Entity {}
