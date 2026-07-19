use thiserror::Error;

pub(crate) fn into_db_error(error: sqlx::Error) -> Error {
    Error::Database(anyhow::Error::new(error))
}

/// Errors surfaced by `skywhale-core`.
///
/// Add a dedicated variant only when callers need to distinguish the failure
/// and take different action, such as retrying, ignoring an already-completed
/// operation, or selecting a different control flow. Otherwise, preserve the
/// original error and its context with [`Error::Other`].
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    /// The operation requires authentication.
    #[error("authentication is required")]
    Unauthenticated,

    /// The authenticated caller is not allowed to perform the operation.
    #[error("permission denied")]
    Forbidden,

    /// A failure returned by the database driver.
    ///
    /// The source retains the original `sqlx::Error` for database-specific
    /// classification at the persistence boundary.
    #[error("database error: {0}")]
    Database(#[source] anyhow::Error),

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

    #[test]
    fn errors_match() {
        let err = Error::Other(anyhow::anyhow!("some error"));
        match err {
            Error::Other(_) => print!("other error"),
            _ => print!("wild matches"),
        }
    }
}
