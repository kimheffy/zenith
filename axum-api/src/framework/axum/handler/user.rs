use crate::application::use_case::user_use_case;
use crate::entity::error::AppError;
use crate::entity::user::RegisterUserRequest;
use crate::framework::axum::app_state::AppState;
use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};
use std::sync::Arc;

async fn register_user(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RegisterUserRequest>,
) -> anyhow::Result<String, AppError> {
    // add in more complex validation
    if payload.username.is_empty() || payload.email.is_empty() || payload.password.is_empty() {
        return Err(AppError::InvalidInput);
    }

    // we can either call user_repo.findEmail here in the handler
    // OR do that check inside use-case

    let registered_user =
        RegisterUserRequest::new(&payload.username, &payload.email, &payload.password);

    let created_user_id =
        user_use_case::register_user_use_case(&registered_user, state.user_repo.as_ref()).await?;

    // after we successfully created the user, we should return back a session
    let token = state
        .authentication_service
        .create_session(created_user_id, state.config.jwt_secret.as_bytes())?;

    Ok(token)
}

pub fn user_routes() -> Router<Arc<AppState>> {
    Router::new().route("/register", post(register_user))
}
