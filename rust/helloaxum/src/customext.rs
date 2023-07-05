use std::convert::Infallible;

use axum::async_trait;
use axum::body::HttpBody;
use axum::extract::{FromRequest, FromRequestParts, MatchedPath};
use axum::extract::path::ErrorKind;
use axum::extract::rejection::{JsonRejection, PathRejection};
use axum::handler::Handler;
use axum::http::Request;
use axum::http::request::Parts;
use axum::RequestPartsExt;
use axum::routing::MethodRouter;
use serde::de::DeserializeOwned;

use crate::error::Error;

pub struct Json<T>(pub T);

#[async_trait]
impl<S, B, T> FromRequest<S, B> for Json<T>
    where
        axum::Json<T>: FromRequest<S, B, Rejection=JsonRejection>,
        S: Send + Sync,
        B: Send + 'static,
{
    type Rejection = Error;

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let (mut parts, body) = req.into_parts();

        let path = parts
            .extract::<MatchedPath>()
            .await
            .map(|path| path.as_str().to_owned())
            .ok();

        let req = Request::from_parts(parts, body);

        match axum::Json::<T>::from_request(req, state).await {
            Ok(value) => Ok(Self(value.0)),
            // convert the error from `axum::Json` into whatever we want
            Err(rejection) => {
                let err = match rejection {
                    JsonRejection::JsonDataError(_) => Error::JsonDataError(
                        format!("{:?} {}", path, rejection.to_string())),
                    JsonRejection::JsonSyntaxError(_) => Error::JsonSyntaxError(
                        format!("{:?} {}", path, rejection.to_string())),
                    JsonRejection::MissingJsonContentType(_) => Error::MissingJsonContentType(
                        format!("{:?} {}", path, rejection.to_string())),
                    _ => Error::JsonRejection(rejection.to_string()),
                };
                Err(err)
            }
        }
    }
}

pub struct Path<T>(pub T);


#[async_trait]
impl<S, T> FromRequestParts<S> for Path<T>
    where
    // these trait bounds are copied from `impl FromRequest for axum::extract::path::Path`
        T: DeserializeOwned + Send,
        S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        match axum::extract::Path::<T>::from_request_parts(parts, state).await {
            Ok(value) => Ok(Self(value.0)),
            Err(rejection) => {
                let err = match rejection {
                    PathRejection::FailedToDeserializePathParams(inner) => {
                        let kind = inner.into_kind();
                        let inner_err = match &kind {
                            ErrorKind::WrongNumberOfParameters { .. } => Error::PathError {
                                message: kind.to_string(),
                                location: None,
                            },

                            ErrorKind::ParseErrorAtKey { key, .. } => Error::PathError {
                                message: kind.to_string(),
                                location: Some(key.clone()),
                            },

                            ErrorKind::ParseErrorAtIndex { index, .. } => Error::PathError {
                                message: kind.to_string(),
                                location: Some(index.to_string()),
                            },

                            ErrorKind::ParseError { .. } => Error::PathError {
                                message: kind.to_string(),
                                location: None,
                            },

                            ErrorKind::InvalidUtf8InPathParam { key } => Error::PathError {
                                message: kind.to_string(),
                                location: Some(key.clone()),
                            },

                            ErrorKind::UnsupportedType { .. } => {
                                // this error is caused by the programmer using an unsupported type
                                // (such as nested maps) so respond with `500` instead
                                Error::PathError {
                                    message: kind.to_string(),
                                    location: None,
                                }
                            }

                            ErrorKind::Message(msg) => Error::PathError {
                                message: msg.clone(),
                                location: None,
                            },

                            _ => Error::PathError {
                                message: format!("Unhandled deserialization error: {}", kind),
                                location: None,
                            },
                        };
                        inner_err
                    }
                    PathRejection::MissingPathParams(error) =>
                        Error::PathError {
                            message: error.to_string(),
                            location: None,
                        },

                    _ =>
                        Error::PathError {
                            message: format!("Unhandled path rejection: {}", rejection),
                            location: None,
                        },
                };
                Err(err)
            }
        }
    }
}

pub trait FallbackCustomMethodNotAllowed<S, B> {
    fn wrap(self) -> MethodRouter<S, B, Infallible>;
}

impl<S, B> FallbackCustomMethodNotAllowed<S, B> for MethodRouter<S, B, Infallible>
    where
        B: HttpBody + Send + 'static,
        S: Clone + Send + Sync + 'static
{
    fn wrap(self) -> MethodRouter<S, B, Infallible> {
        let mut this = self;
        this.fallback(|| async {
            Error::MethodNotAllowed
        })
    }
}

// pub fn router() -> Router {
//     Router::new()
//         .route("/",
//                get(users).post(create_user).wrap())
//         .route("/:id", get(user_detail))
//         .route("/path_test/:a_id/:b_id", get(path_test))
// }

// pub fn wrap<S, B>(r: MethodRouter<S, B, Infallible>) -> MethodRouter<S, B, Infallible>
// where
//     B: HttpBody + Send + 'static,
//     S: Clone + Send + Sync + 'static
// {
//     r.fallback(
//         || async {
//            Error::MethodNotAllowed
//        }
//     )
// }


// macro_rules! custom_fallback_method_router{
//     (
//         $method_name:ident
//     ) => {
//         pub fn $method_name<H, T, S, B>(handler: H) -> MethodRouter<S, B, Infallible>
//                 where
//                     H: Handler<T, S, B>,
//                     B: HttpBody + Send + 'static,
//                     T: 'static,
//                     S: Clone + Send + Sync + 'static
//         {
//             axum::routing::$method_name(handler).fallback(
//                 || async {
//                    Error::MethodNotAllowed
//                }
//             )
//         }
//     }
// }
//
// custom_fallback_method_router!(get);
// custom_fallback_method_router!(post);

/*
pub fn post<H, T, S, B>(handler: H) -> MethodRouter<S, B, Infallible>
        where
            H: Handler<T, S, B>,
            B: HttpBody + Send + 'static,
            T: 'static,
            S: Clone + Send + Sync + 'static
{
    axum::routing::post(handler).fallback(
        || async {
           Error::MethodNotAllowed
       }
    )
}*/