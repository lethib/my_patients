use async_trait::async_trait;
use loco_rs::{auth::jwt, hash, prelude::*};
use serde::{Deserialize, Serialize};
use serde_json::Map;
use uuid::Uuid;

use crate::models::{
  _entities::{
    prelude::UserBusinessInformations, user_business_informations, user_practitioner_offices,
  },
  practitioner_offices,
};

pub use super::_entities::users::{self, ActiveModel, Entity, Model};

pub const MAGIC_LINK_LENGTH: i8 = 32;
pub const MAGIC_LINK_EXPIRATION_MIN: i8 = 5;

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginParams {
  pub email: String,
  pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RegisterParams {
  pub email: String,
  pub password: String,
  pub first_name: String,
  pub last_name: String,
  pub phone_number: String,
}

#[derive(Debug, Validate, Deserialize)]
pub struct Validator {
  #[validate(email(message = "invalid email"))]
  pub email: String,
}

impl Validatable for ActiveModel {
  fn validator(&self) -> Box<dyn Validate> {
    Box::new(Validator {
      email: self.email.as_ref().to_owned(),
    })
  }
}

#[async_trait::async_trait]
impl ActiveModelBehavior for super::_entities::users::ActiveModel {
  async fn before_save<C>(self, _db: &C, insert: bool) -> Result<Self, DbErr>
  where
    C: ConnectionTrait,
  {
    self.validate()?;
    if insert {
      let mut this = self;
      this.pid = ActiveValue::Set(Uuid::new_v4());
      Ok(this)
    } else {
      Ok(self)
    }
  }
}

#[async_trait]
impl Authenticable for Model {
  async fn find_by_api_key(_db: &DatabaseConnection, _api_key: &str) -> ModelResult<Self> {
    return Err(ModelError::Message(("method not implemented").to_string()));
  }

  async fn find_by_claims_key(db: &DatabaseConnection, claims_key: &str) -> ModelResult<Self> {
    Self::find_by_pid(db, claims_key)
      .await
      .map(|(user, _)| user)
  }
}

impl Model {
  pub fn full_name(&self) -> String {
    format!("{} {}", &self.first_name, &self.last_name)
  }

  pub async fn get_my_offices(
    &self,
    db: &DatabaseConnection,
  ) -> ModelResult<Vec<practitioner_offices::Model>> {
    let offices = practitioner_offices::Entity::find()
      .inner_join(user_practitioner_offices::Entity)
      .filter(user_practitioner_offices::Column::UserId.eq(self.id))
      .all(db)
      .await?;

    Ok(offices)
  }

  /// finds a user by the provided email
  ///
  /// # Errors
  ///
  /// When could not find user by the given token or DB query error
  pub async fn find_by_email(db: &DatabaseConnection, email: &str) -> ModelResult<Self> {
    let user = users::Entity::find()
      .filter(
        model::query::condition()
          .eq(users::Column::Email, email)
          .build(),
      )
      .one(db)
      .await?;
    user.ok_or_else(|| ModelError::EntityNotFound)
  }

  /// finds a user by the provided pid
  ///
  /// # Errors
  ///
  /// When could not find user  or DB query error
  pub async fn find_by_pid(
    db: &DatabaseConnection,
    pid: &str,
  ) -> ModelResult<(Self, Option<user_business_informations::Model>)> {
    let parse_uuid = Uuid::parse_str(pid).map_err(|e| ModelError::Any(e.into()))?;
    let user = users::Entity::find()
      .filter(
        model::query::condition()
          .eq(users::Column::Pid, parse_uuid)
          .build(),
      )
      .find_also_related(UserBusinessInformations)
      .one(db)
      .await?;
    user.ok_or_else(|| ModelError::EntityNotFound)
  }

  /// Verifies whether the provided plain password matches the hashed password
  ///
  /// # Errors
  ///
  /// when could not verify password
  #[must_use]
  pub fn verify_password(&self, password: &str) -> bool {
    hash::verify_password(password, &self.password)
  }

  /// Asynchronously creates a user with a password and saves it to the
  /// database.
  ///
  /// # Errors
  ///
  /// When could not save the user into the DB
  pub async fn create_with_password(
    db: &DatabaseConnection,
    params: &RegisterParams,
  ) -> ModelResult<Self> {
    let txn = db.begin().await?;

    if users::Entity::find()
      .filter(
        model::query::condition()
          .eq(users::Column::Email, &params.email)
          .build(),
      )
      .one(&txn)
      .await?
      .is_some()
    {
      return Err(ModelError::EntityAlreadyExists {});
    }

    let password_hash =
      hash::hash_password(&params.password).map_err(|e| ModelError::Any(e.into()))?;
    let user = users::ActiveModel {
      email: ActiveValue::set(params.email.to_string()),
      password: ActiveValue::set(password_hash),
      first_name: ActiveValue::set(params.first_name.clone()),
      last_name: ActiveValue::set(params.last_name.clone()),
      phone_number: ActiveValue::set(params.phone_number.clone()),
      ..Default::default()
    }
    .insert(&txn)
    .await?;

    txn.commit().await?;

    Ok(user)
  }

  /// Creates a JWT
  ///
  /// # Errors
  ///
  /// when could not convert user claims to jwt token
  pub fn generate_jwt(&self, secret: &str, expiration: u64) -> ModelResult<String> {
    jwt::JWT::new(secret)
      .generate_token(expiration, self.pid.to_string(), Map::new())
      .map_err(ModelError::from)
  }
}

impl ActiveModel {}
