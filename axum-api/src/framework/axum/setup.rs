use crate::framework::axum::app_state::AppState;
use crate::framework::axum::config::AppConfig;
use crate::framework::postgres::postgres_persistence;
use std::sync::Arc;

pub async fn init_app() -> anyhow::Result<AppState> {
    // TODO: load in the config
    let config = AppConfig::from_env();

    // init database connection
    let postgres_arc = Arc::new(postgres_persistence().await?);

    // <dependency injections>
    let shared_state = AppState {
        config: Arc::new(config),
        user_repo: postgres_arc.clone(),
    };

    // return a shared app state
    Ok(shared_state)
}
