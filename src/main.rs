mod handlers;
mod templates;
mod db;
mod config;

use std::net::SocketAddr;
use colored::Colorize;
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let pool = match db::connection::create_pool().await {
        Ok(pool) => pool,
        Err(e) => {
            eprintln!("{} {}", "❌ Failed to connect to the database:".red() ,e);
            std::process::exit(1);
        },
    };

    if let Err(e) = db::migrations::run_migrations(&pool).await {
        eprintln!("{}: {}", "❌ Failed to run migrations".red(), e);
        std::process::exit(1)
    };

    let bind_address = SocketAddr::from(([127, 0, 0, 1], config::server_port()));

    let app = handlers::setup_routes();
    let listener = tokio::net::TcpListener::bind(bind_address).await.unwrap();

    println!("The program is running on ip and port http://{}/", bind_address);

    axum::serve(listener, app).await.unwrap();
}
