use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(Iden)]
enum UserBusinessInformations {
  Table,
  SignatureFileName,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .alter_table(
        Table::alter()
          .table(UserBusinessInformations::Table)
          .modify_column(
            ColumnDef::new(UserBusinessInformations::SignatureFileName)
              .string()
              .null()
              .default(Option::<String>::None),
          )
          .to_owned(),
      )
      .await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .alter_table(
        Table::alter()
          .table(UserBusinessInformations::Table)
          .modify_column(
            ColumnDef::new(UserBusinessInformations::SignatureFileName)
              .string()
              .not_null()
              .default("temp.png"),
          )
          .to_owned(),
      )
      .await
  }
}
