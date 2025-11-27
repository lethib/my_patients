use sea_orm_migration::{prelude::*, schema::*, sea_orm::Statement};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .create_table(
        Table::create()
          .table(MedicalAppointments::Table)
          .if_not_exists()
          .col(pk_auto(MedicalAppointments::Id))
          .col(integer_null(MedicalAppointments::UserId))
          .col(integer_null(MedicalAppointments::PatientId))
          .col(integer_null(MedicalAppointments::PractitionerOfficeId))
          .col(date_null(MedicalAppointments::Date))
          .col(timestamp_with_time_zone(MedicalAppointments::CreatedAt))
          .col(timestamp_with_time_zone(MedicalAppointments::UpdatedAt))
          .foreign_key(
            ForeignKey::create()
              .name("fk_medical_appointments_user_id")
              .from(MedicalAppointments::Table, MedicalAppointments::UserId)
              .to(Users::Table, Users::Id)
              .on_delete(ForeignKeyAction::Cascade)
              .on_update(ForeignKeyAction::Cascade),
          )
          .foreign_key(
            ForeignKey::create()
              .name("fk_medical_appointments_patient_id")
              .from(MedicalAppointments::Table, MedicalAppointments::PatientId)
              .to(Patients::Table, Patients::Id)
              .on_delete(ForeignKeyAction::Cascade)
              .on_update(ForeignKeyAction::Cascade),
          )
          .foreign_key(
            ForeignKey::create()
              .name("fk_medical_appointments_practitioner_office_id")
              .from(
                MedicalAppointments::Table,
                MedicalAppointments::PractitionerOfficeId,
              )
              .to(PractitionerOffices::Table, PractitionerOffices::Id)
              .on_delete(ForeignKeyAction::Cascade)
              .on_update(ForeignKeyAction::Cascade),
          )
          .to_owned(),
      )
      .await?;

    // Using created_at::date as the date value for migrated records
    let copy_data_sql = r#"
      INSERT INTO medical_appointments (user_id, patient_id, practitioner_office_id, date, created_at, updated_at)
      SELECT user_id, patient_id, practitioner_office_id, created_at::date, created_at, updated_at
      FROM patient_users
    "#;
    let stmt = Statement::from_string(manager.get_database_backend(), copy_data_sql);
    manager.get_connection().execute(stmt).await?;

    // Make all columns NOT NULL after data is populated
    manager
      .alter_table(
        Table::alter()
          .table(MedicalAppointments::Table)
          .modify_column(
            ColumnDef::new(MedicalAppointments::UserId)
              .integer()
              .not_null(),
          )
          .to_owned(),
      )
      .await?;

    manager
      .alter_table(
        Table::alter()
          .table(MedicalAppointments::Table)
          .modify_column(
            ColumnDef::new(MedicalAppointments::PatientId)
              .integer()
              .not_null(),
          )
          .to_owned(),
      )
      .await?;

    manager
      .alter_table(
        Table::alter()
          .table(MedicalAppointments::Table)
          .modify_column(
            ColumnDef::new(MedicalAppointments::PractitionerOfficeId)
              .integer()
              .not_null(),
          )
          .to_owned(),
      )
      .await?;

    manager
      .alter_table(
        Table::alter()
          .table(MedicalAppointments::Table)
          .modify_column(ColumnDef::new(MedicalAppointments::Date).date().not_null())
          .to_owned(),
      )
      .await?;

    // Create indexes on foreign keys for performance
    manager
      .create_index(
        Index::create()
          .name("idx_medical_appointments_user_id")
          .table(MedicalAppointments::Table)
          .col(MedicalAppointments::UserId)
          .to_owned(),
      )
      .await?;

    manager
      .create_index(
        Index::create()
          .name("idx_medical_appointments_patient_id")
          .table(MedicalAppointments::Table)
          .col(MedicalAppointments::PatientId)
          .to_owned(),
      )
      .await?;

    manager
      .create_index(
        Index::create()
          .name("idx_medical_appointments_office_id")
          .table(MedicalAppointments::Table)
          .col(MedicalAppointments::PractitionerOfficeId)
          .to_owned(),
      )
      .await?;

    Ok(())
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    // Drop indexes
    manager
      .drop_index(
        Index::drop()
          .name("idx_medical_appointments_user_id")
          .table(MedicalAppointments::Table)
          .to_owned(),
      )
      .await?;

    manager
      .drop_index(
        Index::drop()
          .name("idx_medical_appointments_patient_id")
          .table(MedicalAppointments::Table)
          .to_owned(),
      )
      .await?;

    manager
      .drop_index(
        Index::drop()
          .name("idx_medical_appointments_office_id")
          .table(MedicalAppointments::Table)
          .to_owned(),
      )
      .await?;

    // Drop the medical_appointments table
    manager
      .drop_table(Table::drop().table(MedicalAppointments::Table).to_owned())
      .await?;

    Ok(())
  }
}

#[derive(Iden)]
enum MedicalAppointments {
  Table,
  Id,
  UserId,
  PatientId,
  PractitionerOfficeId,
  Date,
  CreatedAt,
  UpdatedAt,
}

#[derive(Iden)]
enum Users {
  Table,
  Id,
}

#[derive(Iden)]
enum Patients {
  Table,
  Id,
}

#[derive(Iden)]
enum PractitionerOffices {
  Table,
  Id,
}
