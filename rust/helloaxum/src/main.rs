use axum::Router;
use axum::routing::get;
use dotenv::dotenv;
use tracing::debug;

use crate::utils::init_tracing;

mod error;
mod utils;

#[tokio::main]
async fn main() {
    let _guard = init_tracing();

    dotenv().ok();

    let port = std::env::var("PORT").unwrap_or_else(|_| "8010".into());
    debug!("{}", port);

    let addr = format!("0.0.0.0:{}", port);
    let router_main = Router::new().route("/", get(|| async { "Hello Axum" }));

    axum::Server::bind(&(addr.parse().unwrap()))
        .serve(router_main.into_make_service())
        .await
        .unwrap();
}
