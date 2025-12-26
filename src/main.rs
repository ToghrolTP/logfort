mod config;
mod db;
mod error;
mod handlers;
mod models;
mod templates;
mod utils;

use colored::Colorize;
use db::connection::create_pool;
use dotenvy::dotenv;
use std::{
    env::args,
    io::{self, Write, stdout},
    net::SocketAddr,
};
use utils::password::hash_password;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let arguments = args();

    for arg in arguments {
        if arg == "create-admin" {
            create_admin_cli().await;
        }
    }

    let pool = match create_pool().await {
        Ok(pool) => pool,
        Err(e) => {
            eprintln!("{} {}", "❌ Failed to connect to the database:".red(), e);
            std::process::exit(1);
        }
    };

    if let Err(e) = db::migrations::run_migrations(&pool).await {
        eprintln!("{}: {}", "❌ Failed to run migrations".red(), e);
        std::process::exit(1)
    };

    let bind_address = SocketAddr::from(([127, 0, 0, 1], config::server_port()));

    let app = handlers::setup_routes(pool);
    let listener = tokio::net::TcpListener::bind(bind_address).await.unwrap();

    println!(
        "The program is running on ip and port http://{}/",
        bind_address
    );

    axum::serve(listener, app).await.unwrap();
}

async fn create_admin_cli() {
    print!("Enter username: ");
    stdout().flush().unwrap();
    let mut username = String::new();
    io::stdin()
        .read_line(&mut username)
        .expect("Failed to read line!");
    let username = username.trim();

    print!("Enter display name: ");
    stdout().flush().unwrap();
    let mut display_name = String::new();
    io::stdin()
        .read_line(&mut display_name)
        .expect("Failed to read line!");
    let display_name = display_name.trim();

    print!("Enter email: ");
    stdout().flush().unwrap();
    let mut email = String::new();
    io::stdin()
        .read_line(&mut email)
        .expect("Failed to read line!");
    let email = email.trim();

    print!("Enter password: ");
    stdout().flush().unwrap();
    let mut password = String::new();
    io::stdin()
        .read_line(&mut password)
        .expect("Failed to read line!");
    let password = password.trim();

    // Generate hash
    println!("Generating password hash...");
    let password_hash = match hash_password(password) {
        Ok(hash) => hash,
        Err(e) => {
            eprintln!("X Error hashing password: {}", e);
            return;
        }
    };

    // Connect to database
    let pool = match create_pool().await {
        Ok(pool) => pool,
        Err(e) => {
            eprintln!("❌ Failed to connect to database: {}", e);
            return;
        }
    };

    let result = sqlx::query(
        r#"
INSERT INTO users ( username, display_name, email, password_hash, role)
VALUES ( ?, ?, ?, ?, "admin"); "#,
    )
    .bind(&username)
    .bind(&display_name)
    .bind(&email)
    .bind(&password_hash)
    .execute(&pool)
    .await;

    match result {
        Ok(_) => {
            println!("\n✅ Admin user created successfully!");
            println!("Username: {}", username);
            println!("Display Name: {}", display_name);
            println!("Role: admin");
            println!("\nYou can now login with these credentials.");
        }
        Err(e) => {
            eprintln!("\n❌ Error creating admin user: {}", e);
        }
    }
}
