use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize)]
#[allow(dead_code)]
struct User {
    id: Uuid,
    username: String,
    email: String,
    // NOTE: add password_hash back when implementing jwt
    password_hashed: String,
    created_at: u64,
}

#[derive(std::fmt::Debug, Deserialize)]
pub struct RegisterUserRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

impl RegisterUserRequest {
    pub fn new(username: &str, email: &str, password: &str) -> Self {
        Self {
            username: username.to_string(),
            email: email.to_string(),
            password: password.to_string(),
        }
    }
}
