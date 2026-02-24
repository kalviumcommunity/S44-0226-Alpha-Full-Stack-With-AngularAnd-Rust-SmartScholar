use axum::{routing::get, Router};
use std::{env, net::SocketAddr};
use tokio::net::TcpListener;
use tracing::{error, info};
use tracing_subscriber;

async fn health_check() -> &'static str {
    "Smart Scholar API is running 🚀"
}

#[tokio::main]
async fn main() {
    // Initialize structured logging
    tracing_subscriber::fmt::init();

    // Read port from environment variable or default to 8080
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr: SocketAddr = format!("0.0.0.0:{}", port)
        .parse()
        .expect("Invalid address format");

    let app = Router::new().route("/health", get(health_check));

    let listener = match TcpListener::bind(addr).await {
        Ok(listener) => listener,
        Err(e) => {
            error!("Failed to bind to address: {}", e);
            std::process::exit(1);
        }
    };

    info!("Server running on http://{}", addr);

    if let Err(e) = axum::serve(listener, app).await {
        error!("Server error: {}", e);
        std::process::exit(1);
    }
}