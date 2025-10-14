//! Card details types for card validation and brand detection.

use adyen_core::{AdyenError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Request to get card details including brand and validation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardDetailsRequest {
    /// The card number to validate.
    pub card_number: String,

    /// The merchant account identifier.
    pub merchant_account: String,

    /// Supported brands to check against.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_brands: Option<Vec<String>>,

    /// The country code for country-specific validation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
}

/// Response containing card details and validation information.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardDetailsResponse {
    /// The detected card brand.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand: Option<CardBrand>,

    /// Whether the card number is valid.
    pub is_valid: bool,

    /// Supported features for this card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_features: Option<Vec<String>>,

    /// Funding source of the card (debit, credit, etc.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funding_source: Option<FundingSource>,

    /// Additional details about the card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<HashMap<String, String>>,
}

/// Card brand information.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardBrand {
    /// The brand type (visa, mc, amex, etc.).
    #[serde(rename = "type")]
    pub brand_type: String,

    /// Whether this brand is supported.
    pub supported: bool,

    /// CVC policy for this brand.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvc_policy: Option<CvcPolicy>,

    /// Whether this brand supports 3D Secure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_luhn_check: Option<bool>,

    /// Additional brand-specific data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<HashMap<String, String>>,
}

/// CVC policy for a card brand.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CvcPolicy {
    /// CVC is required.
    Required,
    /// CVC is optional.
    Optional,
    /// CVC is hidden/not used.
    Hidden,
}

/// Funding source of a card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FundingSource {
    /// Credit card.
    Credit,
    /// Debit card.
    Debit,
    /// Prepaid card.
    Prepaid,
    /// Charge card.
    Charge,
    /// Unknown funding source.
    Unknown,
}

/// Builder for creating card details requests.
#[derive(Debug, Clone, Default)]
pub struct CardDetailsRequestBuilder {
    card_number: Option<String>,
    merchant_account: Option<String>,
    supported_brands: Option<Vec<String>>,
    country_code: Option<String>,
}

impl CardDetailsRequestBuilder {
    /// Create a new builder.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the card number.
    #[must_use]
    pub fn card_number(mut self, card_number: impl Into<String>) -> Self {
        self.card_number = Some(card_number.into());
        self
    }

    /// Set the merchant account.
    #[must_use]
    pub fn merchant_account(mut self, merchant_account: impl Into<String>) -> Self {
        self.merchant_account = Some(merchant_account.into());
        self
    }

    /// Add a supported brand.
    #[must_use]
    pub fn supported_brand(mut self, brand: impl Into<String>) -> Self {
        self.supported_brands
            .get_or_insert_with(Vec::new)
            .push(brand.into());
        self
    }

    /// Set supported brands.
    #[must_use]
    pub fn supported_brands(mut self, brands: Vec<String>) -> Self {
        self.supported_brands = Some(brands);
        self
    }

    /// Set the country code.
    #[must_use]
    pub fn country_code(mut self, country_code: impl Into<String>) -> Self {
        self.country_code = Some(country_code.into());
        self
    }

    /// Build the card details request.
    ///
    /// # Errors
    ///
    /// Returns an error if required fields are not set.
    pub fn build(self) -> Result<CardDetailsRequest> {
        let card_number = self.card_number
            .ok_or_else(|| AdyenError::config("card_number is required"))?;
        let merchant_account = self.merchant_account
            .ok_or_else(|| AdyenError::config("merchant_account is required"))?;

        Ok(CardDetailsRequest {
            card_number,
            merchant_account,
            supported_brands: self.supported_brands,
            country_code: self.country_code,
        })
    }
}

impl CardDetailsRequest {
    /// Create a new builder for card details requests.
    #[must_use]
    pub fn builder() -> CardDetailsRequestBuilder {
        CardDetailsRequestBuilder::new()
    }

    /// Create a simple card details request with just card number and merchant account.
    ///
    /// # Errors
    ///
    /// Returns an error if the card number is empty.
    pub fn simple(card_number: impl Into<String>, merchant_account: impl Into<String>) -> Result<Self> {
        let card_number = card_number.into();
        if card_number.is_empty() {
            return Err(AdyenError::config("card_number cannot be empty"));
        }

        Ok(Self {
            card_number,
            merchant_account: merchant_account.into(),
            supported_brands: None,
            country_code: None,
        })
    }
}

/// Common card brand constants.
pub mod brands {
    /// Visa card brand.
    pub const VISA: &str = "visa";
    /// Mastercard brand.
    pub const MASTERCARD: &str = "mc";
    /// American Express brand.
    pub const AMEX: &str = "amex";
    /// Maestro brand.
    pub const MAESTRO: &str = "maestro";
    /// Diners Club brand.
    pub const DINERS: &str = "diners";
    /// Discover brand.
    pub const DISCOVER: &str = "discover";
    /// JCB brand.
    pub const JCB: &str = "jcb";
    /// UnionPay brand.
    pub const UNIONPAY: &str = "cup";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_details_request_builder() {
        let request = CardDetailsRequest::builder()
            .card_number("4111111111111111")
            .merchant_account("TestMerchant")
            .supported_brand(brands::VISA)
            .supported_brand(brands::MASTERCARD)
            .country_code("NL")
            .build()
            .unwrap();

        assert_eq!(request.card_number, "4111111111111111");
        assert_eq!(request.merchant_account, "TestMerchant");
        assert_eq!(request.supported_brands.as_ref().unwrap().len(), 2);
        assert_eq!(request.country_code, Some("NL".to_string()));
    }

    #[test]
    fn test_card_details_request_simple() {
        let request = CardDetailsRequest::simple("4111111111111111", "TestMerchant").unwrap();

        assert_eq!(request.card_number, "4111111111111111");
        assert_eq!(request.merchant_account, "TestMerchant");
        assert!(request.supported_brands.is_none());
        assert!(request.country_code.is_none());
    }

    #[test]
    fn test_card_details_request_empty_card_number() {
        let result = CardDetailsRequest::simple("", "TestMerchant");
        assert!(result.is_err());
    }

    #[test]
    fn test_funding_source_serialization() {
        assert_eq!(serde_json::to_string(&FundingSource::Credit).unwrap(), "\"credit\"");
        assert_eq!(serde_json::to_string(&FundingSource::Debit).unwrap(), "\"debit\"");
        assert_eq!(serde_json::to_string(&FundingSource::Prepaid).unwrap(), "\"prepaid\"");
    }

    #[test]
    fn test_cvc_policy_serialization() {
        assert_eq!(serde_json::to_string(&CvcPolicy::Required).unwrap(), "\"required\"");
        assert_eq!(serde_json::to_string(&CvcPolicy::Optional).unwrap(), "\"optional\"");
        assert_eq!(serde_json::to_string(&CvcPolicy::Hidden).unwrap(), "\"hidden\"");
    }

    #[test]
    fn test_brand_constants() {
        assert_eq!(brands::VISA, "visa");
        assert_eq!(brands::MASTERCARD, "mc");
        assert_eq!(brands::AMEX, "amex");
    }
}