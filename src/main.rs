mod handlers;
mod templates;

use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    const BIND_ADDRESS: &str = "0.0.0.0:3000";
    let app = Router::new().route("/", get(handlers::home::home));
    let listener = tokio::net::TcpListener::bind(BIND_ADDRESS).await.unwrap();

    println!("The program is running on ip and port http://{}/", BIND_ADDRESS);

    axum::serve(listener, app).await.unwrap();
}
