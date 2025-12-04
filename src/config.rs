use std::env;

pub fn server_port() -> u16 {
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string()).parse::<u16>().expect("Failed to parse into u16");
    port
}
