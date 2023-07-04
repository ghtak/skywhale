use axum::{
    handler::HandlerWithoutStateExt,
    http::StatusCode,
    Router,
    routing::get};
use dotenv::dotenv;
use tower_http::services::ServeDir;
use tracing::debug;

use crate::utils::init_tracing;
use crate::error::Result;

mod error;
mod utils;
mod routers;
mod dtos;

async fn handle_404() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "Not found")
}

#[tokio::main]
async fn main() {
    let _guard = init_tracing();

    dotenv().ok();

    let port = std::env::var("PORT").unwrap_or_else(|_| "8010".into());
    debug!("{}", port);

    let addr = format!("0.0.0.0:{}", port);

    let service = handle_404.into_service();
    let serve_dir = ServeDir::new("static").not_found_service(service);
    let router_main = Router::new()
        .route("/", get(hello_axum))//get(|| async { "Hello Axum" }));
        .nest("/api/v1/user", routers::v1::user::router())
        .nest("/api/v1/login", routers::v1::login::router())
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