use std::io;
use axum::Json;
use axum::response::{IntoResponse, Response};
use hyper::StatusCode;
use serde_json::json;
use thiserror::Error;


#[derive(Error, Debug)]
pub(crate) enum ErrorCode{
    #[error("io::Error {0:?}",)]
    IoError(#[from] io::Error),
}

impl IntoResponse for ErrorCode{
    fn into_response(self) -> Response {
        let (code, message) = match self {
            ErrorCode::IoError(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
        };
        let body = Json(json!({ "error": message }));
        (code, body).into_response()
    }
}