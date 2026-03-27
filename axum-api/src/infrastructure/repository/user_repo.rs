use crate::application::repository::user_repo_trait;
use crate::entity::error::AppError;
use crate::entity::user::RegisterUserRequest;
use crate::framework::postgres::persistence;
use async_trait;
use sqlx;
use uuid::Uuid;

#[async_trait::async_trait]
impl user_repo_trait::UserRepo for persistence::PostgresPersistence {
    async fn register_user(
        &self,
        registered_user: &RegisterUserRequest,
        hashed_password: &[u8],
    ) -> anyhow::Result<Uuid, AppError> {
        sqlx::query_scalar!(
            "INSERT INTO users (username, email, password_hash) VALUES ($1, $2, $3) RETURNING id",
            &registered_user.username,
            &registered_user.email,
            hashed_password
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|_| AppError::DatabaseOperationError)
    }
}
