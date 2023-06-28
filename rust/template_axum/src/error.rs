use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde_json::json;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ErrorCode(u32);

impl ErrorCode {
    pub fn message(&self) -> Option<&'static str> {
        error_message_impl(self.0)
    }
}

macro_rules! error_codes {
    ( $( ($value:expr, $code:ident, $message:expr); )+ ) => {
        impl ErrorCode {
            $( pub const $code: ErrorCode = ErrorCode($value); )+
        }

        fn error_message_impl(value:u32) -> Option<&'static str> {
            match value {
                $( $value => Some($message), )+
                _ => None
            }
        }
    }
}

error_codes! {
    (1, INVALID_PARAMETER, "INVALID_PARAMETER");
}


impl IntoResponse for ErrorCode {
    fn into_response(self) -> Response {
        let body = Json(json!({
            "error": self.message()
        }));
        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}