use async_trait::async_trait;
use loco_rs::{
  app::{AppContext, Hooks, Initializer},
  bgworker::{BackgroundWorker, Queue},
  boot::{create_app, BootResult, StartMode},
  config::Config,
  controller::AppRoutes,
  db::{self, truncate_table},
  environment::Environment,
  task::Tasks,
  Result,
};
use migration::Migrator;
use std::path::Path;

#[allow(unused_imports)]
use crate::{
  controllers, initializers, models::_entities::users, tasks, workers::downloader::DownloadWorker,
};

pub struct App;
#[async_trait]
impl Hooks for App {
  fn app_name() -> &'static str {
    env!("CARGO_CRATE_NAME")
  }

  fn app_version() -> String {
    format!(
      "{} ({})",
      env!("CARGO_PKG_VERSION"),
      option_env!("BUILD_SHA")
        .or(option_env!("GITHUB_SHA"))
        .unwrap_or("dev")
    )
  }

  async fn boot(mode: StartMode, environment: &Environment, config: Config) -> Result<BootResult> {
    create_app::<Self, Migrator>(mode, environment, config).await
  }

  async fn initializers(ctx: &AppContext) -> Result<Vec<Box<dyn Initializer>>> {
    // Initialize app services with database connection
    initializers::init_services(ctx).map_err(|e| loco_rs::Error::string(&e))?;
    Ok(vec![])
  }

  fn routes(ctx: &AppContext) -> AppRoutes {
    AppRoutes::with_default_routes() // controller routes below
      .add_route(controllers::auth::routes(ctx))
      .add_route(controllers::patient::routes(ctx))
  }
  async fn connect_workers(ctx: &AppContext, queue: &Queue) -> Result<()> {
    queue.register(DownloadWorker::build(ctx)).await?;
    Ok(())
  }

  #[allow(unused_variables)]
  fn register_tasks(tasks: &mut Tasks) {
    // tasks-inject (do not remove)
  }
  async fn truncate(ctx: &AppContext) -> Result<()> {
    truncate_table(&ctx.db, users::Entity).await?;
    Ok(())
  }
  async fn seed(ctx: &AppContext, base: &Path) -> Result<()> {
    db::seed::<users::ActiveModel>(&ctx.db, &base.join("users.yaml").display().to_string()).await?;
    Ok(())
  }
}
