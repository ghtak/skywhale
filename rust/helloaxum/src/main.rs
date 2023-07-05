use std::time::Duration;

use axum::{BoxError, handler::HandlerWithoutStateExt, http::StatusCode, Router, routing::get};
use axum::error_handling::HandleErrorLayer;
use axum::http::{Method, Response};
use dotenv::dotenv;
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

#[tokio::main]
async fn main() {
    let _guard = init_tracing();

    dotenv().ok();

    let port = std::env::var("PORT").unwrap_or_else(|_| "8010".into());
    debug!("{}", port);

    let addr = format!("0.0.0.0:{}", port);

    //let service = handle_404.into_service();
    //let serve_dir = ServeDir::new("static").not_found_service(service);
    let router_main = Router::new()
        .route("/", get(hello_axum))
        .nest_service("/api/v1/user", routers::v1::user::router())
        .nest_service("/api/v1/login",
              routers::v1::login::router()
        ).fallback(
           |uri: axum::http::Uri| async move {
               (StatusCode::NOT_FOUND, format!("NotFound {:?}", uri))}
        );
        //.layer(utils::cors())
        // .layer(
        //     ServiceBuilder::new()
        //         .layer(HandleErrorLayer::new(|error: BoxError| async move {
        //             debug!("{:?}", error);
        //         }))
        //         .timeout(Duration::from_secs(10))
        //         .layer(TraceLayer::new_for_http())
        //         .into_inner()
        // )
        // .fallback(|| async {
        //      (StatusCode::NOT_FOUND, "nothing to see here")
        // });
        // .fallback_service(
        //     Router::new().nest_service("/static", serve_dir)
        // );

    axum::Server::bind(&(addr.parse().unwrap()))
        .serve(router_main.into_make_service())
        .await
        .unwrap();
}

async fn hello_axum() -> Result<&'static str> {
    return Ok("Hello Axum");
}