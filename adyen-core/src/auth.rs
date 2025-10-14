//! Authentication mechanisms for Adyen APIs.

use crate::{AdyenError, Result};
use base64::prelude::*;
use std::fmt;

/// Authentication credentials for Adyen APIs.
#[derive(Debug, Clone)]
pub enum Credentials {
    /// API Key authentication (most common)
    ApiKey(ApiKey),
    /// Basic authentication (for Legal Entity Management API)
    Basic(BasicAuth),
}

/// API Key for authentication with Adyen APIs.
///
/// This is the most common authentication method for Adyen APIs.
/// The API key should be kept secure and never logged or exposed.
#[derive(Clone, PartialEq, Eq)]
pub struct ApiKey {
    key: String,
}

impl ApiKey {
    /// Create a new API key.
    ///
    /// # Errors
    ///
    /// Returns an error if the API key is empty or doesn't meet basic validation requirements.
    pub fn new(key: impl Into<String>) -> Result<Self> {
        let key = key.into();

        if key.is_empty() {
            return Err(AdyenError::auth("API key cannot be empty"));
        }

        if key.len() < 10 {
            return Err(AdyenError::auth("API key appears to be too short"));
        }

        if key.len() > 200 {
            return Err(AdyenError::auth("API key appears to be too long"));
        }

        // Basic format validation - should not contain whitespace
        if key.chars().any(char::is_whitespace) {
            return Err(AdyenError::auth("API key cannot contain whitespace"));
        }

        Ok(Self { key })
    }

    /// Get the API key as a string slice.
    ///
    /// This method should only be used when actually making API calls.
    /// Avoid logging or displaying this value.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.key
    }
}

impl fmt::Debug for ApiKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ApiKey([REDACTED])")
    }
}

impl fmt::Display for ApiKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[REDACTED API KEY]")
    }
}

/// Basic authentication credentials.
///
/// Used for some Adyen APIs, particularly the Legal Entity Management API.
#[derive(Clone, PartialEq, Eq)]
pub struct BasicAuth {
    username: String,
    password: String,
}

impl BasicAuth {
    /// Create new basic authentication credentials.
    ///
    /// # Errors
    ///
    /// Returns an error if username or password is empty.
    pub fn new(username: impl Into<String>, password: impl Into<String>) -> Result<Self> {
        let username = username.into();
        let password = password.into();

        if username.is_empty() {
            return Err(AdyenError::auth("Username cannot be empty"));
        }

        if password.is_empty() {
            return Err(AdyenError::auth("Password cannot be empty"));
        }

        Ok(Self { username, password })
    }

    /// Get the username.
    #[must_use]
    pub fn username(&self) -> &str {
        &self.username
    }

    /// Get the password.
    ///
    /// This method should only be used when actually making API calls.
    /// Avoid logging or displaying this value.
    #[must_use]
    pub fn password(&self) -> &str {
        &self.password
    }

    /// Generate the Authorization header value for basic authentication.
    #[must_use]
    pub fn authorization_header(&self) -> String {
        let credentials = format!("{}:{}", self.username, self.password);
        let encoded = base64::prelude::BASE64_STANDARD.encode(credentials);
        format!("Basic {}", encoded)
    }
}

impl fmt::Debug for BasicAuth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BasicAuth")
            .field("username", &self.username)
            .field("password", &"[REDACTED]")
            .finish()
    }
}

impl fmt::Display for BasicAuth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "BasicAuth(username: {}, password: [REDACTED])",
            self.username
        )
    }
}

impl Credentials {
    /// Create API key credentials.
    ///
    /// # Errors
    ///
    /// Returns an error if the API key is invalid.
    pub fn api_key(key: impl Into<String>) -> Result<Self> {
        Ok(Self::ApiKey(ApiKey::new(key)?))
    }

    /// Create basic authentication credentials.
    ///
    /// # Errors
    ///
    /// Returns an error if the username or password is invalid.
    pub fn basic(username: impl Into<String>, password: impl Into<String>) -> Result<Self> {
        Ok(Self::Basic(BasicAuth::new(username, password)?))
    }

    /// Check if these are API key credentials.
    #[must_use]
    pub const fn is_api_key(&self) -> bool {
        matches!(self, Self::ApiKey(_))
    }

    /// Check if these are basic authentication credentials.
    #[must_use]
    pub const fn is_basic(&self) -> bool {
        matches!(self, Self::Basic(_))
    }

    /// Get the API key if these are API key credentials.
    #[must_use]
    pub const fn as_api_key(&self) -> Option<&ApiKey> {
        match self {
            Self::ApiKey(key) => Some(key),
            Self::Basic(_) => None,
        }
    }

    /// Get the basic auth if these are basic authentication credentials.
    #[must_use]
    pub const fn basic_auth(&self) -> Option<&BasicAuth> {
        match self {
            Self::ApiKey(_) => None,
            Self::Basic(auth) => Some(auth),
        }
    }
}

impl fmt::Display for Credentials {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ApiKey(_) => write!(f, "ApiKey([REDACTED])"),
            Self::Basic(auth) => write!(f, "{}", auth),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_key_creation() {
        let key = ApiKey::new("test_api_key_12345").unwrap();
        assert_eq!(key.as_str(), "test_api_key_12345");
    }

    #[test]
    fn test_api_key_validation() {
        assert!(ApiKey::new("").is_err()); // Empty
        assert!(ApiKey::new("short").is_err()); // Too short
        assert!(ApiKey::new("key with spaces").is_err()); // Contains whitespace
        assert!(ApiKey::new("a".repeat(201)).is_err()); // Too long
    }

    #[test]
    fn test_api_key_debug() {
        let key = ApiKey::new("secret_key_12345").unwrap();
        let debug_str = format!("{:?}", key);
        assert!(!debug_str.contains("secret_key_12345"));
        assert!(debug_str.contains("REDACTED"));
    }

    #[test]
    fn test_basic_auth_creation() {
        let auth = BasicAuth::new("username", "password").unwrap();
        assert_eq!(auth.username(), "username");
        assert_eq!(auth.password(), "password");
    }

    #[test]
    fn test_basic_auth_validation() {
        assert!(BasicAuth::new("", "password").is_err()); // Empty username
        assert!(BasicAuth::new("username", "").is_err()); // Empty password
    }

    #[test]
    fn test_basic_auth_authorization_header() {
        let auth = BasicAuth::new("user", "pass").unwrap();
        let header = auth.authorization_header();
        assert_eq!(header, "Basic dXNlcjpwYXNz"); // base64("user:pass")
    }

    #[test]
    fn test_basic_auth_debug() {
        let auth = BasicAuth::new("username", "secret_password").unwrap();
        let debug_str = format!("{:?}", auth);
        assert!(debug_str.contains("username"));
        assert!(!debug_str.contains("secret_password"));
        assert!(debug_str.contains("REDACTED"));
    }

    #[test]
    fn test_credentials() {
        let api_creds = Credentials::api_key("test_key_12345").unwrap();
        assert!(api_creds.is_api_key());
        assert!(!api_creds.is_basic());
        assert!(api_creds.as_api_key().is_some());
        assert!(api_creds.basic_auth().is_none());

        let basic_creds = Credentials::basic("user", "pass").unwrap();
        assert!(!basic_creds.is_api_key());
        assert!(basic_creds.is_basic());
        assert!(basic_creds.as_api_key().is_none());
        assert!(basic_creds.basic_auth().is_some());
    }
}
