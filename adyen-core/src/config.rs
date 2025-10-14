//! Configuration management for Adyen clients.

use crate::{auth::Credentials, environment::Environment, AdyenError, Result};
use std::time::Duration;

/// Configuration for Adyen API clients.
///
/// Contains all necessary settings for connecting to Adyen APIs including
/// authentication, environment, timeouts, and other client options.
#[derive(Debug, Clone)]
pub struct Config {
    /// Authentication credentials
    credentials: Credentials,
    /// API environment (test or live)
    environment: Environment,
    /// HTTP request timeout
    timeout: Duration,
    /// User agent string for requests
    user_agent: String,
    /// Additional default headers
    default_headers: std::collections::HashMap<String, String>,
    /// Enable request/response logging
    enable_logging: bool,
}

/// Builder for creating Adyen client configuration.
///
/// Uses the type-state pattern to ensure required fields are set.
#[derive(Debug, Default)]
pub struct ConfigBuilder {
    credentials: Option<Credentials>,
    environment: Option<Environment>,
    timeout: Option<Duration>,
    user_agent: Option<String>,
    default_headers: std::collections::HashMap<String, String>,
    enable_logging: bool,
}

impl ConfigBuilder {
    /// Create a new configuration builder.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the authentication credentials.
    #[must_use]
    pub fn credentials(mut self, credentials: Credentials) -> Self {
        self.credentials = Some(credentials);
        self
    }

    /// Set the API key for authentication.
    ///
    /// # Errors
    ///
    /// Returns an error if the API key is invalid.
    pub fn api_key(mut self, api_key: impl Into<String>) -> Result<Self> {
        self.credentials = Some(Credentials::api_key(api_key)?);
        Ok(self)
    }

    /// Set basic authentication credentials.
    ///
    /// # Errors
    ///
    /// Returns an error if the username or password is invalid.
    pub fn basic_auth(
        mut self,
        username: impl Into<String>,
        password: impl Into<String>,
    ) -> Result<Self> {
        self.credentials = Some(Credentials::basic(username, password)?);
        Ok(self)
    }

    /// Set the environment.
    #[must_use]
    pub fn environment(mut self, environment: Environment) -> Self {
        self.environment = Some(environment);
        self
    }

    /// Set the request timeout.
    #[must_use]
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Set the user agent string.
    #[must_use]
    pub fn user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = Some(user_agent.into());
        self
    }

    /// Add a default header.
    #[must_use]
    pub fn default_header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.default_headers.insert(name.into(), value.into());
        self
    }

    /// Enable or disable request/response logging.
    #[must_use]
    pub fn enable_logging(mut self, enable: bool) -> Self {
        self.enable_logging = enable;
        self
    }

    /// Build the configuration.
    ///
    /// # Errors
    ///
    /// Returns an error if required fields are missing.
    pub fn build(self) -> Result<Config> {
        let credentials = self.credentials
            .ok_or_else(|| AdyenError::config("Credentials are required"))?;

        let environment = self.environment.unwrap_or_default();

        let timeout = self.timeout.unwrap_or_else(|| Duration::from_secs(60));

        let user_agent = self.user_agent.unwrap_or_else(|| crate::USER_AGENT.to_string());

        Ok(Config {
            credentials,
            environment,
            timeout,
            user_agent,
            default_headers: self.default_headers,
            enable_logging: self.enable_logging,
        })
    }
}

impl Config {
    /// Create a new configuration builder.
    #[must_use]
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::new()
    }

    /// Get the credentials.
    #[must_use]
    pub const fn credentials(&self) -> &Credentials {
        &self.credentials
    }

    /// Get the environment.
    #[must_use]
    pub const fn environment(&self) -> &Environment {
        &self.environment
    }

    /// Get the timeout.
    #[must_use]
    pub const fn timeout(&self) -> Duration {
        self.timeout
    }

    /// Get the user agent.
    #[must_use]
    pub fn user_agent(&self) -> &str {
        &self.user_agent
    }

    /// Get the default headers.
    #[must_use]
    pub fn default_headers(&self) -> &std::collections::HashMap<String, String> {
        &self.default_headers
    }

    /// Check if logging is enabled.
    #[must_use]
    pub const fn is_logging_enabled(&self) -> bool {
        self.enable_logging
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::environment::Environment;

    #[test]
    fn test_config_builder() {
        let config = ConfigBuilder::new()
            .api_key("test_key_12345")
            .unwrap()
            .environment(Environment::test())
            .timeout(Duration::from_secs(30))
            .user_agent("test-agent")
            .default_header("X-Test", "value")
            .enable_logging(true)
            .build()
            .unwrap();

        assert!(config.credentials().is_api_key());
        assert!(config.environment().is_test());
        assert_eq!(config.timeout(), Duration::from_secs(30));
        assert_eq!(config.user_agent(), "test-agent");
        assert_eq!(config.default_headers().get("X-Test"), Some(&"value".to_string()));
        assert!(config.is_logging_enabled());
    }

    #[test]
    fn test_config_builder_defaults() {
        let config = ConfigBuilder::new()
            .api_key("test_key_12345")
            .unwrap()
            .build()
            .unwrap();

        assert!(config.environment().is_test());
        assert_eq!(config.timeout(), Duration::from_secs(60));
        assert_eq!(config.user_agent(), crate::USER_AGENT);
        assert!(!config.is_logging_enabled());
    }

    #[test]
    fn test_config_builder_missing_credentials() {
        let result = ConfigBuilder::new().build();
        assert!(result.is_err());
    }
}