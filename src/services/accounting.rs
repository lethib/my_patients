use chrono::NaiveDate;
use rust_xlsxwriter::*;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder};
use std::collections::HashMap;

use crate::models::{
  _entities::{medical_appointments, patients, practitioner_offices},
  my_errors::MyErrors,
  users,
};

pub struct MedicalAppointmentExtractor<'user> {
  user: &'user users::Model,
}

type MedicalAppointmentDetail = (
  medical_appointments::Model,
  patients::Model,
  practitioner_offices::Model,
);

pub trait ToExcel {
  fn to_excel(&self) -> Result<Workbook, MyErrors>;
}

impl ToExcel for Vec<MedicalAppointmentDetail> {
  fn to_excel(&self) -> Result<Workbook, MyErrors> {
    let mut appointments_by_office: HashMap<String, Vec<&MedicalAppointmentDetail>> =
      HashMap::new();

    for appointment in self {
      let office_name = appointment.2.name.clone();
      appointments_by_office
        .entry(office_name)
        .or_insert_with(Vec::new)
        .push(appointment);
    }

    let mut workbook = Workbook::new();
    let date_format = Format::new().set_num_format("dd/mm/yyyy");

    // Create a worksheet for each office
    for (office_name, office_appointments) in appointments_by_office.iter() {
      let worksheet = workbook.add_worksheet();
      worksheet.set_name(office_name)?;

      for (i, (appointment, patient, _office)) in office_appointments.iter().enumerate() {
        let excel_date = ExcelDateTime::parse_from_str(&appointment.date.to_string())?;
        worksheet.write_with_format(i as u32, 0, &excel_date, &date_format)?;

        worksheet.write(i as u32, 1, &patient.first_name)?;
        worksheet.write(i as u32, 2, &patient.last_name)?;
        worksheet.write(i as u32, 3, appointment.price_in_cents)?;
      }
    }

    Ok(workbook)
  }
}

impl<'user> MedicalAppointmentExtractor<'user> {
  pub fn for_user(user: &'user users::Model) -> Self {
    MedicalAppointmentExtractor { user }
  }

  pub async fn extract(
    &self,
    db: &DatabaseConnection,
    start_date: NaiveDate,
    end_date: NaiveDate,
  ) -> Result<Vec<MedicalAppointmentDetail>, MyErrors> {
    let appointments = medical_appointments::Entity::find()
      .filter(medical_appointments::Column::UserId.eq(self.user.id))
      .filter(medical_appointments::Column::Date.between(start_date, end_date))
      .inner_join(patients::Entity)
      .inner_join(practitioner_offices::Entity)
      .select_also(patients::Entity)
      .select_also(practitioner_offices::Entity)
      .order_by_asc(medical_appointments::Column::PractitionerOfficeId)
      .order_by_asc(medical_appointments::Column::Date)
      .order_by_asc(patients::Column::LastName)
      .all(db)
      .await?;

    let results = appointments
      .into_iter()
      .map(|(appointment, patient, office)| {
        (
          appointment,
          patient.expect("patient should be define"),
          office.expect("office should be define"),
        )
      })
      .collect();

    Ok(results)
  }
}
