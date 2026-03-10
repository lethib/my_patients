use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .alter_table(
        Table::alter()
          .table(UserPractitionerOffices::Table)
          .add_column(
            ColumnDef::new(UserPractitionerOffices::RevenueSharePercentage)
              .decimal_len(5, 2)
              .not_null()
              .default(0.00),
          )
          .to_owned(),
      )
      .await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .alter_table(
        Table::alter()
          .table(UserPractitionerOffices::Table)
          .drop_column(UserPractitionerOffices::RevenueSharePercentage)
          .to_owned(),
      )
      .await
  }
}

#[derive(Iden)]
enum UserPractitionerOffices {
  Table,
  RevenueSharePercentage,
}
