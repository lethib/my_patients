use std::sync::OnceLock;

// Application services to hold database connection and other shared resources
#[derive(Clone)]
pub struct AppServices {
  pub db: sea_orm::DatabaseConnection,
}

impl AppServices {
  pub fn new(db: &sea_orm::DatabaseConnection) -> Self {
    Self { db: db.clone() }
  }
}

// Global static variable to store services
static APP_SERVICES: OnceLock<AppServices> = OnceLock::new();

// Initialize services - should be called during app initialization
pub fn init_services(db: &sea_orm::DatabaseConnection) {
  let _ = APP_SERVICES.set(AppServices::new(db));
}

// Helper function to get services from anywhere in the application
pub fn get_services() -> &'static AppServices {
  APP_SERVICES.get().expect("App services not initialized")
}
