use crate::entity::user::RegisterUserRequest;
use async_trait;

#[async_trait::async_trait]
pub trait UserRepo: Send + Sync {
    async fn register_user(
        &self,
        registered_user: &RegisterUserRequest,
        hashed_password: [u8; 32],
    ) -> anyhow::Result<()>;
}
