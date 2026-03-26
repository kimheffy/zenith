use crate::entity::error::AppError;

pub trait AuthenticationServiceTrait: Send + Sync {
    fn create_session(&self, email: &str, jwt_secret: &[u8]) -> anyhow::Result<String, AppError>;
}
