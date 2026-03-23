use axum::{
    extract::State,
    Json,
    http::StatusCode,
};
use crate::AppState;
use crate::models::application::{CreateApplication, ApplicationStatus};

pub async fn submit_application(
    State(state): State<AppState>,
    Json(payload): Json<CreateApplication>,
) -> Result<StatusCode, StatusCode> {

    let result = sqlx::query!(
        r#"
        INSERT INTO applications (user_id, scholarship_name, status)
        VALUES ($1, $2, $3)
        "#,
        1, // TODO: replace with JWT user_id
        payload.scholarship_name,
        ApplicationStatus::Pending as _
    )
    .execute(&state.db)
    .await;

    match result {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(err) => {
            use tracing::error;
            error!("DB Error: {:?}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}