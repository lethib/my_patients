use crate::workers::mailer::attachment::EmailAttachment;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailArgs {
  pub to: String,
  pub to_name: Option<String>,
  pub subject: String,
  pub text_body: String,
  // pub html_body: Option<String>,
  pub attachments: Vec<EmailAttachment>,
  pub reply_to: Option<String>,
}

impl EmailArgs {
  pub fn new_text(to: String, subject: String, text_body: String) -> Self {
    Self {
      to,
      to_name: None,
      subject,
      text_body,
      attachments: Vec::new(),
      reply_to: None,
    }
  }

  pub fn with_attachment(mut self, attachment: EmailAttachment) -> Self {
    self.attachments.push(attachment);
    self
  }

  pub fn with_reply_to(mut self, reply_to: String) -> Self {
    self.reply_to = Some(reply_to);
    self
  }
}
