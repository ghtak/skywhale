use axum::Router;
use axum::routing::get;
use dotenv::dotenv;
use tracing::debug;
use crate::error::ErrorCode;

use crate::utils::init_tracing;

mod error;
mod utils;
mod routers;
mod dtos;

#[tokio::main]
async fn main() {
    let _guard = init_tracing();

    dotenv().ok();

    let port = std::env::var("PORT").unwrap_or_else(|_| "8010".into());
    debug!("{}", port);

    let addr = format!("0.0.0.0:{}", port);
    let router_main = Router::new()
        .route("/", get(hello_axum))//get(|| async { "Hello Axum" }));
        .nest( "/api/v1/user", routers::v1::user::router());

    axum::Server::bind(&(addr.parse().unwrap()))
        .serve(router_main.into_make_service())
        .await
        .unwrap();
}

async fn hello_axum() -> Result<&'static str, ErrorCode> {
    return Ok("Hello Axum")
}
