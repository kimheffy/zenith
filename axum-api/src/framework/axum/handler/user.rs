use crate::application::use_case::user_use_case;
use crate::entity::error::AppError;
use crate::entity::user::{RegisterUserRequest, SignUserInRequest};
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

    let registered_user =
        RegisterUserRequest::new(&payload.username, &payload.email, &payload.password);

    user_use_case::register_user_use_case(
        &registered_user,
        state.user_repo.as_ref(),
        state.authentication_service.as_ref(),
        &state.config,
    )
    .await
}

async fn sign_user_in(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<SignUserInRequest>,
) -> anyhow::Result<String, AppError> {
    if payload.email.is_empty() || payload.password.is_empty() {
        // TODO: 'InvalidInput' doesn't seem appropriate...
        return Err(AppError::InvalidInput);
    }

    let sign_user_in = SignUserInRequest::new(payload.email, payload.password);

    let token = user_use_case::sign_user_in_use_case(
        &sign_user_in,
        state.user_repo.as_ref(),
        state.authentication_service.as_ref(),
        state.config.as_ref(),
    )
    .await?;

    Ok(token)
}

pub fn user_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/register", post(register_user))
        .route("/sign_in", post(sign_user_in))
}
