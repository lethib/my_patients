use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(Iden)]
enum Users {
  Table,
  ApiKey,
  ResetToken,
  ResetSentAt,
  EmailVerificationToken,
  EmailVerificationSentAt,
  EmailVerifiedAt,
  MagicLinkToken,
  MagicLinkExpiration,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .alter_table(
        Table::alter()
          .table(Users::Table)
          .drop_column(Users::ApiKey)
          .drop_column(Users::ResetToken)
          .drop_column(Users::ResetSentAt)
          .drop_column(Users::EmailVerificationToken)
          .drop_column(Users::EmailVerificationSentAt)
          .drop_column(Users::EmailVerifiedAt)
          .drop_column(Users::MagicLinkToken)
          .drop_column(Users::MagicLinkExpiration)
          .to_owned(),
      )
      .await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .alter_table(
        Table::alter()
          .table(Users::Table)
          .add_column(ColumnDef::new(Users::ApiKey).string().not_null().unique_key())
          .add_column(ColumnDef::new(Users::ResetToken).string())
          .add_column(ColumnDef::new(Users::ResetSentAt).timestamp_with_time_zone())
          .add_column(ColumnDef::new(Users::EmailVerificationToken).string())
          .add_column(ColumnDef::new(Users::EmailVerificationSentAt).timestamp_with_time_zone())
          .add_column(ColumnDef::new(Users::EmailVerifiedAt).timestamp_with_time_zone())
          .add_column(ColumnDef::new(Users::MagicLinkToken).string())
          .add_column(ColumnDef::new(Users::MagicLinkExpiration).timestamp_with_time_zone())
          .to_owned(),
      )
      .await
  }
}
