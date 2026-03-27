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
            AppError::InvalidInput => (StatusCode::NOT_ACCEPTABLE, "Invalid input".to_string()),
            AppError::PasswordHashFailed => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Unexpected error".to_string(),
            ),
            AppError::DatabaseOperationError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Unexpected database error".to_string(),
            ),
            AppError::JWTEncodeFailed => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Unexpected encoding error".to_string(),
            ),
        };

        (status, AppJson(ErrorResponse { message })).into_response()

        // TODO: too advanced for me.. complete this later
        // if let Some(err) = err {
        //     response.extensions_mut().insert(Arc::new(err));
        // }
    }
}
