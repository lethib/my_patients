use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use crate::{
    app_state::{AppState, CurrentUserExt},
    auth::jwt::JwtService,
    models::_entities::users,
};

pub async fn auth_middleware(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, AuthError> {
    let auth_header = request
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| {
            tracing::error!("Authorization header not found");
            AuthError::MissingToken
        })?;

    let token = auth_header.strip_prefix("Bearer ").ok_or_else(|| {
        tracing::error!("No Bearer token found in Authorization header");
        AuthError::InvalidToken
    })?;

    let jwt_service = JwtService::new(&state.config.jwt.secret);
    let claims = jwt_service
        .validate_token(token)
        .map_err(|e| {
            tracing::error!("JWT validation failed: {}", e);
            AuthError::InvalidToken
        })?;

    // Load user from database
    let user_result = users::Model::find_by_pid(&state.db, &claims.pid)
        .await
        .map_err(|e| {
            tracing::error!("Failed to load user from database: {:?}", e);
            AuthError::UserNotFound
        })?;

    // Insert user into request extensions
    request.extensions_mut().insert(CurrentUserExt(user_result.0, user_result.1));

    Ok(next.run(request).await)
}

#[derive(Debug)]
pub enum AuthError {
    MissingToken,
    InvalidToken,
    UserNotFound,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AuthError::MissingToken => (StatusCode::UNAUTHORIZED, "Missing authorization token"),
            AuthError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid or expired token"),
            AuthError::UserNotFound => (StatusCode::UNAUTHORIZED, "User not found"),
        };

        (status, message).into_response()
    }
}
