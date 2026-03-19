use crate::application::repository::user_repo_trait;
use crate::entity::user::RegisterUserRequest;
use crate::framework::postgres::persistence;
use async_trait;
use sqlx;

#[async_trait::async_trait]
impl user_repo_trait::UserRepo for persistence::PostgresPersistence {
    async fn register_user(&self, registered_user: &RegisterUserRequest) -> anyhow::Result<()> {
        println!("calling register_user");

        sqlx::query!(
            "INSERT INTO users (username, email) VALUES ($1, $2)",
            registered_user.username,
            registered_user.email,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
