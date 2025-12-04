mod handlers;
mod templates;
mod db;
mod config;

use std::net::SocketAddr;
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let bind_address = SocketAddr::from(([127, 0, 0, 1], config::server_port()));

    let app = handlers::setup_routes();
    let listener = tokio::net::TcpListener::bind(bind_address).await.unwrap();

    println!("The program is running on ip and port http://{}/", bind_address);

    axum::serve(listener, app).await.unwrap();
}
