use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Application {
    pub id: i32,
    pub user_id: i32,
    pub scholarship_name: String,
    pub status: ApplicationStatus
    pub submitted_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateApplication {
    pub scholarship_name: String,
}

use sqlx::Type;

#[derive(Debug, Serialize, Deserialize, Type)]
#[sqlx(type_name = "application_status", rename_all = "UPPERCASE")]
pub enum ApplicationStatus {
    Pending,
    Verified,
    Approved,
    Rejected,
}