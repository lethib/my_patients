use loco_rs::cli;
use migration::Migrator;
use my_patients::app::App;

#[tokio::main]
async fn main() -> loco_rs::Result<()> {
  dotenvy::from_filename(".env.local").ok();
  cli::main::<App, Migrator>().await
}
