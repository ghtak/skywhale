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
mod middlewares;

#[tokio::main]
async fn main() {
    let _guard = init_tracing();

    dotenv().ok();

    let port = std::env::var("PORT").unwrap_or_else(|_| "8010".into());
    debug!("{}", port);

    let addr = format!("0.0.0.0:{}", port);

    let serve_dir = ServeDir::new("static")
        .not_found_service((|uri: Uri| async move { Error::NotFound }).into_service());

    let router_main = Router::new()
        .route("/", get(hello_axum))
        .merge(routers::v1::user::router())
        .merge(routers::v1::login::router())
        .layer(middleware::from_fn(middlewares::log_req_res))
        .layer(middleware::map_response(middlewares::response_mapper)) //.layer(middleware::from_fn(middlewares::error_mapper))
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