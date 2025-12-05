use migration::MigratorTrait;
use tokio::signal;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod app_state;
mod auth;
mod config;
mod controllers;
mod initializers;
mod middleware;
mod middlewares;
mod models;
mod router;
mod services;
mod validators;
mod views;
mod workers; // Keep for backward compatibility during transition

use app_state::AppState;
use config::Config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  // Load environment variables from .env.local
  dotenvy::from_filename(".env.local").ok();

  // Determine environment (development, production, or test)
  let environment = std::env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_string());

  // Load configuration from YAML files
  let config = Config::load(&environment).expect("Failed to load configuration");

  // Initialize logging
  setup_logging(&config.logger.level);

  tracing::info!(
    "Starting my_patients application (environment: {})",
    environment
  );

  // Connect to database
  let db = sea_orm::Database::connect(&config.database.url)
    .await
    .expect("Failed to connect to database");

  tracing::info!("Connected to database");

  // Run database migrations
  migration::Migrator::up(&db, None)
    .await
    .expect("Failed to run database migrations");

  tracing::info!("Database migrations completed");

  // Create worker channel for background jobs
  let (worker_tx, worker_rx) = workers::create_worker_channel();

  // Initialize application state
  let state = AppState::new(db.clone(), config.clone(), worker_tx);

  // Initialize global services (for backward compatibility with existing code)
  initializers::app_services::init_services(&db);

  // Start worker pool (4 workers)
  let worker_config = state.config.clone();
  let worker_db = db.clone();
  tokio::spawn(async move {
    workers::start_worker_pool(worker_rx, worker_db, worker_config, 4).await;
  });

  tracing::info!("Worker pool started with 4 workers");

  // Create Axum router with all routes
  let app = router::create_router(state.clone());

  // Start HTTP server
  let addr = format!("{}:{}", config.server.binding, config.server.port);
  let listener = tokio::net::TcpListener::bind(&addr)
    .await
    .expect(&format!("Failed to bind to address: {}", addr));

  tracing::info!(
    "Server listening on {}:{}",
    config.server.host,
    config.server.port
  );

  // Run server with graceful shutdown
  axum::serve(listener, app)
    .with_graceful_shutdown(shutdown_signal())
    .await
    .expect("Server error");

  Ok(())
}

fn setup_logging(level: &str) {
  tracing_subscriber::registry()
    .with(
      tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| format!("my_patients={},tower_http=debug", level).into()),
    )
    .with(tracing_subscriber::fmt::layer())
    .init();
}

async fn shutdown_signal() {
  let ctrl_c = async {
    signal::ctrl_c()
      .await
      .expect("Failed to install Ctrl+C handler");
  };

  #[cfg(unix)]
  let terminate = async {
    signal::unix::signal(signal::unix::SignalKind::terminate())
      .expect("Failed to install signal handler")
      .recv()
      .await;
  };

  #[cfg(not(unix))]
  let terminate = std::future::pending::<()>();

  tokio::select! {
      _ = ctrl_c => {},
      _ = terminate => {},
  }

  tracing::info!("Shutdown signal received, starting graceful shutdown");
}
