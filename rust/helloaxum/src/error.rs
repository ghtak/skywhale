use std::io;
use axum::response::{IntoResponse, Response};
use hyper::StatusCode;
use serde_json::json;
use thiserror::Error;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error{
    #[error("io::Error {0:?}",)]
    IoError(#[from] io::Error),

    #[error("Not Implemented")]
    NotImplemented,

    #[error("JsonDataError {0}")]
    JsonDataError(String),
    #[error("JsonSyntaxError {0}")]
    JsonSyntaxError(String),
    #[error("MissingJsonContentType {0}")]
    MissingJsonContentType(String),
    #[error("JsonRejection {0}")]
    JsonRejection(String),
}

impl IntoResponse for Error{
    fn into_response(self) -> Response {
        let (code, message) = match self {
            Error::IoError(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            Error::NotImplemented => (StatusCode::NOT_IMPLEMENTED, self.to_string()),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        };
        let body = axum::Json(json!({ "error": message }));
        (code, body).into_response()
    }
}

