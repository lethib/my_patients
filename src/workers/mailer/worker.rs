use crate::{config::Config, workers::mailer::args::EmailArgs};
use lettre::{
  message::{header::ContentType, Attachment, Mailbox, MultiPart, SinglePart},
  transport::smtp::authentication::Credentials,
  AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};
use std::sync::Arc;

pub async fn process_email(args: EmailArgs, config: &Arc<Config>) -> anyhow::Result<()> {
  tracing::info!("Start sending email to: {}", args.to);

  let email = build_email(&args, config)?;
  send_email(email, config).await?;

  tracing::info!("Email sent successfully");
  Ok(())
}

fn build_email(args: &EmailArgs, _config: &Arc<Config>) -> anyhow::Result<Message> {
  let to: Mailbox = if let Some(name) = &args.to_name {
    format!("{} <{}>", name, args.to)
      .parse()
      .map_err(|e| anyhow::anyhow!("Invalid to address: {}", e))?
  } else {
    args
      .to
      .parse()
      .map_err(|e| anyhow::anyhow!("Invalid to address: {}", e))?
  };

  let smtp_user = std::env::var("SMTP_AUTH_USER")
    .map_err(|_| anyhow::anyhow!("SMTP_AUTH_USER environment variable not set"))?;

  let mut message_builder = Message::builder()
    .from(
      format!("{} <{}>", "My Patients", smtp_user)
        .parse()
        .map_err(|e| anyhow::anyhow!("Invalid from address: {}", e))?,
    )
    .to(to)
    .subject(&args.subject);

  if let Some(reply_to) = &args.reply_to {
    message_builder = message_builder.reply_to(
      reply_to
        .parse()
        .map_err(|e| anyhow::anyhow!("Invalid reply-to address: {}", e))?,
    );
  }

  let message = if args.attachments.is_empty() {
    build_simple_body(message_builder, args)?
  } else {
    build_multipart_body(message_builder, args)?
  };

  Ok(message)
}

fn build_simple_body(
  builder: lettre::message::MessageBuilder,
  args: &EmailArgs,
) -> anyhow::Result<Message> {
  builder
    .body(args.text_body.clone())
    .map_err(|e| anyhow::anyhow!("Failed to build email: {}", e))
}

fn build_multipart_body(
  builder: lettre::message::MessageBuilder,
  args: &EmailArgs,
) -> anyhow::Result<Message> {
  let mut multipart = MultiPart::mixed().singlepart(SinglePart::plain(args.text_body.clone()));

  for attachment in &args.attachments {
    let data = attachment
      .decode_data()
      .map_err(|e| anyhow::anyhow!("Failed to decode attachment: {}", e))?;

    let content_type: ContentType = attachment
      .content_type
      .parse()
      .map_err(|e| anyhow::anyhow!("Invalid content type: {}", e))?;

    multipart =
      multipart.singlepart(Attachment::new(attachment.filename.clone()).body(data, content_type));
  }

  Ok(
    builder
      .multipart(multipart)
      .map_err(|e| anyhow::anyhow!("Failed to build multipart email: {}", e))?,
  )
}

async fn send_email(email: Message, _config: &Arc<Config>) -> anyhow::Result<()> {
  let smtp_host = std::env::var("SMTP_SERVER_HOST")
    .map_err(|_| anyhow::anyhow!("SMTP_SERVER_HOST environment variable not set"))?;
  let smtp_port: u16 = std::env::var("SMTP_SERVER_PORT")
    .map_err(|_| anyhow::anyhow!("SMTP_SERVER_PORT environment variable not set"))?
    .parse()
    .map_err(|_| anyhow::anyhow!("SMTP_SERVER_PORT must be a valid port number"))?;
  let smtp_user = std::env::var("SMTP_AUTH_USER")
    .map_err(|_| anyhow::anyhow!("SMTP_AUTH_USER environment variable not set"))?;
  let smtp_password = std::env::var("SMTP_AUTH_PASSWORD")
    .map_err(|_| anyhow::anyhow!("SMTP_AUTH_PASSWORD environment variable not set"))?;

  let creds = Credentials::new(smtp_user, smtp_password);

  let transport = AsyncSmtpTransport::<Tokio1Executor>::relay(&smtp_host)
    .map_err(|e| anyhow::anyhow!("Failed to create SMTP transport: {}", e))?
    .credentials(creds)
    .port(smtp_port)
    .build();

  transport
    .send(email)
    .await
    .map_err(|e| anyhow::anyhow!("Failed to send email: {}", e))?;

  Ok(())
}
