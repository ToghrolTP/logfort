use crate::handlers;
use axum::{Router, routing::get};
use sqlx::{MySql, Pool};
use tower_http::services::ServeDir;
use tower_http::compression::CompressionLayer;

pub mod home;

pub fn setup_routes(pool: Pool<MySql>) -> Router {
    Router::new()
        .route("/", get(handlers::home::home))
        .route("/contact-info", get(handlers::home::contact_info))
        .route("/about-me", get(handlers::home::about_me))
        .route("/blog", get(handlers::home::blog))
        .with_state(pool)
        .nest_service("/static", ServeDir::new("static"))
        .layer(CompressionLayer::new())
}
