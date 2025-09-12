use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
    add_column(m, "patients", "address_line_1", ColType::String).await?;
    add_column(m, "patients", "address_zip_code", ColType::String).await?;
    add_column(m, "patients", "address_city", ColType::String).await?;
    add_column(m, "patients", "address_country", ColType::String).await?;
    Ok(())
  }

  async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
    remove_column(m, "patients", "address_line_1").await?;
    remove_column(m, "patients", "address_zip_code").await?;
    remove_column(m, "patients", "address_city").await?;
    remove_column(m, "patients", "address_country").await?;
    Ok(())
  }
}
