//! Additional types for extended Checkout API functionality.

use adyen_core::Amount;
use serde::{Deserialize, Serialize};

/// Response from getting session result.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionResultResponse {
    /// A unique identifier of the session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The status of the session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// Response containing stored payment methods for a shopper.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListStoredPaymentMethodsResponse {
    /// Your merchant account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_account: Option<String>,
    /// Your reference to uniquely identify this shopper.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shopper_reference: Option<String>,
    /// List of all stored payment methods.
    #[serde(default)]
    pub stored_payment_methods: Vec<StoredPaymentMethodResource>,
}

/// Stored payment method resource.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StoredPaymentMethodResource {
    /// The unique payment method code.
    pub type_: String,
    /// Unique identifier of this stored payment method.
    pub id: String,
    /// The display name of the stored payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// A shopper's contact details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shopper_email: Option<String>,
    /// Additional stored payment method details.
    #[serde(flatten)]
    pub details: serde_json::Value,
}

/// Request for payment method balance check.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BalanceCheckRequest {
    /// The merchant account identifier.
    pub merchant_account: String,
    /// The payment method for balance check.
    pub payment_method: serde_json::Value,
}

/// Response from balance check request.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BalanceCheckResponse {
    /// The balance amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance: Option<Amount>,
    /// Transaction limit for this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_limit: Option<Amount>,
}

/// Request for creating payment links.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentLinkRequest {
    /// The amount information for the transaction.
    pub amount: Amount,
    /// The merchant account identifier.
    pub merchant_account: String,
    /// A reference to uniquely identify the payment.
    pub reference: String,
    /// The URL to return to after payment completion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
    /// Expiry date for the payment link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    /// Description for the payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Response from creating payment links.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentLinkResponse {
    /// Unique identifier of the payment link.
    pub id: String,
    /// The amount information for the transaction.
    pub amount: Amount,
    /// The merchant account identifier.
    pub merchant_account: String,
    /// A reference to uniquely identify the payment.
    pub reference: String,
    /// Status of the payment link.
    pub status: String,
    /// URL of the payment link.
    pub url: String,
    /// Expiry date for the payment link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
}

/// Request for getting Apple Pay session.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplePaySessionRequest {
    /// The merchant identifier for Apple Pay.
    pub merchant_identifier: String,
    /// The display name shown to users.
    pub display_name: String,
    /// The domain name for validation.
    pub domain_name: String,
}

/// Response from Apple Pay session request.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplePaySessionResponse {
    /// The Apple Pay session data.
    pub data: String,
}

/// Request for getting origin keys.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OriginKeysRequest {
    /// List of origin URLs for which to generate keys.
    pub origin_domains: Vec<String>,
}

/// Response containing origin keys.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OriginKeysResponse {
    /// Origin keys mapped by domain.
    pub origin_keys: std::collections::HashMap<String, String>,
}
