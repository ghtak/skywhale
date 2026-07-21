use std::{env, ffi::OsString, path::PathBuf};

use anyhow::{Context, bail};
use axum::{Router, http::StatusCode, routing::get};
use skywhale_core::{ConfigLoader, SkywhaleConfig, http, init_tracing};

const DEFAULT_CONFIG_PATH: &str = "config/skywhale.toml";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let Some(config_path) = config_path(env::args_os())? else {
        return Ok(());
    };

    let config: SkywhaleConfig = ConfigLoader::optional_file(&config_path)
        .with_env_prefix("SKYWHALE")
        .try_deserialize()
        .with_context(|| {
            format!(
                "failed to load configuration from `{}`",
                config_path.display()
            )
        })?;

    init_tracing(&config.tracing).context("failed to initialize tracing")?;
    http::serve(app(), &config.http)
        .await
        .context("Skywhale server failed")?;
    Ok(())
}

fn app() -> Router {
    Router::new().route("/health", get(health))
}

async fn health() -> StatusCode {
    StatusCode::OK
}

fn config_path(args: impl IntoIterator<Item = OsString>) -> anyhow::Result<Option<PathBuf>> {
    let mut args = args.into_iter();
    let _program_name = args.next();
    let mut path = PathBuf::from(DEFAULT_CONFIG_PATH);

    while let Some(argument) = args.next() {
        if argument == "--help" || argument == "-h" {
            println!("Usage: skywhale-cli [--config <PATH>]\n\nRuns Skywhale's HTTP server.");
            return Ok(None);
        }

        if argument == "--config" {
            let value = args.next().context("`--config` requires a path")?;
            path = PathBuf::from(value);
            continue;
        }

        bail!(
            "unknown argument `{}`; use --help for usage",
            argument.to_string_lossy()
        );
    }

    Ok(Some(path))
}

#[cfg(test)]
mod tests {
    use axum::{body::Body, http::Request};
    use tower::ServiceExt;

    use super::{DEFAULT_CONFIG_PATH, app, config_path};

    #[tokio::test]
    async fn health_endpoint_is_available() {
        let response = app()
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .expect("valid request"),
            )
            .await
            .expect("router response");

        assert_eq!(response.status(), axum::http::StatusCode::OK);
    }

    #[test]
    fn uses_the_default_configuration_path() {
        let path = config_path(["skywhale-cli".into()])
            .expect("arguments must parse")
            .expect("normal invocation must run");

        assert_eq!(path, std::path::PathBuf::from(DEFAULT_CONFIG_PATH));
    }

    #[test]
    fn accepts_a_configuration_path_override() {
        let path = config_path(["skywhale-cli".into(), "--config".into(), "test.toml".into()])
            .expect("arguments must parse")
            .expect("normal invocation must run");

        assert_eq!(path, std::path::PathBuf::from("test.toml"));
    }
}
