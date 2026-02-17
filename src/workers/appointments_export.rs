use crate::{
  app_state::{AppState, WorkerJob},
  models::{
    my_errors::{unexpected_error::UnexpectedError, MyErrors},
    users::users,
  },
  services::appointments::{MedicalAppointmentExtractor, ToExcel},
  workers::mailer::{args::EmailArgs, attachment::EmailAttachment},
};
use chrono::NaiveDate;
use rust_xlsxwriter::Workbook;

#[derive(Debug, Clone)]
pub struct Args {
  pub user: users::Model,
  pub start_date: NaiveDate,
  pub end_date: NaiveDate,
}

const EXCEL_CONTENT_TYPE: &str =
  "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet";

pub async fn process_appointment_extraction(args: Args, state: AppState) -> Result<(), MyErrors> {
  let workbook = MedicalAppointmentExtractor::for_user(&args.user)
    .extract(&state.db, args.start_date, args.end_date)
    .await?
    .to_excel()?;

  send_excel_by_mail(
    workbook,
    args.user.email,
    args.start_date,
    args.end_date,
    state,
  )
  .await?;

  Ok(())
}

async fn send_excel_by_mail(
  mut workbook: Workbook,
  to: String,
  start_date: NaiveDate,
  end_date: NaiveDate,
  state: AppState,
) -> Result<(), MyErrors> {
  let wb_buffer = workbook.save_to_buffer()?;

  let workbook_attachment = EmailAttachment::from_bytes(
    format!(
      "appointments_from_{}_to_{}.xlsx",
      start_date.format("%d-%m-%Y"),
      end_date.format("%d-%m-%Y")
    ),
    EXCEL_CONTENT_TYPE.to_string(),
    &wb_buffer,
  );

  let email_args = EmailArgs::new_text(
    to,
    format!(
      "Vos RDV du {} au {}",
      start_date.format("%d/%m/%Y"),
      end_date.format("%d/%m/%Y")
    ),
    "Bonjour,\n\nVous trouverez tous vos rendez-vous de la période sélectionnée en pièce jointe"
      .to_string(),
  )
  .with_attachment(workbook_attachment);

  state
    .worker_transmitter
    .send(WorkerJob::Email(email_args))
    .await
    .map_err(|_| UnexpectedError::ShouldNotHappen)?;

  Ok(())
}
