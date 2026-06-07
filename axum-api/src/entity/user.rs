use serde::Deserialize;

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
