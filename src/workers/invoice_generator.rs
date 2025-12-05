use axum::http::StatusCode;
use printpdf::*;
use serde::Serialize;
use std::io::BufWriter;

use crate::models::{
  _entities::{patients, practitioner_offices, user_business_informations, users},
  my_errors::MyErrors,
};
use crate::services::storage::StorageService;
use sea_orm::{prelude::Date, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

#[derive(Debug, Clone, Serialize)]
pub struct InvoiceGeneratorArgs {
  pub patient: patients::Model,
  pub user: users::Model,
  pub amount: String,
  pub invoice_date: Date,
  pub practitioner_office: practitioner_offices::Model,
}

#[derive(Debug, Serialize)]
pub struct InvoiceGenerationResult {
  pub success: bool,
  pub pdf_data: Option<Vec<u8>>,
  pub error: Option<String>,
}

/// Process invoice generation as a background job
#[allow(dead_code)]
pub async fn process_invoice(
  args: InvoiceGeneratorArgs,
  db: &DatabaseConnection,
) -> anyhow::Result<()> {
  tracing::info!(
    "Starting invoice generation for patient: {} {}",
    args.patient.first_name,
    args.patient.last_name
  );

  let result = generate_invoice_pdf(db, &args).await;

  match result {
    Ok(_) => {
      tracing::info!("Invoice generated successfully");
      Ok(())
    }
    Err(e) => {
      tracing::error!("Invoice generation failed: {:?}", e);
      Err(anyhow::anyhow!("Invoice generation failed: {:?}", e))
    }
  }
}

/// Generate an invoice PDF based on the French invoice template
pub async fn generate_invoice_pdf(
  db: &DatabaseConnection,
  args: &InvoiceGeneratorArgs,
) -> std::result::Result<Vec<u8>, MyErrors> {
  // Initialize storage service for signature fetching
  let storage_service = match StorageService::new() {
    Ok(service) => Some(service),
    Err(e) => {
      tracing::warn!(
        "Storage service unavailable: {}. Continuing without signature.",
        e
      );
      None
    }
  };

  // Fetch business information separately
  let business_info = user_business_informations::Entity::find()
    .filter(user_business_informations::Column::UserId.eq(args.user.id))
    .one(db)
    .await?
    .ok_or_else(|| MyErrors {
      code: StatusCode::BAD_REQUEST,
      msg: "User business information not found".to_string(),
    })?;

  // Try to fetch signature if storage service is available
  let signature_data = match &storage_service {
    Some(service) => match service
      .fetch_signature(&business_info.signature_file_name)
      .await
    {
      Ok(data) => {
        tracing::info!("Successfully fetched signature for user {}", args.user.id);
        Some(data)
      }
      Err(e) => {
        tracing::warn!(
          "Failed to fetch signature for user {}: {}. Continuing without signature.",
          args.user.id,
          e
        );
        None
      }
    },
    None => None,
  };

  // Decrypt patient SSN
  let patient_ssn = args.patient.decrypt_ssn()?;

  // Generate PDF
  let pdf_data = create_modern_invoice_pdf(
    &args.user,
    &business_info,
    &args.patient,
    &patient_ssn,
    &args.amount,
    &args.invoice_date,
    &args.practitioner_office,
    signature_data.as_deref(),
  )
  .map_err(|e| MyErrors {
    code: StatusCode::INTERNAL_SERVER_ERROR,
    msg: format!("PDF creation failed: {}", e),
  })?;

  Ok(pdf_data)
}

/// Embed a signature image into the PDF
///
/// # Arguments
/// * `doc` - The PDF document
/// * `layer` - The current layer to draw on
/// * `image_bytes` - The image bytes (JPG/PNG)
/// * `x` - X position for the image
/// * `y` - Y position for the image
///
/// # Returns
/// * `Result<(), String>` - Success or error message
fn embed_signature_image(
  _doc: &PdfDocumentReference,
  layer: &PdfLayerReference,
  image_bytes: &[u8],
  x: Mm,
  y: Mm,
) -> std::result::Result<(), String> {
  // Load and decode the image (auto-detect format)
  let img =
    ::image::load_from_memory(image_bytes).map_err(|e| format!("Failed to decode image: {}", e))?;

  // Convert to RGB if needed
  let rgb_img = img.to_rgb8();
  let (width, height) = rgb_img.dimensions();

  tracing::info!("Loaded signature image: {}x{} pixels", width, height);

  // Calculate aspect ratio and target dimensions
  let max_width = Mm(200.0); // Maximum width for signature
  let max_height = Mm(100.0); // Maximum height for signature

  let aspect_ratio = width as f32 / height as f32;
  let (target_width, target_height) = if aspect_ratio > max_width.0 / max_height.0 {
    // Width-constrained
    (max_width, Mm(max_width.0 / aspect_ratio))
  } else {
    // Height-constrained
    (Mm(max_height.0 * aspect_ratio), max_height)
  };

  tracing::info!(
    "Target signature dimensions: {}x{} mm",
    target_width.0,
    target_height.0
  );

  // Get raw RGB image data
  let raw_image_data = rgb_img.into_raw();

  // Create ImageXObject with proper structure for printpdf 0.7
  let image_xobject = ImageXObject {
    width: Px(width as usize),
    height: Px(height as usize),
    color_space: ColorSpace::Rgb,
    bits_per_component: ColorBits::Bit8,
    interpolate: true,
    image_data: raw_image_data,
    image_filter: None, // Use no compression for better compatibility
    smask: None,
    clipping_bbox: None,
  };

  tracing::info!(
    "Created ImageXObject with {} bytes of RGB data",
    image_xobject.image_data.len()
  );

  // Convert to Image wrapper
  let image = Image::from(image_xobject);

  // Calculate scaling factors based on target size in mm
  // printpdf assumes 72 DPI by default for images
  let pixels_per_mm = 72.0 / 25.4; // ~2.834 pixels per mm at 72 DPI

  // Calculate what the image size would be in mm at 72 DPI
  let original_width_mm = width as f32 / pixels_per_mm;
  let original_height_mm = height as f32 / pixels_per_mm;

  // Calculate scale factors to achieve target size
  let scale_x = target_width.0 / original_width_mm;
  let scale_y = target_height.0 / original_height_mm;

  tracing::info!(
    "Image scaling: original {}x{} pixels ({:.2}x{:.2} mm at 72 DPI), target {}x{} mm, scale {:.3}x{:.3}",
    width,
    height,
    original_width_mm,
    original_height_mm,
    target_width.0,
    target_height.0,
    scale_x,
    scale_y
  );

  // Calculate position for image (printpdf coordinate system uses bottom-left origin)
  // Keep coordinates in Mm since ImageTransform expects Mm
  let pdf_x = x;
  let pdf_y = y - target_height; // Adjust for image height

  // Create transform to position and scale the image
  let transform = ImageTransform {
    translate_x: Some(pdf_x),
    translate_y: Some(pdf_y),
    rotate: None,
    scale_x: Some(scale_x),
    scale_y: Some(scale_y),
    dpi: None, // Don't use DPI when we're manually scaling
  };

  // Add the image to the layer
  image.add_to_layer(layer.clone(), transform);

  tracing::info!(
    "Successfully embedded signature image at position ({:.2}, {:.2}) mm with scale ({:.3}, {:.3})",
    pdf_x.0,
    pdf_y.0,
    scale_x,
    scale_y
  );

  Ok(())
}

/// Create a simple invoice PDF matching the provided template
fn create_modern_invoice_pdf(
  user: &users::Model,
  business_info: &user_business_informations::Model,
  patient: &patients::Model,
  patient_ssn: &str,
  amount: &str,
  invoice_date: &Date,
  practitioner_office: &practitioner_offices::Model,
  signature_data: Option<&[u8]>,
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

  current_layer.use_text(
    practitioner_office.address_line_1.clone(),
    10.0,
    margin,
    y_position,
    &font_regular,
  );
  y_position -= Mm(5.0);
  current_layer.use_text(
    format!(
      "{} {}",
      practitioner_office.address_zip_code, practitioner_office.address_city,
    ),
    10.0,
    margin,
    y_position,
    &font_regular,
  );
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
  let invoice_date = invoice_date.format("%d/%m/%Y").to_string();
  let date_location = format!(
    "Fait à {}, le {}",
    practitioner_office.address_city, invoice_date
  );

  // Right align date
  let date_x = Mm(230.0) - margin - Mm(85.0);
  current_layer.use_text(&date_location, 11.0, date_x, y_position, &font_regular);

  // Signature section - left aligned
  let sig_x = Mm(30.0);

  // Try to embed signature image if available
  if let Some(sig_bytes) = signature_data {
    y_position += Mm(70.0);

    match embed_signature_image(&doc, &current_layer, sig_bytes, sig_x, y_position) {
      Ok(_) => {
        tracing::info!("Successfully embedded signature image");
        // Move down to avoid overlap with signature image
        y_position -= Mm(100.0);
      }
      Err(e) => {
        tracing::warn!(
          "Failed to embed signature image: {}. Using text fallback.",
          e
        );
      }
    }
  } else {
    y_position -= Mm(30.0)
  }

  current_layer.use_text(
    &user.full_name(),
    11.0,
    date_x + Mm(20.0),
    y_position,
    &font_regular,
  );

  // Convert to bytes
  let mut buf = Vec::new();
  doc
    .save(&mut BufWriter::new(&mut buf))
    .map_err(|e| format!("Failed to save PDF: {}", e))?;

  Ok(buf)
}
