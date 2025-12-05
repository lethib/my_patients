use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(Iden)]
enum Patients {
  Table,
  AddressLine1,
  AddressZipCode,
  AddressCity,
  AddressCountry,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .alter_table(
        Table::alter()
          .table(Patients::Table)
          .add_column(ColumnDef::new(Patients::AddressLine1).string().not_null())
          .add_column(ColumnDef::new(Patients::AddressZipCode).string().not_null())
          .add_column(ColumnDef::new(Patients::AddressCity).string().not_null())
          .add_column(ColumnDef::new(Patients::AddressCountry).string().not_null())
          .to_owned(),
      )
      .await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .alter_table(
        Table::alter()
          .table(Patients::Table)
          .drop_column(Patients::AddressLine1)
          .drop_column(Patients::AddressZipCode)
          .drop_column(Patients::AddressCity)
          .drop_column(Patients::AddressCountry)
          .to_owned(),
      )
      .await
  }
}
