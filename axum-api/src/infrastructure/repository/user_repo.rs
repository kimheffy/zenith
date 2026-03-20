use crate::application::repository::user_repo_trait;
use crate::entity::user::RegisterUserRequest;
use crate::framework::postgres::persistence;
use async_trait;
use sqlx;

#[async_trait::async_trait]
impl user_repo_trait::UserRepo for persistence::PostgresPersistence {
    async fn register_user(
        &self,
        registered_user: &RegisterUserRequest,
        hashed_password: [u8; 32],
    ) -> anyhow::Result<()> {
        sqlx::query("INSERT INTO users (username, email, password_hash) VALUES ($1, $2, $3)")
            .bind(&registered_user.username)
            .bind(&registered_user.email)
            .bind(hashed_password)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
