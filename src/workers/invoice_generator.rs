use axum::http::StatusCode;
use loco_rs::prelude::*;
use printpdf::*;
use serde::Serialize;
use std::io::BufWriter;

use crate::models::{
  _entities::{patients, user_business_informations, users},
  my_errors::MyErrors,
};
use sea_orm::{ActiveEnum, ColumnTrait, EntityTrait, QueryFilter};

pub struct InvoiceGeneratorWorker {
  pub ctx: AppContext,
}

#[derive(Debug, Serialize)]
pub struct InvoiceGeneratorArgs {
  pub patient: patients::Model,
  pub user: users::Model,
  pub amount: String,
}

#[derive(Debug, Serialize)]
pub struct InvoiceGenerationResult {
  pub success: bool,
  pub pdf_data: Option<Vec<u8>>,
  pub error: Option<String>,
}

#[async_trait]
impl BackgroundWorker<InvoiceGeneratorArgs> for InvoiceGeneratorWorker {
  fn build(ctx: &AppContext) -> Self {
    Self { ctx: ctx.clone() }
  }

  async fn perform(&self, args: InvoiceGeneratorArgs) -> loco_rs::Result<()> {
    let result = generate_invoice_pdf(&self.ctx.db, &args).await;

    match result {
      Ok(_) => Ok(()),
      Err(e) => Err(loco_rs::Error::string(&format!(
        "Invoice generation failed: {}",
        e
      ))),
    }
  }
}

/// Generate an invoice PDF based on the French invoice template
pub async fn generate_invoice_pdf<C: ConnectionTrait>(
  db: &C,
  args: &InvoiceGeneratorArgs,
) -> std::result::Result<Vec<u8>, MyErrors> {
  // Fetch business information separately
  let business_info = user_business_informations::Entity::find()
    .filter(user_business_informations::Column::UserId.eq(args.user.id))
    .one(db)
    .await?
    .ok_or_else(|| MyErrors {
      code: StatusCode::BAD_REQUEST,
      msg: "User business information not found".to_string(),
    })?;

  // Decrypt patient SSN
  let patient_ssn = args.patient.decrypt_ssn()?;

  // Generate PDF
  let pdf_data = create_modern_invoice_pdf(
    &args.user,
    &business_info,
    &args.patient,
    &patient_ssn,
    &args.amount,
  )
  .map_err(|e| MyErrors {
    code: StatusCode::INTERNAL_SERVER_ERROR,
    msg: format!("PDF creation failed: {}", e),
  })?;

  Ok(pdf_data)
}

/// Create a simple invoice PDF matching the provided template
fn create_modern_invoice_pdf(
  user: &users::Model,
  business_info: &user_business_informations::Model,
  patient: &patients::Model,
  patient_ssn: &str,
  amount: &str,
) -> std::result::Result<Vec<u8>, String> {
  // Create A4 PDF document
  let (doc, page1, layer1) = PdfDocument::new(
    "Note d'honoraires acquittée",
    Mm(210.0),
    Mm(297.0),
    "Main Layer",
  );
  let current_layer = doc.get_page(page1).get_layer(layer1);

  // Load fonts
  let font_bold = doc
    .add_builtin_font(BuiltinFont::HelveticaBold)
    .map_err(|e| format!("Failed to load bold font: {}", e))?;
  let font_regular = doc
    .add_builtin_font(BuiltinFont::Helvetica)
    .map_err(|e| format!("Failed to load regular font: {}", e))?;

  // Color - black only
  let black = Color::Rgb(Rgb::new(0.0, 0.0, 0.0, None));

  // Page setup
  let page_height = Mm(297.0);
  let margin = Mm(25.0);
  let mut y_position = page_height - margin - Mm(10.0);

  // === HEADER SECTION ===
  current_layer.set_fill_color(black.clone());

  // Practitioner name with title on same line, separated by dash
  let full_name = format!("{} – OSTEOPATHE D.O", &user.full_name());
  current_layer.use_text(&full_name, 14.0, margin, y_position, &font_bold);
  y_position -= Mm(12.0);

  // Professional numbers with consistent formatting
  if let Some(ref adeli) = business_info.adeli_number {
    current_layer.use_text(
      &format!("N° Adeli : {}", adeli),
      10.0,
      margin,
      y_position,
      &font_regular,
    );
    y_position -= Mm(5.0);
  }

  current_layer.use_text(
    &format!("N°RPPS : {}", business_info.rpps_number),
    10.0,
    margin,
    y_position,
    &font_regular,
  );
  y_position -= Mm(5.0);

  current_layer.use_text(
    &format!("N°SIRET : {}", business_info.siret_number),
    10.0,
    margin,
    y_position,
    &font_regular,
  );
  y_position -= Mm(12.0);

  // Office address - Temporary data
  let address_line_1 = "Adress line 1";
  let address_line_2 = "ZIP_CODE City";

  current_layer.use_text(address_line_1, 10.0, margin, y_position, &font_regular);
  y_position -= Mm(5.0);
  current_layer.use_text(address_line_2, 10.0, margin, y_position, &font_regular);
  y_position -= Mm(8.0);

  // Contact info
  current_layer.use_text(
    &format!("Tel : {}", &user.phone_number),
    10.0,
    margin,
    y_position,
    &font_regular,
  );
  y_position -= Mm(8.0);

  current_layer.use_text(&user.email, 10.0, margin, y_position, &font_regular);
  y_position -= Mm(30.0);

  // === INVOICE TITLE - CENTERED ===
  let title = "Note d'honoraires acquittée";
  // Center the title properly
  current_layer.use_text(title, 20.0, Mm(60.0), y_position, &font_bold);
  y_position -= Mm(30.0);

  // === PATIENT INFORMATION ===
  // Patient name with italic style indicator
  let patient_full_name = format!("{} {}", patient.last_name, patient.first_name);
  let full_text = format!("Reçu de : {}", patient_full_name);

  // Add the text
  current_layer.use_text(&full_text, 11.0, margin, y_position, &font_regular);

  // Draw underline only for "Reçu de :"
  let underline_y = y_position - Mm(1.0); // Position line 1mm below text baseline

  current_layer.set_outline_thickness(0.3);
  let underline = Line {
    points: vec![
      (Point::new(margin, underline_y), false),
      (Point::new(margin + Mm(17.0), underline_y), false),
    ],
    is_closed: false,
  };
  current_layer.add_line(underline);

  y_position -= Mm(12.0);

  // Social security number with box
  let ssn_y = y_position;
  current_layer.use_text(
    &format!("Numéro de sécurité sociale : {}", patient_ssn),
    11.0,
    margin,
    y_position,
    &font_regular,
  );

  // Draw box around SSN field
  current_layer.set_outline_color(black.clone());
  current_layer.set_outline_thickness(0.5);
  let ssn_box = Line {
    points: vec![
      (Point::new(margin - Mm(2.0), ssn_y + Mm(5.0)), false),
      (Point::new(Mm(185.0), ssn_y + Mm(5.0)), false),
      (Point::new(Mm(185.0), ssn_y - Mm(3.0)), false),
      (Point::new(margin - Mm(2.0), ssn_y - Mm(3.0)), false),
      (Point::new(margin - Mm(2.0), ssn_y + Mm(3.0)), false),
    ],
    is_closed: true,
  };
  current_layer.add_line(ssn_box);

  y_position -= Mm(18.0);

  // Address with box
  let addr_y = y_position;
  let address_text = format!(
    "Adresse : {} – {} {}",
    patient.address_line_1, patient.address_zip_code, patient.address_city
  );
  current_layer.use_text(&address_text, 11.0, margin, y_position, &font_regular);

  // Draw box around address field
  let addr_box = Line {
    points: vec![
      (Point::new(margin - Mm(2.0), addr_y + Mm(5.0)), false),
      (Point::new(Mm(185.0), addr_y + Mm(5.0)), false),
      (Point::new(Mm(185.0), addr_y - Mm(3.0)), false),
      (Point::new(margin - Mm(2.0), addr_y - Mm(3.0)), false),
      (Point::new(margin - Mm(2.0), addr_y + Mm(3.0)), false),
    ],
    is_closed: true,
  };
  current_layer.add_line(addr_box);

  y_position -= Mm(18.0);

  // Amount with underline style
  let amount_replaced = amount.replace("€", "");
  let amount_clean = amount_replaced.trim();
  let full_text = format!("Honoraire : {}€", amount_clean);

  current_layer.use_text(&full_text, 11.0, margin, y_position, &font_regular);

  // Draw underline only for "Honoraire :"
  let underline_text = "Honoraire :";
  let text_width = underline_text.len() as f32 * 2.5; // Rough estimate: 2.5mm per character at 11pt
  let underline_y = y_position - Mm(1.0); // Position line 1mm below text baseline

  current_layer.set_outline_thickness(0.3);
  let underline = Line {
    points: vec![
      (Point::new(margin, underline_y), false),
      (Point::new(margin + Mm(text_width), underline_y), false),
    ],
    is_closed: false,
  };
  current_layer.add_line(underline);
  y_position -= Mm(35.0);

  // === DATE AND SIGNATURE ===
  let current_date = chrono::Utc::now().format("%d/%m/%Y").to_string();
  let date_location = format!("Fait à {}, le {}", patient.office.to_value(), current_date);

  // Right align date
  let date_x = Mm(210.0) - margin - Mm(85.0);
  current_layer.use_text(&date_location, 11.0, date_x, y_position, &font_regular);
  y_position -= Mm(40.0);

  // Signature placeholder - right aligned
  let sig_x = Mm(210.0) - margin - Mm(50.0);
  current_layer.use_text(&user.full_name(), 11.0, sig_x, y_position, &font_regular);

  // Convert to bytes
  let mut buf = Vec::new();
  doc
    .save(&mut BufWriter::new(&mut buf))
    .map_err(|e| format!("Failed to save PDF: {}", e))?;

  Ok(buf)
}
