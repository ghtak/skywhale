mod error;

use axum::{
    routing::get,
    Router
};
use dotenv::dotenv;
use crate::error::ErrorCode;
use rand::Rng;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let port = std::env::var("PORT").expect("PORT must be set.");

    let main_router = Router::new().route("/", get(get_impl));

    let addr = format!("0.0.0.0:{}", port);
    // run it with hyper on localhost:3000
    axum::Server::bind(&(addr.parse().unwrap()))
        .serve(main_router.into_make_service())
        .await
        .unwrap();
}

async fn get_impl() -> Result<&'static str, ErrorCode> {
    let mut rng = rand::thread_rng();
    return if rng.gen::<i32>() % 2 == 0 {
        Ok("Hello World")
    } else {
        Err(ErrorCode::INVALID_PARAMETER)
    }

}