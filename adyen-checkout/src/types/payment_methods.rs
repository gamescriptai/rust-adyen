//! Payment methods request and response types.

use adyen_core::{Amount, AdyenError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Request to retrieve available payment methods.
///
/// This request is used to get the list of payment methods available
/// for a specific merchant account, amount, country, and shopper.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentMethodsRequest {
    /// The merchant account identifier.
    pub merchant_account: String,

    /// The transaction amount and currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<Amount>,

    /// The shopper's country code in ISO 3166-1 alpha-2 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,

    /// The shopper's locale in BCP 47 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shopper_locale: Option<String>,

    /// The sales channel for the transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<Channel>,

    /// Whether to include stored payment methods.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store_payment_method: Option<bool>,

    /// The shopper reference for stored payment methods.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shopper_reference: Option<String>,

    /// Additional data for the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<HashMap<String, String>>,
}

/// The sales channel for the transaction.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum Channel {
    /// Online/web channel
    Web,
    /// iOS application
    #[serde(rename = "iOS")]
    Ios,
    /// Android application
    Android,
}

/// Response containing available payment methods.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentMethodsResponse {
    /// List of available payment methods.
    pub payment_methods: Vec<PaymentMethod>,

    /// List of stored payment methods for the shopper.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stored_payment_methods: Option<Vec<StoredPaymentMethod>>,

    /// Groups of payment methods.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<PaymentMethodGroup>>,
}

/// A payment method that can be used for transactions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentMethod {
    /// The payment method type (e.g., "scheme", "ideal", "paypal").
    #[serde(rename = "type")]
    pub payment_method_type: String,

    /// The display name of the payment method.
    pub name: String,

    /// List of supported brands (for card payments).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brands: Option<Vec<String>>,

    /// Configuration details for the payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<PaymentMethodConfiguration>,

    /// Funding source information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funding_source: Option<String>,

    /// Whether the payment method supports recurring transactions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_recurring: Option<bool>,
}

/// Configuration details for a payment method.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentMethodConfiguration {
    /// Available currencies for this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currencies: Option<Vec<String>>,

    /// Additional configuration data.
    #[serde(flatten)]
    pub additional_data: HashMap<String, serde_json::Value>,
}

/// A stored payment method for a shopper.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StoredPaymentMethod {
    /// Unique identifier for the stored payment method.
    pub id: String,

    /// The payment method type.
    #[serde(rename = "type")]
    pub payment_method_type: String,

    /// Display name for the stored payment method.
    pub name: String,

    /// The brand of the stored payment method (for cards).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand: Option<String>,

    /// Last four digits of the card number (for cards).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_four: Option<String>,

    /// Expiry month of the card (for cards).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_month: Option<String>,

    /// Expiry year of the card (for cards).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_year: Option<String>,

    /// The shopper email associated with this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shopper_email: Option<String>,

    /// Whether the stored payment method supports recurring transactions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_recurring: Option<bool>,
}

/// A group of related payment methods.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentMethodGroup {
    /// The group name.
    pub name: String,

    /// The group type.
    #[serde(rename = "type")]
    pub group_type: String,

    /// Payment methods in this group.
    pub types: Vec<String>,
}

/// Builder for creating payment methods requests.
#[derive(Debug, Clone, Default)]
pub struct PaymentMethodsRequestBuilder {
    merchant_account: Option<String>,
    amount: Option<Amount>,
    country_code: Option<String>,
    shopper_locale: Option<String>,
    channel: Option<Channel>,
    store_payment_method: Option<bool>,
    shopper_reference: Option<String>,
    additional_data: Option<HashMap<String, String>>,
}

impl PaymentMethodsRequestBuilder {
    /// Create a new builder.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the merchant account identifier.
    #[must_use]
    pub fn merchant_account(mut self, merchant_account: impl Into<String>) -> Self {
        self.merchant_account = Some(merchant_account.into());
        self
    }

    /// Set the transaction amount and currency.
    #[must_use]
    pub fn amount(mut self, amount: Amount) -> Self {
        self.amount = Some(amount);
        self
    }

    /// Set the shopper's country code.
    #[must_use]
    pub fn country_code(mut self, country_code: impl Into<String>) -> Self {
        self.country_code = Some(country_code.into());
        self
    }

    /// Set the shopper's locale.
    #[must_use]
    pub fn shopper_locale(mut self, locale: impl Into<String>) -> Self {
        self.shopper_locale = Some(locale.into());
        self
    }

    /// Set the sales channel.
    #[must_use]
    pub fn channel(mut self, channel: Channel) -> Self {
        self.channel = Some(channel);
        self
    }

    /// Set whether to include stored payment methods.
    #[must_use]
    pub fn store_payment_method(mut self, store: bool) -> Self {
        self.store_payment_method = Some(store);
        self
    }

    /// Set the shopper reference for stored payment methods.
    #[must_use]
    pub fn shopper_reference(mut self, reference: impl Into<String>) -> Self {
        self.shopper_reference = Some(reference.into());
        self
    }

    /// Add additional data.
    #[must_use]
    pub fn additional_data(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.additional_data
            .get_or_insert_with(HashMap::new)
            .insert(key.into(), value.into());
        self
    }

    /// Build the payment methods request.
    ///
    /// # Errors
    ///
    /// Returns an error if the merchant account is not set.
    pub fn build(self) -> Result<PaymentMethodsRequest> {
        let merchant_account = self.merchant_account
            .ok_or_else(|| AdyenError::config("merchant_account is required"))?;

        Ok(PaymentMethodsRequest {
            merchant_account,
            amount: self.amount,
            country_code: self.country_code,
            shopper_locale: self.shopper_locale,
            channel: self.channel,
            store_payment_method: self.store_payment_method,
            shopper_reference: self.shopper_reference,
            additional_data: self.additional_data,
        })
    }
}

impl PaymentMethodsRequest {
    /// Create a new builder for payment methods requests.
    #[must_use]
    pub fn builder() -> PaymentMethodsRequestBuilder {
        PaymentMethodsRequestBuilder::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use adyen_core::{Amount, Currency};

    #[test]
    fn test_payment_methods_request_builder() {
        let amount = Amount::from_major_units(100, Currency::EUR);

        let request = PaymentMethodsRequest::builder()
            .merchant_account("TestMerchant")
            .amount(amount.clone())
            .country_code("NL")
            .shopper_locale("nl-NL")
            .channel(Channel::Web)
            .store_payment_method(true)
            .shopper_reference("test-shopper")
            .additional_data("testKey", "testValue")
            .build()
            .unwrap();

        assert_eq!(request.merchant_account, "TestMerchant");
        assert_eq!(request.amount, Some(amount));
        assert_eq!(request.country_code, Some("NL".to_string()));
        assert_eq!(request.shopper_locale, Some("nl-NL".to_string()));
        assert_eq!(request.channel, Some(Channel::Web));
        assert_eq!(request.store_payment_method, Some(true));
        assert_eq!(request.shopper_reference, Some("test-shopper".to_string()));
        assert!(request.additional_data.is_some());
    }

    #[test]
    fn test_payment_methods_request_minimal() {
        let request = PaymentMethodsRequest::builder()
            .merchant_account("TestMerchant")
            .build()
            .unwrap();

        assert_eq!(request.merchant_account, "TestMerchant");
        assert!(request.amount.is_none());
        assert!(request.country_code.is_none());
    }

    #[test]
    fn test_payment_methods_request_missing_merchant_account() {
        let result = PaymentMethodsRequest::builder().build();
        assert!(result.is_err());
    }

    #[test]
    fn test_channel_serialization() {
        assert_eq!(serde_json::to_string(&Channel::Web).unwrap(), "\"Web\"");
        assert_eq!(serde_json::to_string(&Channel::Ios).unwrap(), "\"iOS\"");
        assert_eq!(serde_json::to_string(&Channel::Android).unwrap(), "\"Android\"");
    }
}