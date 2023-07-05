use std::str::from_utf8;
use std::time::Duration;

use axum::{BoxError, handler::HandlerWithoutStateExt, http::StatusCode, middleware, Router, routing::get};
use axum::error_handling::HandleErrorLayer;
use axum::extract::MatchedPath;
use axum::http::{Method, Request, Response, response, Uri};
use axum::middleware::Next;
use axum::response::IntoResponse;
use dotenv::dotenv;
use hyper::Body;
use tower::buffer::BufferLayer;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use tracing::debug;

use crate::error::{Error, Result};
use crate::utils::init_tracing;

mod error;
mod utils;
mod routers;
mod dtos;
mod customext;

async fn handle_404() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "Not found")
}

async fn handle_error(err: BoxError) -> Error {
    Error::UnhandledError(err)
}

async fn handle_middleware(
    req: Request<Body>,
    next: Next<Body>) -> axum::response::Response
{
    let method = req.method().clone();
    let path = req.uri().clone();
    let (parts,body) = req.into_parts();
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
    let (parts,body) = res.into_parts();
    let res = Response::from_parts(parts, body);

    tracing::info!("{} {} {}", method, path, res.status());

    if res.status() == StatusCode::METHOD_NOT_ALLOWED {
        Error::MethodNotAllowed.into_response()
    } else {
        res
    }
}


#[tokio::main]
async fn main() {
    let _guard = init_tracing();

    dotenv().ok();

    let port = std::env::var("PORT").unwrap_or_else(|_| "8010".into());
    debug!("{}", port);

    let addr = format!("0.0.0.0:{}", port);

    let serve_dir = ServeDir::new("static")
        .not_found_service(
            (|uri: Uri| async move {
                Error::NotFound(uri)
            }).into_service());


    let router_main = Router::new()
        .route("/", get(hello_axum))
        .nest_service("/api/v1/user", routers::v1::user::router())
        .nest_service("/api/v1/login", routers::v1::login::router())
        .layer(middleware::from_fn(handle_middleware))
        .fallback_service(
            Router::new().nest_service("/static", serve_dir)
        );

    axum::Server::bind(&(addr.parse().unwrap()))
        .serve(router_main.into_make_service())
        .await
        .unwrap();
}

async fn hello_axum() -> Result<&'static str> {
    return Ok("Hello Axum");
}