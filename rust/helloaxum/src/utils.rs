use std::time::Duration;
use axum::http::{header, Method};
use tower_http::cors::{AllowOrigin, Any, CorsLayer};
use tracing::debug;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::fmt;
use tracing_subscriber::layer::SubscriberExt;

pub fn init_tracing() -> WorkerGuard {
    let file_appender = tracing_appender::rolling::hourly("./logs", "log");
    let (file_writer, guard) = tracing_appender::non_blocking(file_appender);
    tracing::subscriber::set_global_default(
        fmt::Subscriber::builder()
            .with_max_level(tracing::Level::DEBUG)
            .finish()
            .with(fmt::Layer::default().with_writer(file_writer))
    ).expect("Unable to set global tracing subscriber");
    debug!("Tracing initialized.");
    guard
}

pub fn cors() -> CorsLayer {
    let cors = CorsLayer::new()
    .allow_credentials(true)
    .allow_headers(vec![
        header::ACCEPT,
        header::ACCEPT_LANGUAGE,
        header::AUTHORIZATION,
        header::CONTENT_LANGUAGE,
        header::CONTENT_TYPE,
    ])
    .allow_methods(vec![
        Method::GET,
        Method::POST,
        Method::PUT,
        Method::DELETE,
        Method::HEAD,
        Method::OPTIONS,
        Method::CONNECT,
        Method::PATCH,
        Method::TRACE,
    ])
    // .allow_origin(AllowOrigin::exact(
    //     "http://localhost:5173".parse().unwrap(), // Make sure this matches your frontend url
    // ))
    .allow_origin(Any)
    .max_age(Duration::from_secs(60 * 60));
    cors
}