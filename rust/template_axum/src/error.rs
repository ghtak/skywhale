use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde_json::json;

pub enum ErrorCode{
    Unknown(&'static str),
}

impl IntoResponse for ErrorCode{
    fn into_response(self) -> Response {
        let message = match self {
            ErrorCode::Unknown(error_message) => {
                error_message
            }
        };
        let body = Json(json!({
            "error": message,
        }));
        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}