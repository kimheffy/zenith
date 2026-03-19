use axum_api::framework::axum::app;
use axum_api::framework::axum::setup::init_app;
use dotenv::dotenv;

// TODO: think of a place for this type
type RepositoryResult<T> = Result<T, sqlx::Error>;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    // setup
    let shared_state = init_app().await?;

    // main
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;

    let app = app::router(shared_state);

    // main
    axum::serve(listener, app).await?;

    // main
    Ok(())
}
