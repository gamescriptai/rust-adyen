//! HTTP utilities and abstractions.

/// HTTP status codes commonly used in Adyen APIs.
pub mod status {
    /// Success
    pub const OK: u16 = 200;
    /// Created
    pub const CREATED: u16 = 201;
    /// Accepted
    pub const ACCEPTED: u16 = 202;
    /// No Content
    pub const NO_CONTENT: u16 = 204;

    /// Bad Request
    pub const BAD_REQUEST: u16 = 400;
    /// Unauthorized
    pub const UNAUTHORIZED: u16 = 401;
    /// Forbidden
    pub const FORBIDDEN: u16 = 403;
    /// Not Found
    pub const NOT_FOUND: u16 = 404;
    /// Method Not Allowed
    pub const METHOD_NOT_ALLOWED: u16 = 405;
    /// Conflict
    pub const CONFLICT: u16 = 409;
    /// Unprocessable Entity
    pub const UNPROCESSABLE_ENTITY: u16 = 422;
    /// Too Many Requests
    pub const TOO_MANY_REQUESTS: u16 = 429;

    /// Internal Server Error
    pub const INTERNAL_SERVER_ERROR: u16 = 500;
    /// Bad Gateway
    pub const BAD_GATEWAY: u16 = 502;
    /// Service Unavailable
    pub const SERVICE_UNAVAILABLE: u16 = 503;
    /// Gateway Timeout
    pub const GATEWAY_TIMEOUT: u16 = 504;
}

/// HTTP methods used in Adyen APIs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Method {
    /// GET method
    Get,
    /// POST method
    Post,
    /// PUT method
    Put,
    /// PATCH method
    Patch,
    /// DELETE method
    Delete,
}

impl Method {
    /// Convert to reqwest Method.
    #[must_use]
    pub const fn as_reqwest(&self) -> reqwest::Method {
        match self {
            Self::Get => reqwest::Method::GET,
            Self::Post => reqwest::Method::POST,
            Self::Put => reqwest::Method::PUT,
            Self::Patch => reqwest::Method::PATCH,
            Self::Delete => reqwest::Method::DELETE,
        }
    }
}

impl std::fmt::Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Get => write!(f, "GET"),
            Self::Post => write!(f, "POST"),
            Self::Put => write!(f, "PUT"),
            Self::Patch => write!(f, "PATCH"),
            Self::Delete => write!(f, "DELETE"),
        }
    }
}
