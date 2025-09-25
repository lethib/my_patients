use loco_rs::schema::{
  add_column, add_reference, create_join_table, create_table, drop_table, remove_column,
  remove_reference, ColType,
};
use sea_orm_migration::{prelude::*, sea_orm::Statement};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
    create_table(
      m,
      "practitioner_offices",
      &[
        ("id", ColType::PkAuto),
        ("name", ColType::String),
        ("address_line_1", ColType::String),
        ("address_zip_code", ColType::String),
        ("address_city", ColType::String),
        ("address_country", ColType::String),
      ],
      &[],
    )
    .await?;
    create_join_table(
      m,
      "user_practitioner_offices",
      &[],
      &[("user", ""), ("practitioner_office", "")],
    )
    .await?;

    remove_column(m, "patients", "office").await?;
    // Drop the office enum type using raw SQL
    let drop_enum_sql = "DROP TYPE IF EXISTS office";
    let stmt = Statement::from_string(m.get_database_backend(), drop_enum_sql);
    m.get_connection().execute(stmt).await?;

    add_reference(m, "patient_user", "practitioner_offices", "").await?;

    Ok(())
  }

  async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
    remove_reference(m, "patient_user", "practitioner_offices", "").await?;
    add_column(m, "patient", "office", ColType::String).await?;
    drop_table(m, "user_practitioner_offices").await?;
    drop_table(m, "practitioner_offices").await?;
    Ok(())
  }
}
