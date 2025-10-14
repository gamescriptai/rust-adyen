//! HTTP client implementation for Adyen APIs.

use crate::{Config, Result};

/// HTTP client for making requests to Adyen APIs.
///
/// This client handles authentication, request/response serialization,
/// error handling, and provides a consistent interface for all Adyen APIs.
#[derive(Debug, Clone)]
pub struct Client {
    config: Config,
    http_client: reqwest::Client,
}

impl Client {
    /// Create a new client with the given configuration.
    ///
    /// # Errors
    ///
    /// Returns an error if the HTTP client cannot be created.
    pub fn new(config: Config) -> Result<Self> {
        let mut headers = reqwest::header::HeaderMap::new();

        // Add default headers
        for (name, value) in config.default_headers() {
            let header_name = reqwest::header::HeaderName::from_bytes(name.as_bytes())
                .map_err(|e| crate::AdyenError::config(format!("Invalid header name '{}': {}", name, e)))?;
            let header_value = reqwest::header::HeaderValue::from_str(value)
                .map_err(|e| crate::AdyenError::config(format!("Invalid header value '{}': {}", value, e)))?;
            headers.insert(header_name, header_value);
        }

        // Add User-Agent
        headers.insert(
            reqwest::header::USER_AGENT,
            reqwest::header::HeaderValue::from_str(config.user_agent())
                .map_err(|e| crate::AdyenError::config(format!("Invalid user agent: {}", e)))?,
        );

        // Build HTTP client
        let http_client = reqwest::ClientBuilder::new()
            .timeout(config.timeout())
            .default_headers(headers)
            .build()
            .map_err(|e| crate::AdyenError::config(format!("Failed to create HTTP client: {}", e)))?;

        Ok(Self {
            config,
            http_client,
        })
    }

    /// Get the client configuration.
    #[must_use]
    pub const fn config(&self) -> &Config {
        &self.config
    }

    /// Get the underlying HTTP client.
    #[must_use]
    pub const fn http_client(&self) -> &reqwest::Client {
        &self.http_client
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ConfigBuilder, Environment};

    #[test]
    fn test_client_creation() {
        let config = ConfigBuilder::new()
            .api_key("test_key_12345")
            .unwrap()
            .environment(Environment::test())
            .build()
            .unwrap();

        let client = Client::new(config).unwrap();
        assert!(client.config().environment().is_test());
    }
}