use std::future::Future;
use axum::extract::FromRequestParts;
use axum::http::{Request, StatusCode};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use hyper::Body;
use tower_cookies::Cookies;
use tracing::log::debug;
use crate::error::Error;

pub async fn log_req_res(
    req: Request<Body>,
    next: Next<Body>) -> axum::response::Response
{
    let method = req.method().clone();
    let path = req.uri().clone();
    let (parts, body) = req.into_parts();

    let bytes = match hyper::body::to_bytes(body).await {
        Ok(bytes) => bytes,
        Err(err) => {
            return Error::UnhandledError(Box::try_from(err).unwrap()).into_response();
        }
    };
    if let Ok(body) = std::str::from_utf8(&bytes) {
        tracing::info!("{} {} {}", method, path, body);
    } else {
        tracing::info!("{} {}", method, path);
    }

    let res = next.run(Request::from_parts(parts, Body::from(bytes))).await;
    // let (parts, body) = res.into_parts();
    // let res = Response::from_parts(parts, body);
    tracing::info!("{} {} {}", method, path, res.status());
    res
}

pub async fn error_mapper(
    req: Request<Body>,
    next: Next<Body>) -> axum::response::Response
{
    let res = next.run(req).await;
    if res.status() == StatusCode::METHOD_NOT_ALLOWED {
        Error::MethodNotAllowed.into_response()
    } else {
        res
    }
}

pub async fn response_mapper(res: Response) -> Response{
    println!("->> {:<12} - response_mapper", "RES_MAPPER");
    if res.status() == StatusCode::METHOD_NOT_ALLOWED {
        Error::MethodNotAllowed.into_response()
    } else {
        res
    }
}