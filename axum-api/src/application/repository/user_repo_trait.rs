use crate::entity::error::AppError;
use crate::entity::user::{RegisterUserRequest, User};
use async_trait;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait UserRepo: Send + Sync {
    async fn register_user(
        &self,
        registered_user: &RegisterUserRequest,
        hashed_password: &String,
    ) -> anyhow::Result<Uuid, AppError>;

    async fn find_user_by_email(&self, email: &str) -> anyhow::Result<User, AppError>;
}
