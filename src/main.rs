mod handlers;
mod templates;


use axum::{Router, routing::get};
use tower_http::services::ServeDir;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let bind_address = SocketAddr::from(([127, 0, 0, 1], 3000));

    let app = Router::new()
        .route("/", get(handlers::home::home))
        .nest_service("/static", ServeDir::new("static"));
    let listener = tokio::net::TcpListener::bind(bind_address).await.unwrap();

    println!("The program is running on ip and port http://{}/", bind_address);

    axum::serve(listener, app).await.unwrap();
}
