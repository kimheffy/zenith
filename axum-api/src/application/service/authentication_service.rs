use uuid::Uuid;

use crate::entity::error::AppError;

pub trait AuthenticationServiceTrait: Send + Sync {
    fn create_session(&self, id: Uuid, jwt_secret: &[u8]) -> anyhow::Result<String, AppError>;
}
