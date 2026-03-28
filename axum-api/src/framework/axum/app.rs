use crate::application::use_case::user_use_case;
use crate::entity::error::AppError;
use crate::entity::user::RegisterUserRequest;
use crate::framework::axum::app_state::AppState;
use axum::Json;
use axum::extract::State;
// use axum::response::IntoResponse;
use axum::{Router, routing::get, routing::post};
// use serde::{Deserialize, Serialize};
use std::sync::Arc;

// #[derive(Serialize, Deserialize)]
// struct AuthBody {
//     access_token: String,
//     token_type: String,
// }
//
// impl AuthBody {
//     fn new(access_token: String) -> Self {
//         Self {
//             access_token,
//             token_type: "Bearer".to_string(),
//         }
//     }
// }
//
// impl IntoResponse for AuthBody {
//     fn into_response(self) -> axum::response::Response {
//         Json(self).into_response()
//     }
// }
//
// #[derive(Serialize, Deserialize)]
// struct AuthorizeUserRequest {
//     email: String,
// }

pub fn router(shared_state: AppState) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/register", post(register_handler))
        // .route("/authorize", post(authorize_handler))
        .with_state(Arc::new(shared_state))
}

async fn root() -> &'static str {
    "Hello world!"
}

// handler could be renamed to 'controller'
async fn register_handler(
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
