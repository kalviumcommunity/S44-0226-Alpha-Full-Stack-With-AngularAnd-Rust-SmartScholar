use bcrypt::{hash, verify, DEFAULT_COST};
use std::env;
use tracing::error;

pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    let cost = env::var("BCRYPT_COST")
        .ok()
        .and_then(|c| c.parse::<u32>().ok())
        .unwrap_or(DEFAULT_COST);

    hash(password, cost)
}

pub fn verify_password(password: &str, hash_value: &str) -> bool {
    match verify(password, hash_value) {
        Ok(valid) => valid,
        Err(e) => {
            error!("Password verification error: {}", e);
            false
        }
    }
}