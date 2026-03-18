use async_trait;
use sqlx;
use uuid::Uuid;

use crate::application::repository::user_repo_trait;
use crate::framework::postgres::persistence;

#[async_trait::async_trait]
impl user_repo_trait::UserRepo for persistence::PostgresPersistence {
    async fn register_user(&self) {
        println!("calling register_user");
        let uuid = Uuid::new_v4();
        sqlx::query!(
            "INSERT INTO users (id, username, email) VALUES ($1, $2, $3)",
            uuid,
            "heffy cuh testing clean",
            "clean.code@arch.com"
        )
        .execute(&self.pool)
        .await;
    }
}
