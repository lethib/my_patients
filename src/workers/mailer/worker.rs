use async_trait::async_trait;
use lettre::{
  message::{header::ContentType, Attachment, Mailbox, MultiPart, SinglePart},
  transport::smtp::authentication::Credentials,
  AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};
use loco_rs::{
  app::AppContext,
  config::{MailerAuth, SmtpMailer},
  prelude::BackgroundWorker,
};

use crate::workers::mailer::args::EmailArgs;

pub struct EmailWorker {
  pub config: SmtpMailer,
  pub auth: MailerAuth,
}

#[async_trait]
impl BackgroundWorker<EmailArgs> for EmailWorker {
  fn class_name() -> String
  where
    Self: Sized,
  {
    "EmailWorker".to_string()
  }

  fn build(ctx: &AppContext) -> Self {
    let mailer_config = ctx
      .config
      .mailer
      .as_ref()
      .expect("Mailer configuration is required but missing from application config");

    let smtp_config = mailer_config
      .smtp
      .as_ref()
      .expect("SMTP configuration is required but missing from mailer config");

    Self {
      config: smtp_config.clone(),
      auth: smtp_config
        .auth
        .as_ref()
        .expect("SMTP authentication credentials are required but missing from SMTP config")
        .clone(),
    }
  }

  async fn perform(&self, args: EmailArgs) -> loco_rs::Result<()> {
    tracing::info!("Start sending email with args");
    let email = Self::build_email(&self, &args)?;

    Self::send_email(&self, email).await?;

    tracing::info!("Email sent");

    Ok(())
  }
}

impl EmailWorker {
  fn build_email(&self, args: &EmailArgs) -> loco_rs::Result<Message> {
    let to: Mailbox = if let Some(name) = &args.to_name {
      format!("{} <{}>", name, args.to)
        .parse()
        .map_err(|e| loco_rs::Error::Message(format!("Invalid to address: {}", e)))?
    } else {
      args
        .to
        .parse()
        .map_err(|e| loco_rs::Error::Message(format!("Invalid to address: {}", e)))?
    };

    let mut message_builder = Message::builder()
      .from(format!("{} <{}>", "My Patients", self.auth.user).parse()?)
      .to(to)
      .subject(&args.subject);

    if let Some(reply_to) = &args.reply_to {
      message_builder = message_builder.reply_to(reply_to.parse()?);
    }

    let message = if args.attachments.is_empty() {
      Self::build_simple_body(message_builder, args)?
    } else {
      Self::build_multipart_body(message_builder, args)?
    };

    Ok(message)
  }

  fn build_simple_body(
    builder: lettre::message::MessageBuilder,
    args: &EmailArgs,
  ) -> loco_rs::Result<Message> {
    builder
      .body(args.text_body.clone())
      .map_err(|e| loco_rs::Error::Message(format!("Failed to build email: {}", e)))
  }

  fn build_multipart_body(
    builder: lettre::message::MessageBuilder,
    args: &EmailArgs,
  ) -> loco_rs::Result<Message> {
    let mut multipart = MultiPart::mixed().singlepart(SinglePart::plain(args.text_body.clone()));

    for attachment in &args.attachments {
      let data = attachment
        .decode_data()
        .map_err(|e| loco_rs::Error::Message(format!("Failed to decode attachment: {}", e)))?;

      let content_type: ContentType = attachment
        .content_type
        .parse()
        .map_err(|e| loco_rs::Error::Message(format!("Invalid content type: {}", e)))?;

      multipart =
        multipart.singlepart(Attachment::new(attachment.filename.clone()).body(data, content_type));
    }

    Ok(builder.multipart(multipart)?)
  }

  async fn send_email(&self, email: Message) -> loco_rs::Result<()> {
    let creds = Credentials::new(self.auth.user.clone(), self.auth.password.clone());

    let transport = AsyncSmtpTransport::<Tokio1Executor>::relay(&self.config.host)
      .map_err(|e| loco_rs::Error::Message(format!("Failed to create SMTP transport: {}", e)))?
      .credentials(creds)
      .port(self.config.port)
      .build();

    transport
      .send(email)
      .await
      .map_err(|e| loco_rs::Error::Message(format!("Failed to send email: {}", e)))?;

    Ok(())
  }
}
