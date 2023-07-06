use std::io;

use axum::http::Uri;
use axum::response::{IntoResponse, Response};
use hyper::StatusCode;
use serde_json::json;
use thiserror;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(thiserror::Error, Debug, Clone)]
pub enum Error {
    #[error("io::Error {0}")]
    IoError(String),

    #[error("Not Implemented")]
    NotImplemented,

    #[error("Auth Failed, AuthToken Not Exist")]
    AuthTokenNotExist,

    #[error("JsonDataError {0}")]
    JsonDataError(String),
    #[error("JsonSyntaxError {0}")]
    JsonSyntaxError(String),
    #[error("MissingJsonContentType {0}")]
    MissingJsonContentType(String),
    #[error("JsonRejection {0}")]
    JsonRejection(String),

    #[error("PathError {message} {location:?}")]
    PathError {
        message: String,
        location: Option<String>,
    },

    #[error("Method Not Allowed")]
    MethodNotAllowed,

    #[error("Not Found")]
    NotFound,

    #[error("Unhandled Error {0}")]
    UnhandledError(String),
}

impl IntoResponse for Error {
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

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Error::IoError(
            format!("{value:?}")
        )
    }
}