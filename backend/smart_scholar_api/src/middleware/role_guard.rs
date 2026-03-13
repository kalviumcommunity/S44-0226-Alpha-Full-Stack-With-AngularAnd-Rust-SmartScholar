use axum::{
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use crate::utils::jwt::Claims;

pub fn role_guard(
    required_roles: Vec<i32>,
) -> impl Fn(Claims, Next) -> std::pin::Pin<
    Box<dyn std::future::Future<Output = Result<Response, StatusCode>> + Send>
> + Clone {

    move |claims: Claims, next: Next| {
        let roles = required_roles.clone();

        Box::pin(async move {
            if roles.contains(&claims.role_id) {
                Ok(next.run().await)
            } else {
                Err(StatusCode::FORBIDDEN)
            }
        })
    }
}