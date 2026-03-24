use std::env;

pub struct AppConfig {
    pub jwt_secret: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

        Self { jwt_secret }
    }
}
