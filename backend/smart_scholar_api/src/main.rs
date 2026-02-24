use axum::{routing::get, Router};
use dotenvy::dotenv;
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::{env, net::SocketAddr};
use tokio::net::TcpListener;
use tracing::{error, info};
use tracing_subscriber;

#[derive(Clone)]
struct AppState {
    db: PgPool,
}

async fn health_check() -> &'static str {
    "Smart Scholar API is running 🚀"
}

async fn db_health(state: axum::extract::State<AppState>) -> &'static str {
    match sqlx::query("SELECT 1")
        .execute(&state.db)
        .await
    {
        Ok(_) => "Database connection OK ✅",
        Err(_) => "Database connection FAILED ❌",
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dotenv().ok();

    let db_url =
        env::var("DATABASE_URL").expect("DATABASE_URL not set in .env file");

    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr: SocketAddr = format!("0.0.0.0:{}", port)
        .parse()
        .expect("Invalid address");

    // Create connection pool
    let pool = match PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
    {
        Ok(pool) => pool,
        Err(e) => {
            error!("Failed to connect to database: {}", e);
            std::process::exit(1);
        }
    };

    info!("Connected to PostgreSQL");

    let state = AppState { db: pool };

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/health/db", get(db_health))
        .with_state(state);

    let listener = match TcpListener::bind(addr).await {
        Ok(listener) => listener,
        Err(e) => {
            error!("Failed to bind: {}", e);
            std::process::exit(1);
        }
    };

    info!("Server running on http://{}", addr);

    if let Err(e) = axum::serve(listener, app).await {
        error!("Server error: {}", e);
        std::process::exit(1);
    }
}