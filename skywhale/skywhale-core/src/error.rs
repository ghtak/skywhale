use thiserror::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

#[cfg(test)]
mod tests {
    use super::Error;
    use anyhow::Context;
    use std::error::Error as StdError;

    #[test]
    fn converts_anyhow_errors_transparently() {
        let source = anyhow::Error::msg("connection refused").context("loading account record");
        let error = Error::from(source);

        assert_eq!(error.to_string(), "loading account record");
        assert!(StdError::source(&error).is_some());
    }

    #[test]
    fn question_mark_converts_anyhow_errors() {
        fn load() -> Result<(), Error> {
            Err(anyhow::anyhow!("connection refused")).context("loading account record")?;
            Ok(())
        }

        let error = load().expect_err("the test operation must fail");
        assert_eq!(error.to_string(), "loading account record");
    }
}
