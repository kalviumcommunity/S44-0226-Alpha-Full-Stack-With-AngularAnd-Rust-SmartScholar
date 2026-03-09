use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub full_name: String,
    pub email: String,
    pub password_hash: String,
    pub role_id: i32,
}