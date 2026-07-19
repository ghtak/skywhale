mod config;
mod error;
mod tracing;

pub use config::{
    ConfigLoader, ConsoleTraceConfig, FileTraceConfig, HttpConfig, SkywhaleConfig, TraceConfig,
};
pub use error::Error;
pub use tracing::init_tracing;

pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::{Error, Result};

    #[test]
    fn result_alias_uses_core_error() {
        fn fails() -> Result<()> {
            Err(anyhow::anyhow!("connection refused"))?;
            Ok(())
        }

        assert!(matches!(fails(), Err(Error::Other(_))));
    }
}
