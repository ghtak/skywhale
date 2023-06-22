use dotenv::dotenv;
use std::env;

use axum::{routing::get, Router};

use log::info;
mod dtos;
mod routers;

#[tokio::main]
async fn main() {
    dotenv().ok(); //dotenv::from_filename(".env.local").ok();
    env_logger::init();
    let app = Router::new()
        .nest("/api/v1/sample", routers::v1::sample::router())
        .route("/", get(|| async { "Hello, Axum" }));

    let addr = format!("0.0.0.0:{}", env::var("PORT").unwrap());
    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
