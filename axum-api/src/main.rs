use axum::{extract::MatchedPath, http::Request};
use axum_api::framework::axum::app;
use axum_api::framework::axum::setup::init_app;
use dotenv::dotenv;
use tower_http::trace::TraceLayer;
use tracing::info_span;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// TODO: think of a place for this type
type RepositoryResult<T> = Result<T, sqlx::Error>;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!(
                    "{}=debug,tower_http=debug,axum::rejection=true",
                    env!("CARGO_CRATE_NAME")
                )
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // setup
    let shared_state = init_app().await?;

    // main
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    tracing::debug!("listening on {}", listener.local_addr().unwrap());

    let app = app::router(shared_state).layer(TraceLayer::new_for_http().make_span_with(
        |request: &Request<_>| {
            let matched_path = request
                .extensions()
                .get::<MatchedPath>()
                .map(MatchedPath::as_str);

            info_span!("http_request", method = ?request.method(), matched_path, some_other_field = tracing::field::Empty)
        },
    ));

    // main
    axum::serve(listener, app).await?;

    // main
    Ok(())
}
