use crate::framework::axum::app_state::AppState;
use crate::framework::axum::handler::user::user_routes;
// use axum::response::IntoResponse;
use axum::Router;
// use serde::{Deserialize, Serialize};
use std::sync::Arc;

// TODO: still not sure what this snippet is. looked at past history and it was deleted when rust
// jwt was completed
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
        .merge(user_routes())
        .with_state(Arc::new(shared_state))
}
