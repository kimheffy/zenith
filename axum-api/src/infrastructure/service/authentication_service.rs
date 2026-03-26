use crate::{
    application::service::authentication_service::AuthenticationServiceTrait,
    entity::error::AppError,
};
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};

const TWO_WEEKS_IN_SECONDS: usize = 1_209_600;

pub struct AuthenticationService;

#[derive(Serialize, Deserialize)]
struct Claim {
    // issued at
    iat: usize,
    email: String,
    // expire at
    exp: usize,
}

impl AuthenticationServiceTrait for AuthenticationService {
    fn create_session(&self, email: &str, jwt_secret: &[u8]) -> anyhow::Result<String, AppError> {
        let now = chrono::Utc::now().timestamp() as usize;

        let claim = Claim {
            iat: now,
            exp: now + TWO_WEEKS_IN_SECONDS,
            email: email.to_string(),
        };

        let token = encode(
            &Header::default(),
            &claim,
            &EncodingKey::from_secret(jwt_secret),
        )
        .map_err(|_| AppError::JWTEncodeFailed)?;

        Ok(token)
    }
}
