//! Environment configuration for Adyen APIs.

use crate::{AdyenError, Result};
use std::fmt;

/// Adyen API environment configuration.
///
/// Determines which Adyen environment to connect to and provides
/// the appropriate endpoint URLs for different APIs.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
)]
pub enum Environment {
    /// Test environment for development and testing
    Test,
    /// Live environment for production with URL prefix
    Live {
        /// URL prefix for live endpoints (e.g., "1797a841fbb37ca7-AdyenDemo")
        url_prefix: UrlPrefix,
    },
}

/// URL prefix for live environment endpoints.
///
/// This is a validated string that ensures the URL prefix meets Adyen's requirements.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
)]
pub struct UrlPrefix(Box<str>);

impl UrlPrefix {
    /// Create a new URL prefix with validation.
    ///
    /// # Errors
    ///
    /// Returns an error if the prefix is invalid (empty, too long, or contains invalid characters).
    pub fn new(prefix: impl Into<String>) -> Result<Self> {
        let prefix = prefix.into();

        if prefix.is_empty() {
            return Err(AdyenError::config("URL prefix cannot be empty"));
        }

        if prefix.len() > 100 {
            return Err(AdyenError::config(
                "URL prefix cannot be longer than 100 characters",
            ));
        }

        // Basic validation - only alphanumeric characters, hyphens, and underscores
        if !prefix
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
        {
            return Err(AdyenError::config(
                "URL prefix can only contain alphanumeric characters, hyphens, and underscores",
            ));
        }

        Ok(Self(prefix.into_boxed_str()))
    }

    /// Get the URL prefix as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for UrlPrefix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for UrlPrefix {
    type Err = AdyenError;

    fn from_str(s: &str) -> Result<Self> {
        Self::new(s)
    }
}

impl Environment {
    /// Create a test environment.
    #[must_use]
    pub const fn test() -> Self {
        Self::Test
    }

    /// Create a live environment with the given URL prefix.
    ///
    /// # Errors
    ///
    /// Returns an error if the URL prefix is invalid.
    pub fn live(url_prefix: impl Into<String>) -> Result<Self> {
        Ok(Self::Live {
            url_prefix: UrlPrefix::new(url_prefix)?,
        })
    }

    /// Check if this is the test environment.
    #[must_use]
    pub const fn is_test(&self) -> bool {
        matches!(self, Self::Test)
    }

    /// Check if this is the live environment.
    #[must_use]
    pub const fn is_live(&self) -> bool {
        matches!(self, Self::Live { .. })
    }

    /// Get the URL prefix for live environment.
    ///
    /// Returns `None` for test environment.
    #[must_use]
    pub const fn url_prefix(&self) -> Option<&UrlPrefix> {
        match self {
            Self::Test => None,
            Self::Live { url_prefix } => Some(url_prefix),
        }
    }

    /// Get the base URL for classic APIs (Payments, Recurring, etc.).
    #[must_use]
    pub fn classic_api_url(&self) -> String {
        match self {
            Self::Test => "https://pal-test.adyen.com".to_string(),
            Self::Live { url_prefix } => {
                format!("https://{}-pal-live.adyenpayments.com", url_prefix.as_str())
            }
        }
    }

    /// Get the base URL for Checkout API.
    #[must_use]
    pub fn checkout_api_url(&self) -> String {
        match self {
            Self::Test => "https://checkout-test.adyen.com".to_string(),
            Self::Live { url_prefix } => {
                format!(
                    "https://{}-checkout-live.adyenpayments.com",
                    url_prefix.as_str()
                )
            }
        }
    }

    /// Get the base URL for Management API.
    #[must_use]
    pub fn management_api_url(&self) -> String {
        match self {
            Self::Test => "https://management-test.adyen.com".to_string(),
            Self::Live { .. } => "https://management-live.adyen.com".to_string(),
        }
    }

    /// Get the base URL for Balance Platform API.
    #[must_use]
    pub fn balance_platform_api_url(&self) -> String {
        match self {
            Self::Test => "https://balanceplatform-api-test.adyen.com".to_string(),
            Self::Live { .. } => "https://balanceplatform-api-live.adyen.com".to_string(),
        }
    }

    /// Get the base URL for Transfers API.
    #[must_use]
    pub fn transfers_api_url(&self) -> String {
        match self {
            Self::Test => "https://balanceplatform-api-test.adyen.com".to_string(),
            Self::Live { .. } => "https://balanceplatform-api-live.adyen.com".to_string(),
        }
    }

    /// Get the base URL for Legal Entity Management API.
    #[must_use]
    pub fn legal_entity_api_url(&self) -> String {
        match self {
            Self::Test => "https://kyc-test.adyen.com".to_string(),
            Self::Live { .. } => "https://kyc-live.adyen.com".to_string(),
        }
    }

    /// Get the base URL for Disputes API.
    #[must_use]
    pub fn disputes_api_url(&self) -> String {
        match self {
            Self::Test => "https://ca-test.adyen.com".to_string(),
            Self::Live { .. } => "https://ca-live.adyen.com".to_string(),
        }
    }

    /// Get the base URL for Data Protection API.
    #[must_use]
    pub fn data_protection_api_url(&self) -> String {
        match self {
            Self::Test => "https://ca-test.adyen.com".to_string(),
            Self::Live { .. } => "https://ca-live.adyen.com".to_string(),
        }
    }

    /// Get the base URL for Terminal API.
    #[must_use]
    pub fn terminal_api_url(&self) -> String {
        match self {
            Self::Test => "https://terminal-api-test.adyen.com".to_string(),
            Self::Live { .. } => "https://terminal-api-live.adyen.com".to_string(),
        }
    }
}

impl fmt::Display for Environment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Test => write!(f, "test"),
            Self::Live { url_prefix } => write!(f, "live({})", url_prefix),
        }
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::test()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_environment_test() {
        let env = Environment::test();
        assert!(env.is_test());
        assert!(!env.is_live());
        assert!(env.url_prefix().is_none());
    }

    #[test]
    fn test_environment_live() {
        let env = Environment::live("test-prefix").unwrap();
        assert!(!env.is_test());
        assert!(env.is_live());
        assert_eq!(env.url_prefix().unwrap().as_str(), "test-prefix");
    }

    #[test]
    fn test_url_prefix_validation() {
        assert!(UrlPrefix::new("valid-prefix_123").is_ok());
        assert!(UrlPrefix::new("").is_err());
        assert!(UrlPrefix::new("invalid@prefix").is_err());
        assert!(UrlPrefix::new("a".repeat(101)).is_err());
    }

    #[test]
    fn test_api_urls() {
        let test_env = Environment::test();
        assert_eq!(test_env.classic_api_url(), "https://pal-test.adyen.com");
        assert_eq!(
            test_env.checkout_api_url(),
            "https://checkout-test.adyen.com"
        );
        assert_eq!(
            test_env.management_api_url(),
            "https://management-test.adyen.com"
        );

        let live_env = Environment::live("test-prefix").unwrap();
        assert_eq!(
            live_env.classic_api_url(),
            "https://test-prefix-pal-live.adyenpayments.com"
        );
        assert_eq!(
            live_env.checkout_api_url(),
            "https://test-prefix-checkout-live.adyenpayments.com"
        );
        assert_eq!(
            live_env.management_api_url(),
            "https://management-live.adyen.com"
        );
    }

    #[test]
    fn test_environment_display() {
        let test_env = Environment::test();
        assert_eq!(format!("{}", test_env), "test");

        let live_env = Environment::live("test-prefix").unwrap();
        assert_eq!(format!("{}", live_env), "live(test-prefix)");
    }
}
