use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use chrono::{Utc, Duration};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,        // user id
    pub email: String,   // user email
    pub role_id: i32,    // role for RBAC
    pub iat: usize,      // issued at
    pub exp: usize,      // expiration
}

use jsonwebtoken::{encode, EncodingKey, Header};
use chrono::{Utc, Duration};
use std::env;

pub fn generate_jwt(
    user_id: i32,
    email: String,
    role_id: i32
) -> Result<String, jsonwebtoken::errors::Error> {

    let secret = env::var("JWT_SECRET")
        .expect("JWT_SECRET must be set");

    let now = Utc::now().timestamp() as usize;

    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .unwrap()
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id,
        email,
        role_id,
        iat: now,
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}