use loco_rs::schema::{add_column, remove_column, ColType};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
    remove_column(m, "users", "name").await?;
    add_column(m, "users", "first_name", ColType::String).await?;
    add_column(m, "users", "last_name", ColType::String).await?;
    Ok(())
  }

  async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
    add_column(m, "users", "name", ColType::String).await?;
    remove_column(m, "users", "first_name").await?;
    remove_column(m, "users", "last_name").await?;
    Ok(())
  }
}
