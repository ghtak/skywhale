use std::fmt::Debug;
use std::io;
use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ErrorCode{
    #[error("Io Error {0}")]
    IoError(#[from] io::Error),
    #[error("Unknown Error")]
    UnknownError,
}

impl IntoResponse for ErrorCode {
    fn into_response(self) -> Response {
        let (code, message) = match self {
            ErrorCode::IoError(io_error) =>
                ( StatusCode::INTERNAL_SERVER_ERROR,
                  format!("{:?}", io_error)),
            ErrorCode::UnknownError =>
                ( StatusCode::INTERNAL_SERVER_ERROR,
                  self.to_string())
        };
        let body = Json(json!({ "error": message }));
        (code, body).into_response()
    }
}