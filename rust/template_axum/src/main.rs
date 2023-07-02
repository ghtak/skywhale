mod error;

use std::io;
use std::io::ErrorKind;
use error::ErrorCode;

use dotenv::dotenv;

use axum::{
    routing::get,
    Router,
};

use tracing::debug;
use tracing_subscriber::{
    fmt,
    layer::SubscriberExt,
};
use tracing_appender::{
    non_blocking::{
        WorkerGuard,
    },
};

use rand::Rng;

fn init_tracing() -> WorkerGuard {
    let file_appender = tracing_appender::rolling::hourly("./logs", "log");
    let (file_writer, guard) = tracing_appender::non_blocking(file_appender);
    tracing::subscriber::set_global_default(
        fmt::Subscriber::builder()
            .with_max_level(tracing::Level::INFO)
            .finish()
            .with(fmt::Layer::default().with_writer(file_writer))
    ).expect("Unable to set global tracing subscriber");
    debug!("Tracing initialized.");
    guard
}


#[tokio::main]
async fn main() {
    dotenv().ok();
    let _guard = init_tracing();
    let port = std::env::var("PORT").unwrap_or_else(|_| "8009".into());

    let main_router = Router::new().route("/", get(hello_axum));

    let addr = format!("0.0.0.0:{}", port);

    debug!("listening on {}", addr);

    // run it with hyper on localhost:3000
    axum::Server::bind(&(addr.parse().unwrap()))
        .serve(main_router.into_make_service())
        .await
        .unwrap();
}


async fn random_hello_world() -> Result<&'static str, ErrorCode> {
    let mut rng = rand::thread_rng();
    return if rng.gen::<i32>() % 2 == 0 {
        Ok("Hello Axum")
    } else {
        tracing::info!("{:?}", ErrorCode::UnknownError);
        Err(
            io::Error::new(
                ErrorKind::AddrInUse,
                io::Error::new(ErrorKind::AlreadyExists, "Inner")
            ).into()
        )
    };
}

async fn hello_axum() -> Result<&'static str, ErrorCode> {
    let ret = random_hello_world().await?;
    return Ok(ret);
}
