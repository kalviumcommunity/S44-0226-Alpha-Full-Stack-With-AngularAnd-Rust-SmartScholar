use axum::{
    http::StatusCode,
    response::Response,
    middleware::Next,
};
use crate::utils::jwt::Claims;

pub async fn role_guard(
    claims: Claims,
    required_role: i32,
    next: Next,
) -> Result<Response, StatusCode> {

    if claims.role_id != required_role {
        return Err(StatusCode::FORBIDDEN);
    }

    Ok(next.run().await)
}