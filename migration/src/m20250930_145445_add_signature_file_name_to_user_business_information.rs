use loco_rs::schema::{add_column, remove_column};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
    add_column(
      m,
      "user_business_informations",
      "signature_file_name",
      loco_rs::schema::ColType::StringWithDefault("temp.png".to_string()),
    )
    .await?;
    Ok(())
  }

  async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
    remove_column(m, "user_business_informations", "signature_file_name").await?;
    Ok(())
  }
}
