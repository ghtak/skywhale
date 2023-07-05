use std::io;
use axum::http::Uri;
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

    #[error("PathError {message} {location:?}")]
    PathError{
        message: String,
        location: Option<String>
    },

    #[error("Method Not Allowed")]
    MethodNotAllowed,

    #[error("Not Found{0:?}")]
    NotFound(Uri),

    #[error("Unhandled Error {0:?}")]
    UnhandledError(Box<dyn std::error::Error>),
}

impl IntoResponse for Error{
    fn into_response(self) -> Response {
        let (code, message) = match self {
            Error::IoError(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            Error::NotImplemented => (StatusCode::NOT_IMPLEMENTED, self.to_string()),
            Error::MethodNotAllowed => (StatusCode::METHOD_NOT_ALLOWED, self.to_string()),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        };
        let body = axum::Json(json!({ "error": message }));
        (code, body).into_response()
    }
}

