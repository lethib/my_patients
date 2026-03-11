use chrono::NaiveDate;
use rust_xlsxwriter::*;
use sea_orm::{ActiveEnum, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder};
use std::collections::HashMap;

use crate::models::{
  _entities::{medical_appointments, patients, practitioner_offices, user_practitioner_offices},
  my_errors::{unexpected_error::UnexpectedError, MyErrors},
  users,
};

pub struct MedicalAppointmentExtractor<'user> {
  user: &'user users::Model,
}

type MedicalAppointmentDetail = (
  medical_appointments::Model,
  patients::Model,
  practitioner_offices::Model,
  f64, // revenue_share_percentage
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
        .or_default()
        .push(appointment);
    }

    let mut workbook = Workbook::new();
    let date_format = Format::new().set_num_format("dd/mm/yyyy");
    let header_format = Format::new()
      .set_bold()
      .set_background_color(Color::Green)
      .set_font_color(Color::White);

    let mut sorted_offices: Vec<_> = appointments_by_office.iter().collect();
    sorted_offices.sort_by_key(|(name, _)| name.as_str());

    // Create a worksheet for each office
    for (office_name, office_appointments) in sorted_offices {
      let worksheet = workbook.add_worksheet();
      worksheet.set_name(office_name)?;

      worksheet.write_with_format(0, 0, "date", &header_format)?;
      worksheet.set_column_width(0, 15)?;

      worksheet.write_with_format(0, 1, "first_name", &header_format)?;
      worksheet.set_column_width(1, 20)?;

      worksheet.write_with_format(0, 2, "last_name", &header_format)?;
      worksheet.set_column_width(2, 20)?;

      worksheet.write_with_format(0, 3, "payment_method", &header_format)?;
      worksheet.set_column_width(3, 15)?;

      worksheet.write_with_format(0, 4, "price_in_euros", &header_format)?;
      worksheet.set_column_width(4, 20)?;

      worksheet.write_with_format(0, 5, "your_revenue", &header_format)?;
      worksheet.set_column_width(5, 20)?;

      worksheet.write_with_format(0, 6, "hand_back", &header_format)?;
      worksheet.set_column_width(6, 20)?;

      for (i, (appointment, patient, _office, revenue_share_percentage)) in
        office_appointments.iter().enumerate()
      {
        let excel_date = ExcelDateTime::parse_from_str(&appointment.date.to_string())?;
        let price = appointment.price_in_cents as f64 / 100.0;
        let hand_back = price * revenue_share_percentage / 100.0;

        worksheet.write_with_format(i as u32 + 1, 0, &excel_date, &date_format)?;

        worksheet.write(i as u32 + 1, 1, &patient.first_name)?;
        worksheet.write(i as u32 + 1, 2, &patient.last_name)?;
        worksheet.write(
          i as u32 + 1,
          3,
          appointment.payment_method.clone().map(|p| p.to_value()),
        )?;

        worksheet.write(i as u32 + 1, 4, price)?;
        worksheet.write(i as u32 + 1, 5, format!("{:.2}", price - hand_back))?;
        worksheet.write(i as u32 + 1, 6, format!("{:.2}", hand_back))?;
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

    let user_offices = user_practitioner_offices::Entity::find()
      .filter(user_practitioner_offices::Column::UserId.eq(self.user.id))
      .all(db)
      .await?;

    let revenue_share_by_office: HashMap<i32, f64> = user_offices
      .into_iter()
      .map(|uo| {
        let pct = uo
          .revenue_share_percentage
          .to_string()
          .parse::<f64>()
          .unwrap_or(0.0);
        (uo.practitioner_office_id, pct)
      })
      .collect();

    let results = appointments
      .into_iter()
      .map(|(appointment, patient, office)| -> Result<_, MyErrors> {
        let office = office.ok_or(UnexpectedError::new("office_should_be_define".to_string()))?;
        let revenue_share_percentage =
          *revenue_share_by_office
            .get(&office.id)
            .ok_or(UnexpectedError::new(
              "revenue_share_percentage_should_be_define".to_string(),
            ))?;
        Ok((
          appointment,
          patient.ok_or(UnexpectedError::new("patient_should_be_define".to_string()))?,
          office,
          revenue_share_percentage,
        ))
      })
      .collect::<Result<Vec<_>, MyErrors>>()?;

    Ok(results)
  }
}
