use std::str::from_utf8;
use std::time::Duration;

use axum::{BoxError, handler::HandlerWithoutStateExt, http::StatusCode, Router, routing::get};
use axum::error_handling::HandleErrorLayer;
use axum::extract::MatchedPath;
use axum::http::{Method, Request, Response, response, Uri};
use axum::middleware::Next;
use axum::response::IntoResponse;
use dotenv::dotenv;
use hyper::Body;
use tower::buffer::BufferLayer;
use tower::ServiceBuilder;
use tower_cookies::CookieManagerLayer;
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
mod middleware;

#[tokio::main]
async fn main() {
    let _guard = init_tracing();

    dotenv().ok();

    let port = std::env::var("PORT").unwrap_or_else(|_| "8010".into());
    debug!("{}", port);

    let addr = format!("0.0.0.0:{}", port);

    let serve_dir = ServeDir::new("static")
        .not_found_service((|uri: Uri| async move { Error::NotFound }).into_service());

    let api_routers = routers::v1::user::router()
        .route_layer(axum::middleware::from_fn(
            crate::middleware::auth::cookie_auth));

    let router_main = Router::new()
        .route("/", get(hello_axum))
        .merge(routers::v1::login::router())
        .nest("/api", api_routers)
        .layer(axum::middleware::from_fn(middlewares::log_req_res))
        .layer(axum::middleware::map_response(middlewares::response_mapper)) //.layer(middleware::from_fn(middlewares::error_mapper))
        .route_layer( axum::middleware::from_fn(
            crate::middleware::auth::session_resolver))
        .layer(CookieManagerLayer::new())
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