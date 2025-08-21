use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
    create_table(
      m,
      "patients",
      &[
        ("id", ColType::PkAuto),
        ("name", ColType::String),
        ("ssn", ColType::StringUniq),
        ("pid", ColType::UuidUniq),
      ],
      &[],
    )
    .await
  }

  async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
    drop_table(m, "patients").await
  }
}
