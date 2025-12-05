use axum::{
    middleware,
    routing::{delete, get, post, put},
    Router,
};
use tower_http::{
    cors::CorsLayer,
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};
use crate::{app_state::AppState, controllers, middleware::auth::auth_middleware};

pub fn create_router(state: AppState) -> Router {
    // Public routes (no authentication required)
    let public_routes = Router::new()
        .route("/api/auth/register", post(controllers::auth::register))
        .route("/api/auth/login", post(controllers::auth::login))
        .route("/api/auth/forgot", post(controllers::auth::forgot))
        .route("/api/auth/reset", post(controllers::auth::reset));

    // Protected routes (require authentication)
    let protected_routes = Router::new()
        // Auth routes
        .route("/api/auth/me", get(controllers::auth::me))
        // Patient routes
        .route("/api/patient/create", post(controllers::patient::create))
        .route("/api/patient/{patient_id}", put(controllers::patient::update))
        .route("/api/patient/_search_by_ssn", get(controllers::patient::search_by_ssn))
        .route("/api/patient/_search", get(controllers::patient::search))
        .route("/api/patient/{patient_id}/_generate_invoice", post(controllers::patient::generate_invoice))
        // User routes
        .route("/api/user/_save_business_information", post(controllers::user::save_business_info))
        .route("/api/user/my_offices", get(controllers::user::my_offices))
        // Practitioner office routes
        .route("/api/practitioner_office/create", post(controllers::practitioner_office::create))
        .route("/api/practitioner_office/{office_id}", put(controllers::practitioner_office::update))
        .route("/api/practitioner_office/{office_id}", delete(controllers::practitioner_office::destroy))
        // Apply auth middleware to all protected routes
        .layer(middleware::from_fn_with_state(state.clone(), auth_middleware));

    // Combine all routes
    let app = Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        // Serve static files for frontend
        .fallback_service(
            ServeDir::new("frontend/dist")
                .fallback(ServeFile::new("frontend/dist/index.html"))
        )
        // HTTP request tracing middleware
        .layer(TraceLayer::new_for_http())
        // CORS middleware
        .layer(
            CorsLayer::new()
                .allow_origin(tower_http::cors::Any)
                .allow_methods(tower_http::cors::Any)
                .allow_headers(vec![
                    axum::http::header::AUTHORIZATION,
                    axum::http::header::CONTENT_TYPE,
                    axum::http::header::ACCEPT,
                ])
        )
        .with_state(state);

    app
}
