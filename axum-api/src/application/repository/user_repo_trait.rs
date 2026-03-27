use crate::entity::error::AppError;
use crate::entity::user::RegisterUserRequest;
use async_trait;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait UserRepo: Send + Sync {
    async fn register_user(
        &self,
        registered_user: &RegisterUserRequest,
        hashed_password: &[u8],
    ) -> anyhow::Result<Uuid, AppError>;
}
