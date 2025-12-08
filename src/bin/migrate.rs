use std::{env, error::Error};

use migration::MigratorTrait;
use my_patients::config::Config;
use sea_orm::Database;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  dotenvy::from_filename(".env.local").ok();

  let environment = env::var("ENVIRONMENT").unwrap_or("development".to_string());
  let config = Config::load(&environment)?;

  println!("Connecting to database...");
  let db = Database::connect(&config.database.url).await?;

  println!("Running migrations...");
  migration::Migrator::up(&db, None).await?;

  println!("Migrations run successfully!");

  Ok(())
}
