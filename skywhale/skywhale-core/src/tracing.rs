//! Process-wide tracing subscriber initialization.

use std::sync::OnceLock;

use anyhow::{Context, bail};
use tracing_appender::{non_blocking::WorkerGuard, rolling::daily};
use tracing_subscriber::{
    EnvFilter, Layer, Registry, fmt, layer::SubscriberExt, util::SubscriberInitExt,
};

use crate::{
    Result,
    config::{ConsoleTraceConfig, FileTraceConfig, TraceConfig},
};

static LOGGING_GUARDS: OnceLock<Vec<WorkerGuard>> = OnceLock::new();

/// Installs Skywhale's process-wide tracing subscriber.
///
/// If `RUST_LOG` is set, its valid value overrides the filters for every
/// configured output. Otherwise each output's configured filter is used.
/// Calling this more than once, or after another global subscriber has been
/// installed, returns an error.
pub fn init_tracing(trace_config: &TraceConfig) -> Result<()> {
    if trace_config.console.is_none() && trace_config.file.is_none() {
        return Ok(());
    }

    let mut guards = Vec::new();

    let console_layer = trace_config
        .console
        .as_ref()
        .map(|config| -> anyhow::Result<_> {
            validate_console_config(config)?;
            let (writer, guard) = tracing_appender::non_blocking::NonBlockingBuilder::default()
                .buffered_lines_limit(config.buffered_lines_limit)
                .lossy(config.lossy)
                .finish(std::io::stderr());
            guards.push(guard);

            Ok(fmt::layer()
                .with_writer(writer)
                .compact()
                .with_filter(resolve_filter(&config.filter)?))
        })
        .transpose()?;

    let file_layer = trace_config
        .file
        .as_ref()
        .map(|config| -> anyhow::Result<_> {
            validate_file_config(config)?;
            std::fs::create_dir_all(&config.directory).with_context(|| {
                format!(
                    "failed to create log directory `{}`",
                    config.directory.display()
                )
            })?;
            let (writer, guard) = tracing_appender::non_blocking::NonBlockingBuilder::default()
                .buffered_lines_limit(config.buffered_lines_limit)
                .lossy(config.lossy)
                .finish(daily(&config.directory, &config.filename));
            guards.push(guard);

            Ok(fmt::layer()
                .with_writer(writer)
                .json()
                .with_filter(resolve_filter(&config.filter)?))
        })
        .transpose()?;

    Registry::default()
        .with(console_layer)
        .with(file_layer)
        .try_init()
        .context("failed to install the global tracing subscriber")?;

    LOGGING_GUARDS
        .set(guards)
        .map_err(|_| anyhow::anyhow!("tracing worker guards were already installed"))?;
    Ok(())
}

fn resolve_filter(configured_filter: &str) -> anyhow::Result<EnvFilter> {
    if std::env::var_os(EnvFilter::DEFAULT_ENV).is_some() {
        EnvFilter::try_from_default_env().context("invalid RUST_LOG filter")
    } else {
        EnvFilter::try_new(configured_filter)
            .with_context(|| format!("invalid tracing filter `{configured_filter}`"))
    }
}

fn validate_console_config(config: &ConsoleTraceConfig) -> anyhow::Result<()> {
    validate_buffered_lines_limit(config.buffered_lines_limit)
}

fn validate_file_config(config: &FileTraceConfig) -> anyhow::Result<()> {
    validate_buffered_lines_limit(config.buffered_lines_limit)?;
    if config.directory.as_os_str().is_empty() {
        bail!("log directory must not be empty");
    }
    if config.filename.is_empty() {
        bail!("log filename must not be empty");
    }
    Ok(())
}

fn validate_buffered_lines_limit(limit: usize) -> anyhow::Result<()> {
    if limit == 0 {
        bail!("tracing buffered_lines_limit must be greater than zero");
    }
    Ok(())
}
