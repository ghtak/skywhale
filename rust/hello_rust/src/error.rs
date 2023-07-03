use std::error::Error;
use std::fmt::{Debug, Display, Formatter, write};
use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ErrorCode{
    #[error("Io Error {0}")]
    IoError(#[from] io::Error),
    #[error("Unknown Error")]
    UnknownError,

    #[error("SrcToDst {src} {dst}")]
    SrcToDst{
        src: String,
        dst: String
    }
}

pub fn error_into() -> ErrorCode {
    io::Error::from(io::ErrorKind::Interrupted).into()
}

/*
#[derive(Debug)]
struct DynErrorTest{}

unsafe impl Send for DynErrorTest{}
unsafe impl Sync for DynErrorTest{}
impl std::error::Error for DynErrorTest{}

impl Display for DynErrorTest{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "DynErrorTest")
    }
}

fn dyn_error_into() -> ErrorCode {
    DynErrorTest{}.into() as Box<dyn std::error::Error>
}
*/
//
// use std::error::Error;
// use std::fmt::{Display, Formatter, write};
// use axum::http::StatusCode;
// use axum::Json;
// use axum::response::{IntoResponse, Response};
// use serde_json::{error, json};
//
// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
// pub struct ErrorCode(u32);
//
// impl Display for ErrorCode {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         return if let Some(msg) = error_to_string(self.0) {
//             write!(f, "{}", msg)
//         } else {
//             write!(f, "undefined")
//         }
//     }
// }
//
// macro_rules! error_codes {
//     ( $( ($value:expr, $code:ident, $message:expr); )+ ) => {
//         impl ErrorCode {
//             $( pub const $code: ErrorCode = ErrorCode($value); )+
//         }
//
//         fn error_to_string(value:u32) -> Option<&'static str> {
//             match value {
//                 $( $value => Some($message), )+
//                 _ => None
//             }
//         }
//     }
// }
//
// error_codes! {
//     (1, INVALID_PARAMETER, "INVALID_PARAMETER");
// }
//
//
// impl IntoResponse for ErrorCode {
//     fn into_response(self) -> Response {
//         let body = Json(json!({
//             "error": self.to_string()
//         }));
//         (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
//     }
// }