use crate::application::use_case::user_use_case;
use crate::entity::error::AppError;
use crate::entity::user::RegisterUserRequest;
use crate::framework::axum::app_state::AppState;
use axum::Json;
use axum::extract::State;
use axum::{Router, routing::get, routing::post};
use std::sync::Arc;

pub fn router(shared_state: AppState) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/register", post(register_handler))
        .with_state(Arc::new(shared_state))
}

async fn root() -> &'static str {
    "Hello world!"
}

// handler could be renamed to 'controller'
async fn register_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RegisterUserRequest>,
) -> anyhow::Result<(), AppError> {
    // add in more complex validation
    if payload.username.is_empty() || payload.email.is_empty() || payload.password.is_empty() {
        return Err(AppError::InvalidInput);
    }

    let registered_user =
        RegisterUserRequest::new(payload.username, payload.email, payload.password);

    user_use_case::register_user_use_case(&registered_user, state.user_repo.as_ref()).await;

    Ok(())
}
