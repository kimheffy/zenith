use crate::{
    application::service::authentication_service::AuthenticationServiceTrait,
    entity::error::AppError,
};
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

const TWO_WEEKS_IN_SECONDS: usize = 1_209_600;

pub struct AuthenticationService;

#[derive(Serialize, Deserialize)]
struct Claim {
    // issued at
    iat: usize,
    // expire at
    exp: usize,
    user_id: String,
}

impl AuthenticationServiceTrait for AuthenticationService {
    fn create_session(&self, id: Uuid, jwt_secret: &[u8]) -> anyhow::Result<String, AppError> {
        let now = chrono::Utc::now().timestamp() as usize;

        let claim = Claim {
            iat: now,
            exp: now + TWO_WEEKS_IN_SECONDS,
            user_id: id.to_string(),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::service::authentication_service::AuthenticationServiceTrait;
    use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode};
    use uuid::Uuid;

    fn token_decode(token: &str, secret: &[u8]) -> anyhow::Result<Claim, String> {
        match decode::<Claim>(
            &token,
            &DecodingKey::from_secret(secret),
            &Validation::new(Algorithm::HS256),
        ) {
            Ok(token) => Ok(token.claims),
            Err(_) => Err("decoding error".to_string()),
        }
    }

    #[test]
    fn test_authentication_service_create_session() {
        let user_a_id = Uuid::new_v4();
        let mock_jwt_secret = b"test_mock_secret_for_create_session";

        let mock_token = AuthenticationService.create_session(user_a_id, mock_jwt_secret);

        assert!(mock_token.is_ok());

        let mock_token = mock_token.unwrap();
        let is_valid_jwt_token = mock_token.chars().filter(|c| c == &'.').count() == 2;
        assert!(is_valid_jwt_token);

        let mock_claims = token_decode(&mock_token, mock_jwt_secret).unwrap();
        assert_eq!(mock_claims.user_id, user_a_id.to_string());
    }

    #[test]
    fn test_authentication_service_no_identical_tokens() {
        let user_a = Uuid::new_v4();
        let user_b = Uuid::new_v4();
        let mock_jwt_secret = b"test_mock_secret_for_no_identical_tokens";

        let mock_token_a = AuthenticationService.create_session(user_a, mock_jwt_secret);
        let mock_token_b = AuthenticationService.create_session(user_b, mock_jwt_secret);

        assert!(mock_token_a.is_ok());
        assert!(mock_token_b.is_ok());

        let mock_token_a = mock_token_a.unwrap();
        let mock_token_b = mock_token_b.unwrap();

        assert_ne!(mock_token_a, mock_token_b);
    }
}
