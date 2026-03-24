use crate::application::use_case::user_use_case;
use crate::entity::error::AppError;
use crate::entity::user::RegisterUserRequest;
use crate::framework::axum::app_state::AppState;
use anyhow::Context;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Json, debug_handler};
use axum::{Router, routing::get, routing::post};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Deserialize)]
struct Claim {
    email: String,
}

#[derive(Serialize, Deserialize)]
struct AuthBody {
    access_token: String,
    token_type: String,
}

impl AuthBody {
    fn new(access_token: String) -> Self {
        Self {
            access_token,
            token_type: "Bearer".to_string(),
        }
    }
}

impl IntoResponse for AuthBody {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}

pub fn router(shared_state: AppState) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/register", post(register_handler))
        .route("/authorize", get(authorize_handler))
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

    user_use_case::register_user_use_case(&registered_user, state.user_repo.as_ref()).await?;

    Ok(())
}

#[debug_handler]
async fn authorize_handler(
    State(state): State<Arc<AppState>>,
) -> anyhow::Result<AuthBody, AppError> {
    let claim = Claim {
        email: "test@test.com".to_string(),
    };
    let token = encode(
        &Header::default(),
        &claim,
        &EncodingKey::from_secret(state.config.jwt_secret.as_bytes()),
    )
    .unwrap();

    Ok(AuthBody::new(token))
}
