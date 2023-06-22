extern crate dotenv;

use dotenv::dotenv;
use std::env;

use axum::{
    routing::get,
    Router,
};


#[tokio::main]
async fn main() {
    dotenv().ok();
    //dotenv::from_filename(".env.local").ok();

    if let Ok(port) = env::var("PORT") {
        let app = Router::new()
            .route("/", get(|| async { "Hello, Axum" }));

        axum::Server::bind(& format!("0.0.0.0:{}", port).parse().unwrap())
            .serve(app.into_make_service())
            .await
            .unwrap();
    }
}
