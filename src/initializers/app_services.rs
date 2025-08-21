use loco_rs::app::AppContext;
use std::sync::OnceLock;

// Application services to hold database connection and other shared resources
#[derive(Clone)]
pub struct AppServices {
  pub db: sea_orm::DatabaseConnection,
}

impl AppServices {
  pub fn new(ctx: &AppContext) -> Self {
    Self { db: ctx.db.clone() }
  }
}

// Global static variable to store services
static APP_SERVICES: OnceLock<AppServices> = OnceLock::new();

// Initialize services - should be called during app initialization
pub fn init_services(ctx: &AppContext) -> Result<(), String> {
  APP_SERVICES
    .set(AppServices::new(ctx))
    .map_err(|_| "Failed to initialize app services".to_string())
}

// Helper function to get services from anywhere in the application
pub fn get_services() -> &'static AppServices {
  APP_SERVICES.get().expect("App services not initialized")
}
