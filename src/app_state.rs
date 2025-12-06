use crate::config::Config;
use axum::extract::FromRequestParts;
use axum::http::{request::Parts, StatusCode};
use sea_orm::DatabaseConnection;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
  pub db: DatabaseConnection,
  pub config: Arc<Config>,
  pub worker_transmitter: tokio::sync::mpsc::Sender<WorkerJob>,
}

impl AppState {
  pub fn new(
    db: DatabaseConnection,
    config: Config,
    worker_transmitter: tokio::sync::mpsc::Sender<WorkerJob>,
  ) -> Self {
    Self {
      db,
      config: Arc::new(config),
      worker_transmitter,
    }
  }
}

#[derive(Clone)]
pub struct CurrentUserExt(
  pub crate::models::_entities::users::Model,
  pub Option<crate::models::_entities::user_business_informations::Model>,
);

// Custom extractor for CurrentUserExt to make it easy to use in handlers
impl<S> FromRequestParts<S> for CurrentUserExt
where
  S: Send + Sync,
{
  type Rejection = (StatusCode, &'static str);

  async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
    parts.extensions.get::<CurrentUserExt>().cloned().ok_or((
      StatusCode::INTERNAL_SERVER_ERROR,
      "User not found in request",
    ))
  }
}

// Worker job enum for all background tasks
#[derive(Debug, Clone)]
pub enum WorkerJob {
  Email(crate::workers::mailer::args::EmailArgs),
}
