use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(Iden)]
enum UserBusinessInformations {
  Table,
  Id,
  AdeliNumber,
  RppsNumber,
  SiretNumber,
  UserId,
}

#[derive(Iden)]
enum Users {
  Table,
  Id,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .create_table(
        Table::create()
          .table(UserBusinessInformations::Table)
          .if_not_exists()
          .col(
            ColumnDef::new(UserBusinessInformations::Id)
              .integer()
              .not_null()
              .auto_increment()
              .primary_key(),
          )
          .col(ColumnDef::new(UserBusinessInformations::AdeliNumber).string())
          .col(ColumnDef::new(UserBusinessInformations::RppsNumber).string().not_null())
          .col(ColumnDef::new(UserBusinessInformations::SiretNumber).string().not_null())
          .col(ColumnDef::new(UserBusinessInformations::UserId).integer().not_null())
          .foreign_key(
            ForeignKey::create()
              .name("fk-user_business_informations-user_id")
              .from(UserBusinessInformations::Table, UserBusinessInformations::UserId)
              .to(Users::Table, Users::Id)
              .on_delete(ForeignKeyAction::Cascade)
              .on_update(ForeignKeyAction::Cascade),
          )
          .to_owned(),
      )
      .await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .drop_table(Table::drop().table(UserBusinessInformations::Table).to_owned())
      .await
  }
}
