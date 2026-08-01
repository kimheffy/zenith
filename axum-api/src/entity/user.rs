use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(sqlx::FromRow, Serialize, sqlx::Type)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
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

#[derive(std::fmt::Debug, Deserialize)]
pub struct SignUserInRequest {
    pub email: String,
    pub password: String,
}

impl SignUserInRequest {
    pub fn new(email: String, password: String) -> Self {
        Self { email, password }
    }
}
