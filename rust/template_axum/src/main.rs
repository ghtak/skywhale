use axum::{
    routing::get,
    Router
};
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let port = std::env::var("PORT").expect("PORT must be set.");

    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    let addr = format!("0.0.0.0:{}", port);
    // run it with hyper on localhost:3000
    axum::Server::bind(&(addr.parse().unwrap()))
        .serve(app.into_make_service())
        .await
        .unwrap();
}
