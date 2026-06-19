use axum::{extract::MatchedPath, http::Request};
use axum_api::framework::axum::app;
use axum_api::framework::axum::setup::init_app;
use axum_api::framework::tracing::init_tracing;
use dotenv::dotenv;
use tower_http::trace::TraceLayer;
use tracing::info_span;

// TODO: think of a place for this type
#[allow(dead_code)]
type RepositoryResult<T> = Result<T, sqlx::Error>;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // read env
    dotenv().ok();

    // initialize tracing
    init_tracing();

    // init app state
    let shared_state = init_app().await?;

    // setup listener to port
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    tracing::debug!("listening on {}", listener.local_addr().unwrap());

    // load app with shared_state and layer in tracer
    let app = app::router(shared_state).layer(TraceLayer::new_for_http().make_span_with(
        |request: &Request<_>| {
            let matched_path = request
                .extensions()
                .get::<MatchedPath>()
                .map(MatchedPath::as_str);

            info_span!("http_request", method = ?request.method(), matched_path, some_other_field = tracing::field::Empty)
        },
    ));

    // serve the app to listener
    axum::serve(listener, app).await?;

    Ok(())
}
