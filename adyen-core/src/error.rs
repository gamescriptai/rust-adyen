//! Error types for the Adyen library.


/// Result type alias for Adyen operations.
pub type Result<T> = std::result::Result<T, AdyenError>;

/// Main error type for all Adyen operations.
#[derive(Debug, thiserror::Error)]
pub enum AdyenError {
    /// HTTP request/response errors
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    /// JSON serialization/deserialization errors
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// API errors returned by Adyen
    #[error("API error: {error_message}")]
    Api {
        /// HTTP status code
        status: u16,
        /// Error code from Adyen
        error_code: String,
        /// Human-readable error message
        error_message: String,
        /// Error type classification
        error_type: String,
        /// PSP reference for tracking
        psp_reference: Option<String>,
    },

    /// Configuration errors
    #[error("Configuration error: {0}")]
    Config(String),

    /// Authentication errors
    #[error("Authentication error: {0}")]
    Auth(String),

    /// Validation errors
    #[error("Validation error: {0}")]
    Validation(#[from] validator::ValidationErrors),

    /// URL parsing errors
    #[error("URL error: {0}")]
    Url(#[from] url::ParseError),

    /// Generic errors for cases not covered above
    #[error("Adyen error: {message}")]
    Generic {
        /// Error message
        message: String,
        /// Optional source error
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },
}

impl AdyenError {
    /// Create a new API error.
    #[must_use]
    pub fn api(
        status: u16,
        error_code: impl Into<String>,
        error_message: impl Into<String>,
        error_type: impl Into<String>,
        psp_reference: Option<String>,
    ) -> Self {
        Self::Api {
            status,
            error_code: error_code.into(),
            error_message: error_message.into(),
            error_type: error_type.into(),
            psp_reference,
        }
    }

    /// Create a new configuration error.
    #[must_use]
    pub fn config(message: impl Into<String>) -> Self {
        Self::Config(message.into())
    }

    /// Create a new authentication error.
    #[must_use]
    pub fn auth(message: impl Into<String>) -> Self {
        Self::Auth(message.into())
    }

    /// Create a new generic error.
    #[must_use]
    pub fn generic(message: impl Into<String>) -> Self {
        Self::Generic {
            message: message.into(),
            source: None,
        }
    }

    /// Create a new generic error with a source.
    #[must_use]
    pub fn generic_with_source(
        message: impl Into<String>,
        source: Box<dyn std::error::Error + Send + Sync>,
    ) -> Self {
        Self::Generic {
            message: message.into(),
            source: Some(source),
        }
    }

    /// Check if this is an API error.
    #[must_use]
    pub const fn is_api_error(&self) -> bool {
        matches!(self, Self::Api { .. })
    }

    /// Check if this is a client error (4xx status code).
    #[must_use]
    pub const fn is_client_error(&self) -> bool {
        match self {
            Self::Api { status, .. } => *status >= 400 && *status < 500,
            _ => false,
        }
    }

    /// Check if this is a server error (5xx status code).
    #[must_use]
    pub const fn is_server_error(&self) -> bool {
        match self {
            Self::Api { status, .. } => *status >= 500,
            _ => false,
        }
    }

    /// Get the HTTP status code if this is an API error.
    #[must_use]
    pub const fn status_code(&self) -> Option<u16> {
        match self {
            Self::Api { status, .. } => Some(*status),
            _ => None,
        }
    }

    /// Get the PSP reference if available.
    #[must_use]
    pub fn psp_reference(&self) -> Option<&str> {
        match self {
            Self::Api { psp_reference, .. } => psp_reference.as_deref(),
            _ => None,
        }
    }
}