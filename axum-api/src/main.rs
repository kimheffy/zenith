use std::sync::Arc;

use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

type RepositoryResult<T> = Result<T, sqlx::Error>;

#[async_trait::async_trait]
trait UserRepo: Send + Sync {
    async fn register_user(&self, user: RegisterUser) -> RepositoryResult<User>;
}

#[derive(Clone)]
struct AppState {
    pool: Postgres,
    user_repo: Arc<dyn UserRepo>,
}

#[derive(Clone)]
struct Postgres {
    pub pool: sqlx::PgPool,
}

impl Postgres {
    async fn new(database_url: &str) -> anyhow::Result<Self> {
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await?;

        Ok(Self { pool })
    }
}

#[async_trait::async_trait]
impl UserRepo for Postgres {
    async fn register_user(&self, user: RegisterUser) -> RepositoryResult<User> {
        let id = Uuid::new_v4();

        sqlx::query!(
            "INSERT INTO users (id, username, email) VALUES ($1, $2, $3)",
            id,
            user.username,
            user.email
        )
        .execute(&self.pool)
        .await;

        let user = User {
            id,
            username: user.username,
            email: user.email,
        };

        Ok(user)
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let database_url = "postgres://postgres:postgres@localhost/zenith";

    let postgres = Postgres::new(&database_url).await?;

    let shared_state = AppState {
        pool: postgres.clone(),
        user_repo: Arc::new(postgres.clone()),
    };

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    let app = Router::new()
        .route("/", get(root))
        .route("/register", post(register_handler))
        .with_state(Arc::new(shared_state));

    axum::serve(listener, app).await?;

    Ok(())
}

#[derive(Deserialize)]
struct RegisterUser {
    username: String,
    email: String,
}

#[derive(Serialize)]
struct User {
    id: Uuid,
    username: String,
    email: String,
}

async fn root() -> &'static str {
    "Hello world!"
}

async fn register_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RegisterUser>,
) -> (StatusCode, Json<User>) {
    let user = state
        .user_repo
        .register_user(RegisterUser {
            username: payload.username,
            email: payload.email,
        })
        .await
        .expect("failed to create user");

    (StatusCode::CREATED, Json(user))
}
