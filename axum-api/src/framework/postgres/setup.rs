use anyhow;
use sqlx::{PgPool, postgres};

pub async fn init_db() -> anyhow::Result<PgPool> {
    let database_url = std::env::var("DATABASE_URL").expect("failed to grab database_url from env");

    let pool = postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    sqlx::migrate!().run(&pool).await?;

    Ok(pool)
}
