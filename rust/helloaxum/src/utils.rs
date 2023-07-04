use axum::extract::rejection::JsonRejection;
use axum_macros::FromRequest;
use tracing::debug;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::fmt;
use tracing_subscriber::layer::SubscriberExt;
use crate::error::Error;

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

#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(Error))]
pub struct JsonParam<T>(pub T);

impl From<JsonRejection> for Error {
    fn from(rejection: JsonRejection) -> Self {
        return match rejection {
            JsonRejection::JsonDataError(_) => Error::JsonDataError(rejection.to_string()),
            JsonRejection::JsonSyntaxError(_) => Error::JsonSyntaxError(rejection.to_string()),
            JsonRejection::MissingJsonContentType(_) => Error::MissingJsonContentType(rejection.to_string()),
            _ => Error::JsonRejection(rejection.to_string()),
        }
    }
}
