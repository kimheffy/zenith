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
        sqlx::query_scalar(
            "INSERT INTO users (username, email, password_hash) VALUES ($1, $2, $3) RETURNING id",
        )
        .bind(&registered_user.username)
        .bind(&registered_user.email)
        .bind(hashed_password)
        .fetch_one(&self.pool)
        .await
        .map_err(|_| AppError::DatabaseOperationError)
    }
}

#[cfg(test)]
mod tests {
    use crate::application::repository::user_repo_trait::UserRepo;
    use crate::entity::error::AppError;
    use crate::entity::user::RegisterUserRequest;
    use crate::framework::postgres::persistence;
    use sqlx::PgPool;
    use uuid::Uuid;

    #[sqlx::test]
    async fn test_user_repo_register_user(pool: PgPool) -> sqlx::Result<(), AppError> {
        let in_memory_persistence = persistence::PostgresPersistence::new(pool.clone());

        let username = "test user 1";
        let email = "test@test.com";
        let password = "test1";

        let registered_user = RegisterUserRequest::new(username, email, password);

        let hashed_password = b"hello world!";

        let test_id = in_memory_persistence
            .register_user(&registered_user, hashed_password)
            .await?;

        let user: (Uuid, String, String) =
            sqlx::query_as("SELECT id, username, email FROM users WHERE email = $1")
                .bind(email)
                .fetch_one(&pool)
                .await
                .unwrap();

        assert_eq!(test_id, user.0);
        assert_eq!(username, user.1);
        assert_eq!(email, user.2);

        Ok(())
    }
}
