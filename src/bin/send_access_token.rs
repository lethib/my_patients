use std::{env, error, sync::Arc};

use my_patients::{
  config::Config,
  models::_entities::users::Entity as Users,
  workers::mailer::{self, args::EmailArgs},
};
use sea_orm::{Database, EntityTrait};

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
  dotenvy::from_filename(".env.local").ok();

  let environment = env::var("ENVIRONMENT").unwrap_or("development".to_string());
  let config = Arc::new(Config::load(&environment)?);

  let args: Vec<String> = env::args().collect();
  let user_id: i32 = args.get(1).ok_or("user_id must be provided")?.parse()?;

  let db = Database::connect(&config.database.url).await?;

  let user_to_invite = Users::find_by_id(user_id)
    .one(&db)
    .await?
    .ok_or("user not found")?;

  match user_to_invite.access_key {
    Some(access_key) => {
      let email_args = EmailArgs::new_text(user_to_invite.email, "Votre code d'accès à My Patients".to_string(), format!("Bonjour,\n\nVoici votre code d'accès à la plateforme My Patients: {}\nVous pouvez l'utiliser juste après vous être connecté: https://my-patients-64695224709.europe-west9.run.app/login", access_key));

      println!("Sending email...");

      mailer::worker::process_email(email_args, &config).await?;

      println!("Email sent successfully !")
    }
    None => println!("No access key registered for this user"),
  }

  Ok(())
}
