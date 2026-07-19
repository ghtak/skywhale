use std::path::{Path, PathBuf};

use anyhow::Context;
use serde::Deserialize;
use serde::de::DeserializeOwned;

use crate::Result;

/// Loads configuration from a TOML file, environment variables, and explicit
/// programmatic overrides.
///
/// Later layers take precedence: file, environment variables, then overrides.
pub struct ConfigLoader {
    file: Option<(PathBuf, bool)>,
    env_prefix: Option<String>,
    overrides: Vec<(String, config::Value)>,
}

impl ConfigLoader {
    /// Creates a loader with a required TOML configuration file.
    pub fn from_file(path: impl AsRef<Path>) -> Self {
        Self {
            file: Some((path.as_ref().to_path_buf(), true)),
            env_prefix: None,
            overrides: Vec::new(),
        }
    }

    /// Creates a loader whose TOML file is optional.
    pub fn optional_file(path: impl AsRef<Path>) -> Self {
        Self {
            file: Some((path.as_ref().to_path_buf(), false)),
            env_prefix: None,
            overrides: Vec::new(),
        }
    }

    /// Adds environment variables with `prefix` and `__` as a nesting separator.
    ///
    /// For example, `SKYWHALE_HTTP__PORT=9090` maps to `http.port` when the
    /// prefix is `SKYWHALE`.
    pub fn with_env_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.env_prefix = Some(prefix.into());
        self
    }

    /// Adds a programmatic value that takes precedence over all other layers.
    pub fn with_override(
        mut self,
        key: impl Into<String>,
        value: impl Into<config::Value>,
    ) -> Self {
        self.overrides.push((key.into(), value.into()));
        self
    }

    /// Loads and merges all configured sources.
    pub fn load(self) -> Result<config::Config> {
        let mut builder = config::Config::builder();

        if let Some((path, required)) = self.file {
            builder = builder.add_source(config::File::from(path).required(required));
        }

        if let Some(prefix) = self.env_prefix {
            builder = builder.add_source(
                config::Environment::with_prefix(&prefix)
                    .prefix_separator("_")
                    .separator("__")
                    .try_parsing(true),
            );
        }

        for (key, value) in self.overrides {
            builder = builder
                .set_override(&key, value)
                .with_context(|| format!("invalid configuration override key `{key}`"))?;
        }

        let config = builder.build().context("failed to build configuration")?;
        Ok(config)
    }

    /// Loads the merged configuration and deserializes it into `T`.
    pub fn try_deserialize<T: DeserializeOwned>(self) -> Result<T> {
        self.load()?
            .try_deserialize()
            .context("failed to deserialize configuration")
            .map_err(Into::into)
    }
}

/// The built-in Skywhale configuration.
#[derive(Debug, Clone, Deserialize)]
pub struct SkywhaleConfig {
    #[serde(default)]
    pub http: HttpConfig,
}

/// HTTP server settings for Skywhale.
#[derive(Debug, Clone, Deserialize)]
pub struct HttpConfig {
    #[serde(default = "default_http_host")]
    pub host: String,

    #[serde(default = "default_http_port")]
    pub port: u16,
}

impl Default for HttpConfig {
    fn default() -> Self {
        Self {
            host: default_http_host(),
            port: default_http_port(),
        }
    }
}

fn default_http_host() -> String {
    "127.0.0.1".to_owned()
}

const fn default_http_port() -> u16 {
    8080
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::sync::Mutex;
    use std::sync::atomic::{AtomicUsize, Ordering};

    use serde::Deserialize;

    use super::{ConfigLoader, SkywhaleConfig};

    static TEMP_FILE_COUNTER: AtomicUsize = AtomicUsize::new(0);
    static ENVIRONMENT_LOCK: Mutex<()> = Mutex::new(());

    fn temporary_toml(contents: &str) -> std::path::PathBuf {
        let id = TEMP_FILE_COUNTER.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "skywhale-config-test-{}-{id}.toml",
            std::process::id()
        ));
        fs::write(&path, contents).expect("test configuration must be writable");
        path
    }

    #[test]
    fn loads_toml_into_a_custom_structure() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct Settings {
            service: Service,
        }

        #[derive(Debug, Deserialize, PartialEq)]
        struct Service {
            name: String,
        }

        let path = temporary_toml("[service]\nname = 'whale'\n");
        let settings: Settings = ConfigLoader::from_file(&path)
            .try_deserialize()
            .expect("TOML must deserialize");
        fs::remove_file(path).expect("test configuration must be removable");

        assert_eq!(settings.service.name, "whale");
    }

    #[test]
    fn explicit_override_has_highest_precedence() {
        #[derive(Deserialize)]
        struct Settings {
            http: Http,
        }

        #[derive(Deserialize)]
        struct Http {
            port: u16,
        }

        let path = temporary_toml("[http]\nport = 8081\n");
        let settings: Settings = ConfigLoader::from_file(&path)
            .with_override("http.port", 0_u16)
            .try_deserialize()
            .expect("override must deserialize");
        fs::remove_file(path).expect("test configuration must be removable");

        assert_eq!(settings.http.port, 0);
    }

    #[test]
    fn environment_overrides_toml() {
        #[derive(Deserialize)]
        struct Settings {
            http: Http,
        }

        #[derive(Deserialize)]
        struct Http {
            port: u16,
        }

        let _environment_lock = ENVIRONMENT_LOCK
            .lock()
            .expect("environment test lock must not be poisoned");
        let id = TEMP_FILE_COUNTER.fetch_add(1, Ordering::Relaxed);
        let prefix = format!("SKYWHALE_TEST_{id}");
        let variable = format!("{prefix}_HTTP__PORT");
        let path = temporary_toml("[http]\nport = 8081\n");

        // Environment mutation is process-global. The mutex prevents this test
        // from racing with other tests in this crate.
        unsafe { std::env::set_var(&variable, "9090") };
        let settings: Settings = ConfigLoader::from_file(&path)
            .with_env_prefix(&prefix)
            .try_deserialize()
            .expect("environment value must deserialize");
        unsafe { std::env::remove_var(&variable) };
        fs::remove_file(path).expect("test configuration must be removable");

        assert_eq!(settings.http.port, 9090);
    }

    #[test]
    fn skywhale_config_uses_http_defaults_when_optional_file_is_missing() {
        let path = std::env::temp_dir().join(format!(
            "skywhale-config-missing-{}.toml",
            std::process::id()
        ));
        let config: SkywhaleConfig = ConfigLoader::optional_file(path)
            .try_deserialize()
            .expect("missing optional file must use defaults");

        assert_eq!(config.http.host, "127.0.0.1");
        assert_eq!(config.http.port, 8080);
    }
}
