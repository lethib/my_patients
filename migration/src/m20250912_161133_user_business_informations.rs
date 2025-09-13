use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
    create_table(
      m,
      "user_business_informations",
      &[
        ("id", ColType::PkAuto),
        ("adeli_number", ColType::StringNull),
        ("rpps_number", ColType::String),
        ("siret_number", ColType::String),
      ],
      &[("user", "")],
    )
    .await
  }

  async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
    drop_table(m, "user_business_informations").await
  }
}
