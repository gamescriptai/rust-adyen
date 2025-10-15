//! HMAC signature validation for Adyen webhooks.
//!
//! This module provides utilities to validate webhook authenticity using HMAC-SHA256 signatures.
//! Adyen uses HMAC signatures to ensure webhooks are genuine and haven't been tampered with.

use crate::types::NotificationRequestItem;
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::collections::HashMap;

type HmacSha256 = Hmac<Sha256>;

/// Error types for webhook validation.
#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    /// Invalid HMAC key format.
    #[error("Invalid HMAC key: {0}")]
    InvalidKey(String),
    /// HMAC calculation failed.
    #[error("HMAC calculation failed: {0}")]
    HmacError(String),
    /// Missing HMAC signature in webhook data.
    #[error("HMAC signature not found in additional data")]
    MissingSignature,
}

/// HMAC validator for Adyen webhooks.
///
/// This validator provides methods to verify webhook authenticity using HMAC-SHA256 signatures.
/// It supports both payload-based validation (for HTTP header signatures) and additional-data
/// based validation (for signatures embedded in the webhook payload).
#[derive(Debug)]
pub struct HmacValidator {
    secret_key: Vec<u8>,
}

impl HmacValidator {
    /// Create a new HMAC validator with the given secret key.
    ///
    /// The secret key should be the hex-encoded HMAC key provided by Adyen.
    ///
    /// # Errors
    ///
    /// Returns an error if the secret key is not valid hex.
    pub fn new(secret_key: &str) -> Result<Self, ValidationError> {
        let key = hex::decode(secret_key)
            .map_err(|e| ValidationError::InvalidKey(format!("Invalid hex key: {e}")))?;

        Ok(Self { secret_key: key })
    }

    /// Validate HMAC signature for a notification request item.
    ///
    /// This method is used for webhooks that include the HMAC signature in the
    /// `additionalData` field (e.g., standard payment webhooks).
    ///
    /// # Arguments
    ///
    /// * `item` - The notification request item to validate
    ///
    /// # Returns
    ///
    /// Returns `true` if the signature is valid, `false` otherwise.
    #[must_use]
    pub fn validate_notification(&self, item: &NotificationRequestItem) -> bool {
        // Get the HMAC signature from additional data
        let Some(signature) = item.hmac_signature() else {
            return false;
        };

        // Calculate expected signature
        let Ok(expected_signature) = self.calculate_notification_signature(item) else {
            return false;
        };

        // Compare signatures
        signature == expected_signature
    }

    /// Validate HMAC signature for a raw payload.
    ///
    /// This method is used for webhooks that provide the HMAC signature in
    /// HTTP headers (e.g., Banking API, Management API webhooks).
    ///
    /// # Arguments
    ///
    /// * `payload` - The raw webhook payload
    /// * `signature` - The HMAC signature from HTTP headers
    ///
    /// # Returns
    ///
    /// Returns `true` if the signature is valid, `false` otherwise.
    #[must_use]
    pub fn validate_payload(&self, payload: &str, signature: &str) -> bool {
        match self.calculate_payload_signature(payload) {
            Ok(expected_signature) => signature == expected_signature,
            Err(_) => false,
        }
    }

    /// Calculate HMAC signature for a notification request item.
    ///
    /// This creates the data-to-sign string and calculates the HMAC signature
    /// following Adyen's specification.
    ///
    /// # Errors
    ///
    /// Returns an error if HMAC calculation fails.
    pub fn calculate_notification_signature(
        &self,
        item: &NotificationRequestItem,
    ) -> Result<String, ValidationError> {
        let data_to_sign = self.get_notification_data_to_sign(item);
        self.calculate_hmac(&data_to_sign)
    }

    /// Calculate HMAC signature for a raw payload.
    ///
    /// # Errors
    ///
    /// Returns an error if HMAC calculation fails.
    pub fn calculate_payload_signature(&self, payload: &str) -> Result<String, ValidationError> {
        self.calculate_hmac(payload)
    }

    /// Get the data-to-sign string for a notification request item.
    ///
    /// This method constructs the canonical string that Adyen uses for HMAC calculation,
    /// following the exact format: pspReference:originalReference:merchantAccountCode:
    /// merchantReference:amount:currency:eventCode:success
    #[allow(clippy::unused_self)]
    fn get_notification_data_to_sign(&self, item: &NotificationRequestItem) -> String {
        let original_reference = item.original_reference.as_deref().unwrap_or("");

        format!(
            "{}:{}:{}:{}:{}:{}:{}:{}",
            item.psp_reference,
            original_reference,
            item.merchant_account_code,
            item.merchant_reference,
            item.amount.minor_units(),
            item.amount.currency_string(),
            item.event_code,
            item.success
        )
    }

    /// Calculate HMAC-SHA256 signature for the given data.
    fn calculate_hmac(&self, data: &str) -> Result<String, ValidationError> {
        let mut mac = HmacSha256::new_from_slice(&self.secret_key)
            .map_err(|e| ValidationError::HmacError(format!("Failed to create HMAC: {e}")))?;

        // Apply escaping for backslashes and colons as per Adyen specification
        let escaped_data = self.escape_data(data);
        mac.update(escaped_data.as_bytes());

        let result = mac.finalize();
        Ok(BASE64.encode(result.into_bytes()))
    }

    /// Escape backslashes and colons in data as required by Adyen HMAC specification.
    ///
    /// This matches the escaping logic from the Go library:
    /// - Backslashes are escaped as `\\`
    /// - Colons are escaped as `\:`
    #[allow(clippy::unused_self)]
    fn escape_data(&self, data: &str) -> String {
        data.replace('\\', "\\\\").replace(':', "\\:")
    }

    /// Validate HMAC signature for a map of key-value pairs.
    ///
    /// This method is used for webhooks that provide data as key-value pairs
    /// rather than structured notification items.
    #[must_use]
    pub fn validate_key_value_pairs(
        &self,
        data: &HashMap<String, String>,
        signature: &str,
    ) -> bool {
        match self.calculate_key_value_signature(data) {
            Ok(expected_signature) => signature == expected_signature,
            Err(_) => false,
        }
    }

    /// Calculate HMAC signature for key-value pairs.
    ///
    /// This method follows Adyen's specification for signing key-value data:
    /// 1. Sort keys alphabetically
    /// 2. Escape keys and values
    /// 3. Create signature string: "key1:key2:...:value1:value2:..."
    fn calculate_key_value_signature(
        &self,
        data: &HashMap<String, String>,
    ) -> Result<String, ValidationError> {
        let mut keys: Vec<&String> = data.keys().collect();
        keys.sort();

        let escaped_keys: Vec<String> = keys.iter().map(|k| self.escape_data(k)).collect();

        let escaped_values: Vec<String> =
            keys.iter().map(|k| self.escape_data(&data[*k])).collect();

        let data_to_sign = format!("{}:{}", escaped_keys.join(":"), escaped_values.join(":"));

        self.calculate_hmac(&data_to_sign)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Amount, NotificationRequestItem};
    use std::collections::HashMap;

    const TEST_HMAC_KEY: &str = "44782DEF547AAA06C910C43932B1EB0C71FC68D9D0C057550C48EC2ACF6BA056";

    #[test]
    fn test_hmac_validator_creation() {
        let validator = HmacValidator::new(TEST_HMAC_KEY).unwrap();
        assert_eq!(validator.secret_key.len(), 32); // 256 bits = 32 bytes
    }

    #[test]
    fn test_invalid_hmac_key() {
        let result = HmacValidator::new("invalid_hex_key");
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ValidationError::InvalidKey(_)
        ));
    }

    #[test]
    fn test_data_to_sign_generation() {
        let validator = HmacValidator::new(TEST_HMAC_KEY).unwrap();

        let item = NotificationRequestItem {
            additional_data: None,
            amount: Amount::new(1000, "EUR"),
            event_code: "AUTHORISATION".to_string(),
            event_date: None,
            merchant_account_code: "TestMerchant".to_string(),
            merchant_reference: "test-payment-123".to_string(),
            operations: vec![],
            original_reference: Some("original-123".to_string()),
            payment_method: "visa".to_string(),
            psp_reference: "8515131751004933".to_string(),
            reason: "test".to_string(),
            success: "true".to_string(),
        };

        let data_to_sign = validator.get_notification_data_to_sign(&item);
        let expected = "8515131751004933:original-123:TestMerchant:test-payment-123:1000:EUR:AUTHORISATION:true";
        assert_eq!(data_to_sign, expected);
    }

    #[test]
    fn test_data_escaping() {
        let validator = HmacValidator::new(TEST_HMAC_KEY).unwrap();

        let test_data = "test\\data:with:special\\chars";
        let escaped = validator.escape_data(test_data);
        assert_eq!(escaped, "test\\\\data\\:with\\:special\\\\chars");
    }

    #[test]
    fn test_notification_signature_calculation() {
        let validator = HmacValidator::new(TEST_HMAC_KEY).unwrap();

        let item = NotificationRequestItem {
            additional_data: None,
            amount: Amount::new(1000, "EUR"),
            event_code: "AUTHORISATION".to_string(),
            event_date: None,
            merchant_account_code: "TestMerchant".to_string(),
            merchant_reference: "test-payment-123".to_string(),
            operations: vec![],
            original_reference: None,
            payment_method: "visa".to_string(),
            psp_reference: "8515131751004933".to_string(),
            reason: "test".to_string(),
            success: "true".to_string(),
        };

        let signature = validator.calculate_notification_signature(&item).unwrap();

        // Signature should be a valid base64 string
        assert!(BASE64.decode(&signature).is_ok());

        // Should be deterministic
        let signature2 = validator.calculate_notification_signature(&item).unwrap();
        assert_eq!(signature, signature2);
    }

    #[test]
    fn test_notification_validation_with_valid_signature() {
        let validator = HmacValidator::new(TEST_HMAC_KEY).unwrap();

        let mut additional_data = HashMap::new();
        let expected_signature = validator
            .calculate_notification_signature(&NotificationRequestItem {
                additional_data: None,
                amount: Amount::new(1000, "EUR"),
                event_code: "AUTHORISATION".to_string(),
                event_date: None,
                merchant_account_code: "TestMerchant".to_string(),
                merchant_reference: "test-payment-123".to_string(),
                operations: vec![],
                original_reference: None,
                payment_method: "visa".to_string(),
                psp_reference: "8515131751004933".to_string(),
                reason: "test".to_string(),
                success: "true".to_string(),
            })
            .unwrap();

        additional_data.insert(
            "hmacSignature".to_string(),
            serde_json::Value::String(expected_signature),
        );

        let item = NotificationRequestItem {
            additional_data: Some(additional_data),
            amount: Amount::new(1000, "EUR"),
            event_code: "AUTHORISATION".to_string(),
            event_date: None,
            merchant_account_code: "TestMerchant".to_string(),
            merchant_reference: "test-payment-123".to_string(),
            operations: vec![],
            original_reference: None,
            payment_method: "visa".to_string(),
            psp_reference: "8515131751004933".to_string(),
            reason: "test".to_string(),
            success: "true".to_string(),
        };

        assert!(validator.validate_notification(&item));
    }

    #[test]
    fn test_notification_validation_with_invalid_signature() {
        let validator = HmacValidator::new(TEST_HMAC_KEY).unwrap();

        let mut additional_data = HashMap::new();
        additional_data.insert(
            "hmacSignature".to_string(),
            serde_json::Value::String("invalid_signature".to_string()),
        );

        let item = NotificationRequestItem {
            additional_data: Some(additional_data),
            amount: Amount::new(1000, "EUR"),
            event_code: "AUTHORISATION".to_string(),
            event_date: None,
            merchant_account_code: "TestMerchant".to_string(),
            merchant_reference: "test-payment-123".to_string(),
            operations: vec![],
            original_reference: None,
            payment_method: "visa".to_string(),
            psp_reference: "8515131751004933".to_string(),
            reason: "test".to_string(),
            success: "true".to_string(),
        };

        assert!(!validator.validate_notification(&item));
    }

    #[test]
    fn test_payload_validation() {
        let validator = HmacValidator::new(TEST_HMAC_KEY).unwrap();

        let payload = r#"{"test": "data"}"#;
        let expected_signature = validator.calculate_payload_signature(payload).unwrap();

        assert!(validator.validate_payload(payload, &expected_signature));
        assert!(!validator.validate_payload(payload, "invalid_signature"));
    }

    #[test]
    fn test_key_value_signature_calculation() {
        let validator = HmacValidator::new(TEST_HMAC_KEY).unwrap();

        let mut data = HashMap::new();
        data.insert("key2".to_string(), "value2".to_string());
        data.insert("key1".to_string(), "value1".to_string());
        data.insert("key3".to_string(), "value3".to_string());

        let signature = validator.calculate_key_value_signature(&data).unwrap();

        // Should be deterministic (keys are sorted)
        let signature2 = validator.calculate_key_value_signature(&data).unwrap();
        assert_eq!(signature, signature2);

        // Validate the signature
        assert!(validator.validate_key_value_pairs(&data, &signature));
    }
}
