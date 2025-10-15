//! # Adyen Webhooks v1
//!
//! This crate provides secure webhook processing and HMAC validation for Adyen webhooks.
//! It includes support for all webhook types, HMAC signature verification, and type-safe
//! event handling.
//!
//! ## Features
//!
//! - **HMAC Signature Validation**: Verify webhook authenticity using SHA-256 HMAC
//! - **Type-Safe Event Handling**: Strongly typed webhook events and data structures
//! - **Multiple Validation Methods**: Support for both payload and additional-data signatures
//! - **Complete Event Coverage**: All Adyen webhook event types supported
//! - **Zero-Copy Processing**: Optional rkyv serialization for performance
//!
//! ## Quick Start
//!
//! ```rust
//! use adyen_webhooks::{HmacValidator, Webhook};
//!
//! # fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Create validator with your HMAC key
//! let validator = HmacValidator::new("your_hmac_key_in_hex")?;
//!
//! // Parse incoming webhook
//! let webhook_json = r#"{"live": "false", "notificationItems": [...]}"#;
//! let webhook: Webhook = serde_json::from_str(webhook_json)?;
//!
//! // Validate each notification item
//! for item in webhook.get_notification_items() {
//!     if validator.validate_notification(item) {
//!         println!("Valid webhook: {}", item.event_code);
//!         // Process the webhook...
//!     } else {
//!         println!("Invalid webhook signature!");
//!     }
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## HMAC Validation
//!
//! Adyen uses HMAC-SHA256 signatures to ensure webhook authenticity. This crate supports
//! both validation methods:
//!
//! ### Additional Data Signature (Standard Webhooks)
//!
//! Most webhooks include the HMAC signature in the `additionalData` field:
//!
//! ```rust
//! # use adyen_webhooks::HmacValidator;
//! # fn example(validator: HmacValidator, notification_item: adyen_webhooks::NotificationRequestItem) {
//! if validator.validate_notification(&notification_item) {
//!     // Webhook is authentic
//! }
//! # }
//! ```
//!
//! ### HTTP Header Signature (Management/Banking APIs)
//!
//! Some webhooks provide the signature in HTTP headers:
//!
//! ```rust
//! # use adyen_webhooks::HmacValidator;
//! # fn example(validator: HmacValidator, payload: &str, signature: &str) {
//! if validator.validate_payload(payload, signature) {
//!     // Webhook is authentic
//! }
//! # }
//! ```

#![deny(missing_docs)]
#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

pub mod types;
pub mod validation;

// Re-export main types for convenience
pub use types::{
    EventCode, NotificationItem, NotificationRequestItem, Webhook,
};
pub use validation::{HmacValidator, ValidationError};

/// Handle and parse a webhook request from JSON.
///
/// This convenience function parses a webhook JSON payload into a structured
/// `Webhook` object. It does not perform HMAC validation - use `HmacValidator`
/// for signature verification.
///
/// # Arguments
///
/// * `json_payload` - The raw JSON webhook payload
///
/// # Returns
///
/// Returns a parsed `Webhook` object or a JSON parsing error.
///
/// # Example
///
/// ```rust
/// use adyen_webhooks::handle_webhook;
///
/// let webhook_json = r#"{
///     "live": "false",
///     "notificationItems": [
///         {
///             "NotificationRequestItem": {
///                 "amount": {"currency": "EUR", "value": 1000},
///                 "eventCode": "AUTHORISATION",
///                 "merchantAccountCode": "TestMerchant",
///                 "merchantReference": "test-123",
///                 "operations": [],
///                 "paymentMethod": "visa",
///                 "pspReference": "8515131751004933",
///                 "reason": "Approved",
///                 "success": "true"
///             }
///         }
///     ]
/// }"#;
///
/// let webhook = handle_webhook(webhook_json).unwrap();
/// assert_eq!(webhook.notification_items.len(), 1);
/// ```
pub fn handle_webhook(json_payload: &str) -> Result<Webhook, serde_json::Error> {
    serde_json::from_str(json_payload)
}

/// Current version of the Adyen Webhooks library.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_webhook() {
        let webhook_json = r#"{
            "live": "false",
            "notificationItems": [
                {
                    "NotificationRequestItem": {
                        "amount": {
                            "currency": "EUR",
                            "value": 1000
                        },
                        "eventCode": "AUTHORISATION",
                        "merchantAccountCode": "TestMerchant",
                        "merchantReference": "test-payment-123",
                        "operations": ["CAPTURE", "REFUND"],
                        "paymentMethod": "visa",
                        "pspReference": "8515131751004933",
                        "reason": "Approved",
                        "success": "true"
                    }
                }
            ]
        }"#;

        let webhook = handle_webhook(webhook_json).unwrap();
        assert!(!webhook.is_live());
        assert_eq!(webhook.notification_items.len(), 1);

        let item = &webhook.notification_items[0].notification_request_item;
        assert_eq!(item.event_code, "AUTHORISATION");
        assert!(item.is_success());
    }

    #[test]
    fn test_invalid_webhook_json() {
        let result = handle_webhook("invalid json");
        assert!(result.is_err());
    }

    #[test]
    fn test_library_version() {
        assert!(!VERSION.is_empty());
    }
}