use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(Iden)]
enum Users {
  Table,
  Name,
  FirstName,
  LastName,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .alter_table(
        Table::alter()
          .table(Users::Table)
          .drop_column(Users::Name)
          .add_column(ColumnDef::new(Users::FirstName).string().not_null())
          .add_column(ColumnDef::new(Users::LastName).string().not_null())
          .to_owned(),
      )
      .await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .alter_table(
        Table::alter()
          .table(Users::Table)
          .add_column(ColumnDef::new(Users::Name).string().not_null())
          .drop_column(Users::FirstName)
          .drop_column(Users::LastName)
          .to_owned(),
      )
      .await
  }
}
