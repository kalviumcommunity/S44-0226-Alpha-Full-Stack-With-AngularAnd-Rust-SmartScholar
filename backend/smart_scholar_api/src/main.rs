use axum::{
    routing::get,
    Router,
    response::IntoResponse,
};
use tokio::net::TcpListener;

async fn health_check() -> impl IntoResponse {
    "Smart Scholar API is running 🚀"
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/health", get(health_check));

    let listener = TcpListener::bind("127.0.0.1:8080")
        .await
        .expect("Failed to bind address");

    println!("Server running on http://127.0.0.1:8080");

    axum::serve(listener, app)
        .await
        .expect("Server error");
}