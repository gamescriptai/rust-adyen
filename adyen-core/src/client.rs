//! HTTP client implementation for Adyen APIs.

use crate::{auth::Credentials, AdyenError, Config, Result};
use reqwest::{header::HeaderMap, RequestBuilder, Response};
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// HTTP client for making requests to Adyen APIs.
///
/// This client handles authentication, request/response serialization,
/// error handling, retry logic, and provides a consistent interface for all Adyen APIs.
#[derive(Debug, Clone)]
pub struct Client {
    config: Config,
    http_client: reqwest::Client,
}

/// Request configuration for API calls.
#[derive(Debug, Clone)]
pub struct Request {
    /// HTTP method
    pub method: crate::http::Method,
    /// Request URL
    pub url: String,
    /// Optional request body
    pub body: Option<serde_json::Value>,
    /// Additional headers
    pub headers: HeaderMap,
    /// Request timeout override
    pub timeout: Option<Duration>,
    /// Enable retry on failure
    pub retry: bool,
}

/// Response from an API call.
#[derive(Debug, Clone)]
pub struct ApiResponse<T> {
    /// Response body
    pub data: T,
    /// HTTP status code
    pub status: u16,
    /// Response headers
    pub headers: HeaderMap,
    /// PSP reference for tracking
    pub psp_reference: Option<String>,
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
            let header_name =
                reqwest::header::HeaderName::from_bytes(name.as_bytes()).map_err(|e| {
                    crate::AdyenError::config(format!("Invalid header name '{}': {}", name, e))
                })?;
            let header_value = reqwest::header::HeaderValue::from_str(value).map_err(|e| {
                crate::AdyenError::config(format!("Invalid header value '{}': {}", value, e))
            })?;
            headers.insert(header_name, header_value);
        }

        // Add User-Agent
        headers.insert(
            reqwest::header::USER_AGENT,
            reqwest::header::HeaderValue::from_str(config.user_agent())
                .map_err(|e| crate::AdyenError::config(format!("Invalid user agent: {}", e)))?,
        );

        // Add Content-Type
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            reqwest::header::HeaderValue::from_static("application/json"),
        );
        headers.insert(
            reqwest::header::ACCEPT,
            reqwest::header::HeaderValue::from_static("application/json"),
        );

        // Build HTTP client
        let http_client = reqwest::ClientBuilder::new()
            .timeout(config.timeout())
            .default_headers(headers)
            .https_only(true)
            .build()
            .map_err(|e| {
                crate::AdyenError::config(format!("Failed to create HTTP client: {}", e))
            })?;

        Ok(Self {
            config,
            http_client,
        })
    }

    /// Execute a request with automatic retry and error handling.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails after all retry attempts.
    pub async fn execute<T>(&self, request: Request) -> Result<ApiResponse<T>>
    where
        T: for<'de> Deserialize<'de>,
    {
        let max_retries = if request.retry { 3 } else { 1 };
        let mut last_error = None;

        for attempt in 0..max_retries {
            match self.try_request(&request).await {
                Ok(response) => {
                    return self.handle_response::<T>(response).await;
                }
                Err(e) => {
                    last_error = Some(e);

                    if attempt < max_retries - 1 {
                        // Exponential backoff: 100ms, 200ms, 400ms
                        let delay = Duration::from_millis(100 * (1 << attempt));
                        tokio::time::sleep(delay).await;

                        if self.config.is_logging_enabled() {
                            #[cfg(feature = "tracing")]
                            tracing::warn!(
                                "Request failed, retrying in {:?} (attempt {})",
                                delay,
                                attempt + 1
                            );
                        }
                    }
                }
            }
        }

        Err(last_error
            .unwrap_or_else(|| AdyenError::generic("Request failed with no error details")))
    }

    /// Send a POST request with JSON body.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or response cannot be parsed.
    pub async fn post<T, R>(&self, url: &str, body: &T) -> Result<ApiResponse<R>>
    where
        T: Serialize,
        R: for<'de> Deserialize<'de>,
    {
        let request = Request {
            method: crate::http::Method::Post,
            url: url.to_string(),
            body: Some(serde_json::to_value(body)?),
            headers: HeaderMap::new(),
            timeout: None,
            retry: true,
        };

        self.execute(request).await
    }

    /// Send a GET request.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or response cannot be parsed.
    pub async fn get<R>(&self, url: &str) -> Result<ApiResponse<R>>
    where
        R: for<'de> Deserialize<'de>,
    {
        let request = Request {
            method: crate::http::Method::Get,
            url: url.to_string(),
            body: None,
            headers: HeaderMap::new(),
            timeout: None,
            retry: true,
        };

        self.execute(request).await
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

    /// Try to execute a single request attempt.
    async fn try_request(&self, request: &Request) -> Result<Response> {
        let mut req_builder = match request.method {
            crate::http::Method::Get => self.http_client.get(&request.url),
            crate::http::Method::Post => self.http_client.post(&request.url),
            crate::http::Method::Put => self.http_client.put(&request.url),
            crate::http::Method::Patch => self.http_client.patch(&request.url),
            crate::http::Method::Delete => self.http_client.delete(&request.url),
        };

        // Add authentication
        req_builder = self.add_authentication(req_builder)?;

        // Add custom headers
        for (name, value) in &request.headers {
            req_builder = req_builder.header(name, value);
        }

        // Add body if present
        if let Some(body) = &request.body {
            req_builder = req_builder.json(body);
        }

        // Set timeout if specified
        if let Some(timeout) = request.timeout {
            req_builder = req_builder.timeout(timeout);
        }

        // Execute request
        let response = req_builder.send().await?;
        Ok(response)
    }

    /// Add authentication headers to the request.
    fn add_authentication(&self, mut req_builder: RequestBuilder) -> Result<RequestBuilder> {
        match self.config.credentials() {
            Credentials::ApiKey(api_key) => {
                req_builder = req_builder.header("X-API-Key", api_key.as_str());
            }
            Credentials::Basic(basic_auth) => {
                req_builder =
                    req_builder.header("Authorization", basic_auth.authorization_header());
            }
        }
        Ok(req_builder)
    }

    /// Handle the HTTP response and convert to ApiResponse.
    async fn handle_response<T>(&self, response: Response) -> Result<ApiResponse<T>>
    where
        T: for<'de> Deserialize<'de>,
    {
        let status = response.status().as_u16();
        let headers = response.headers().clone();

        // Extract PSP reference from headers
        let psp_reference = headers
            .get("psp-reference")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());

        let response_text = response.text().await?;

        // Log response if enabled
        if self.config.is_logging_enabled() {
            #[cfg(feature = "tracing")]
            tracing::debug!("Response status: {}, body: {}", status, response_text);
        }

        // Handle error responses
        if status >= 400 {
            let api_error = self.parse_api_error(&response_text, status, psp_reference)?;
            return Err(api_error);
        }

        // Parse successful response
        let data: T = serde_json::from_str(&response_text).map_err(|e| {
            AdyenError::generic_with_source(
                format!("Failed to parse response: {}", response_text),
                Box::new(e),
            )
        })?;

        Ok(ApiResponse {
            data,
            status,
            headers,
            psp_reference,
        })
    }

    /// Parse API error from response text.
    fn parse_api_error(
        &self,
        response_text: &str,
        status: u16,
        psp_reference: Option<String>,
    ) -> Result<AdyenError> {
        // Try to parse structured error response
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct ErrorResponse {
            status: Option<u16>,
            error_code: Option<String>,
            message: Option<String>,
            error_type: Option<String>,
            psp_reference: Option<String>,
        }

        match serde_json::from_str::<ErrorResponse>(response_text) {
            Ok(error_resp) => Ok(AdyenError::api(
                error_resp.status.unwrap_or(status),
                error_resp
                    .error_code
                    .unwrap_or_else(|| "UNKNOWN_ERROR".to_string()),
                error_resp
                    .message
                    .unwrap_or_else(|| "Unknown error".to_string()),
                error_resp
                    .error_type
                    .unwrap_or_else(|| "UNKNOWN".to_string()),
                error_resp.psp_reference.or(psp_reference),
            )),
            Err(_) => {
                // Fallback for non-structured errors
                Ok(AdyenError::api(
                    status,
                    format!("HTTP_{}", status),
                    response_text.to_string(),
                    "HTTP_ERROR".to_string(),
                    psp_reference,
                ))
            }
        }
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

    #[test]
    fn test_request_builder() {
        let request = Request {
            method: crate::http::Method::Post,
            url: "https://checkout-test.adyen.com/v71/payments".to_string(),
            body: Some(serde_json::json!({"amount": {"value": 1000, "currency": "EUR"}})),
            headers: reqwest::header::HeaderMap::new(),
            timeout: Some(std::time::Duration::from_secs(30)),
            retry: true,
        };

        assert_eq!(request.url, "https://checkout-test.adyen.com/v71/payments");
        assert!(request.retry);
        assert!(request.body.is_some());
    }

    #[test]
    fn test_api_response() {
        let response = ApiResponse {
            data: serde_json::json!({"status": "success"}),
            status: 200,
            headers: reqwest::header::HeaderMap::new(),
            psp_reference: Some("12345678901234567890".to_string()),
        };

        assert_eq!(response.status, 200);
        assert!(response.psp_reference.is_some());
    }
}
