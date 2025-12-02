use crate::handlers;
use axum::{Router, routing::get};
use tower_http::services::ServeDir;

pub mod home;

pub fn setup_routes() -> Router {
    Router::new()
        .route("/", get(handlers::home::home))
        .route("/contact-info", get(handlers::home::contact_info))
        .route("/about-me", get(handlers::home::about_me))
        .nest_service("/static", ServeDir::new("static"))
}
