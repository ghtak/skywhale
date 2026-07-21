//! Axum HTTP server lifecycle helpers.

use std::future::Future;

use anyhow::Context;
use axum::Router;
use tokio::net::TcpListener;
use tracing::info;

use crate::{HttpConfig, Result};

/// Binds and serves an Axum router using the supplied HTTP configuration.
///
/// The function returns when the process receives Ctrl-C or, on Unix, a
/// `SIGTERM`. Binding and server errors are returned with context.
pub async fn serve(router: Router, config: &HttpConfig) -> Result<()> {
    serve_with_shutdown(router, config, shutdown_signal()).await
}

async fn serve_with_shutdown<F>(router: Router, config: &HttpConfig, shutdown: F) -> Result<()>
where
    F: Future<Output = ()> + Send + 'static,
{
    let listener = TcpListener::bind((config.host.as_str(), config.port))
        .await
        .with_context(|| {
            format!(
                "failed to bind HTTP listener to {}:{}",
                config.host, config.port
            )
        })?;
    let address = listener
        .local_addr()
        .context("failed to read bound HTTP listener address")?;

    info!(%address, "HTTP server listening");

    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown)
        .await
        .context("HTTP server terminated unexpectedly")?;

    info!(%address, "HTTP server stopped");
    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        if let Err(error) = tokio::signal::ctrl_c().await {
            tracing::warn!(%error, "failed to listen for Ctrl-C");
        }
    };

    #[cfg(unix)]
    {
        use tokio::signal::unix::{SignalKind, signal};

        let terminate = async {
            match signal(SignalKind::terminate()) {
                Ok(mut stream) => {
                    stream.recv().await;
                }
                Err(error) => tracing::warn!(%error, "failed to listen for SIGTERM"),
            }
        };

        tokio::select! {
            () = ctrl_c => {},
            () = terminate => {},
        }
    }

    #[cfg(not(unix))]
    ctrl_c.await;
}

#[cfg(test)]
mod tests {
    use std::future;

    use axum::{
        Router,
        body::Body,
        http::{Request, StatusCode},
        routing::get,
    };
    use tokio::net::TcpListener;
    use tower::ServiceExt;

    use super::serve_with_shutdown;
    use crate::HttpConfig;

    #[tokio::test]
    async fn router_handles_a_basic_request() {
        let app = Router::new().route("/health", get(|| async { StatusCode::OK }));

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .expect("valid test request"),
            )
            .await
            .expect("router response");

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn returns_an_error_for_an_invalid_host() {
        let config = HttpConfig {
            host: "[".to_owned(),
            port: 0,
        };

        let result = serve_with_shutdown(Router::new(), &config, future::pending()).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn returns_an_error_when_the_port_is_already_in_use() {
        let occupied_listener = TcpListener::bind(("127.0.0.1", 0))
            .await
            .expect("test listener must bind");
        let config = HttpConfig {
            host: "127.0.0.1".to_owned(),
            port: occupied_listener
                .local_addr()
                .expect("test listener address")
                .port(),
        };

        let result = serve_with_shutdown(Router::new(), &config, future::pending()).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn stops_when_the_shutdown_future_completes() {
        let config = HttpConfig {
            host: "127.0.0.1".to_owned(),
            port: 0,
        };

        serve_with_shutdown(Router::new(), &config, async {})
            .await
            .expect("server must stop cleanly");
    }
}
