use axum::{routing::get, Router};
use axum::http::StatusCode;
use dotenvy::dotenv;
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::{env, net::SocketAddr};
use tokio::net::TcpListener;
use tracing::{error, info};
use tracing_subscriber;
mod models;
mod utils;

#[derive(Clone)]
struct AppState {
    db: PgPool,
}

async fn health_check() -> &'static str {
    "Smart Scholar API is running 🚀"
}

async fn db_health(
    state: axum::extract::State<AppState>,
) -> (StatusCode, &'static str) {
    match sqlx::query("SELECT 1")
        .execute(&state.db)
        .await
    {
        Ok(_) => (StatusCode::OK, "Database connection OK ✅"),
        Err(_) => (
            StatusCode::SERVICE_UNAVAILABLE,
            "Database connection FAILED ❌",
        ),
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
        Ok(pool) => {
            info!("Connected to PostgreSQL");
            pool
        }
        Err(e) => {
            error!("Database connection failed at startup: {}", e);
            error!("Starting server without active DB connection");

            PgPoolOptions::new()
                .max_connections(5)
                .connect_lazy(&db_url)
                .expect("Failed to create lazy DB pool")
        }
    };

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
