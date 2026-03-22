use crate::entity::error::AppError;
use crate::framework::axum::json::AppJson;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::Serialize;

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        #[derive(Serialize)]
        struct ErrorResponse {
            message: String,
        }

        let (status, message) = match &self {
            AppError::InvalidInput => (StatusCode::NOT_ACCEPTABLE, "invalid input".to_string()),
        };

        (status, AppJson(ErrorResponse { message })).into_response()

        // TODO: too advanced for me.. complete this later
        // if let Some(err) = err {
        //     response.extensions_mut().insert(Arc::new(err));
        // }
    }
}
