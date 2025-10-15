//! Payment request and response types.

use adyen_core::{Amount, AdyenError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Request to initiate a payment transaction.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentRequest {
    /// The payment amount and currency.
    pub amount: Amount,

    /// The merchant account identifier.
    pub merchant_account: String,

    /// Your reference for the payment.
    pub reference: String,

    /// The URL to return to after the payment.
    pub return_url: String,

    /// Payment method details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<PaymentMethodDetails>,

    /// The sales channel for the transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,

    /// The origin URL of the payment request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,

    /// The shopper's country code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,

    /// The shopper's locale.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shopper_locale: Option<String>,

    /// The shopper reference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shopper_reference: Option<String>,

    /// The shopper's email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shopper_email: Option<String>,

    /// Whether to store the payment method for future use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store_payment_method: Option<bool>,

    /// Additional data for the payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<HashMap<String, String>>,

    /// Browser information for web payments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_info: Option<BrowserInfo>,

    /// Billing address for the payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<Address>,

    /// Delivery address for the payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_address: Option<Address>,
}

/// Payment method details for different payment types.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum PaymentMethodDetails {
    /// Credit/debit card payment.
    #[serde(rename = "scheme")]
    Card {
        /// The card number.
        number: String,
        /// The expiry month (MM).
        expiry_month: String,
        /// The expiry year (YYYY).
        expiry_year: String,
        /// The card security code.
        cvc: String,
        /// The cardholder name.
        #[serde(skip_serializing_if = "Option::is_none")]
        holder_name: Option<String>,
    },

    /// PayPal payment.
    #[serde(rename = "paypal")]
    PayPal {
        /// The PayPal payer ID.
        #[serde(skip_serializing_if = "Option::is_none")]
        payer_id: Option<String>,
    },

    /// iDEAL payment (Netherlands).
    #[serde(rename = "ideal")]
    Ideal {
        /// The selected bank issuer.
        issuer: String,
    },

    /// Google Pay payment.
    #[serde(rename = "googlepay")]
    GooglePay {
        /// The Google Pay token.
        google_pay_token: String,
    },

    /// Apple Pay payment.
    #[serde(rename = "applepay")]
    ApplePay {
        /// The Apple Pay token.
        apple_pay_token: String,
    },

    /// Generic payment method for other types.
    #[serde(untagged)]
    Other(HashMap<String, serde_json::Value>),
}

/// Browser information for web payments.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserInfo {
    /// The accept header value.
    pub accept_header: String,

    /// The color depth of the screen.
    pub color_depth: u32,

    /// Whether Java is enabled.
    pub java_enabled: bool,

    /// Whether JavaScript is enabled.
    pub java_script_enabled: bool,

    /// The browser language.
    pub language: String,

    /// The screen height in pixels.
    pub screen_height: u32,

    /// The screen width in pixels.
    pub screen_width: u32,

    /// The timezone offset in minutes.
    pub time_zone_offset: i32,

    /// The user agent string.
    pub user_agent: String,
}

/// Address information.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    /// The street address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street: Option<String>,

    /// The house number or name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub house_number_or_name: Option<String>,

    /// The city.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,

    /// The postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,

    /// The state or province.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_or_province: Option<String>,

    /// The country code in ISO 3166-1 alpha-2 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
}

/// Response from a payment request.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentResponse {
    /// The result of the payment request.
    pub result_code: PaymentResultCode,

    /// Adyen's 16-character string reference associated with the transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub psp_reference: Option<String>,

    /// Action required to complete the payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<PaymentAction>,

    /// Additional data returned by Adyen.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<HashMap<String, String>>,

    /// The merchant reference for the payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_reference: Option<String>,

    /// Fraud detection results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fraud_result: Option<FraudResult>,

    /// The reason for the payment result.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refusal_reason: Option<String>,
}

/// The result code of a payment request.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum PaymentResultCode {
    /// The payment was successful.
    Authorised,
    /// The payment was declined.
    Refused,
    /// The payment requires additional action.
    RedirectShopper,
    /// The payment requires 3D Secure authentication.
    #[serde(rename = "IdentifyShopper")]
    IdentifyShopper,
    /// The payment requires additional action.
    #[serde(rename = "ChallengeShopper")]
    ChallengeShopper,
    /// The payment is pending.
    Pending,
    /// The payment was cancelled.
    Cancelled,
    /// An error occurred.
    Error,
}

/// Action required to complete a payment.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum PaymentAction {
    /// Redirect the shopper to a URL.
    #[serde(rename = "redirect")]
    Redirect {
        /// The URL to redirect to.
        url: String,
        /// The HTTP method to use.
        method: String,
        /// Additional data to send.
        #[serde(skip_serializing_if = "Option::is_none")]
        data: Option<HashMap<String, String>>,
    },

    /// 3D Secure 2 authentication.
    #[serde(rename = "threeDS2")]
    ThreeDS2 {
        /// The authentication token.
        token: String,
        /// Additional authentication data.
        #[serde(skip_serializing_if = "Option::is_none")]
        authentication_data: Option<HashMap<String, String>>,
    },

    /// Display a QR code to the shopper.
    #[serde(rename = "qrCode")]
    QrCode {
        /// The QR code data.
        qr_code_data: String,
        /// The URL encoded in the QR code.
        #[serde(skip_serializing_if = "Option::is_none")]
        url: Option<String>,
    },

    /// Generic action for other types.
    #[serde(untagged)]
    Other(HashMap<String, serde_json::Value>),
}

/// Fraud detection results.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FraudResult {
    /// The fraud score.
    pub account_score: u32,

    /// The fraud check results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<FraudCheckResult>>,
}

/// Individual fraud check result.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FraudCheckResult {
    /// The name of the fraud check.
    pub name: String,

    /// The result of the fraud check.
    pub check_result: String,

    /// The score assigned by this check.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_score: Option<u32>,
}

/// Request to submit additional payment details.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentDetailsRequest {
    /// The payment details to submit.
    pub details: HashMap<String, String>,

    /// The payment data from the previous response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_data: Option<String>,

    /// The threeDSAuthenticationOnly indicator.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub three_ds_authentication_only: Option<bool>,
}

/// Response from submitting payment details.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentDetailsResponse {
    /// The result of the payment details submission.
    pub result_code: PaymentResultCode,

    /// Adyen's 16-character string reference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub psp_reference: Option<String>,

    /// Additional action required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<PaymentAction>,

    /// Additional data from Adyen.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<HashMap<String, String>>,

    /// The merchant reference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_reference: Option<String>,
}

/// Builder for creating payment requests.
#[derive(Debug, Clone, Default)]
pub struct PaymentRequestBuilder {
    amount: Option<Amount>,
    merchant_account: Option<String>,
    reference: Option<String>,
    return_url: Option<String>,
    payment_method: Option<PaymentMethodDetails>,
    channel: Option<String>,
    origin: Option<String>,
    country_code: Option<String>,
    shopper_locale: Option<String>,
    shopper_reference: Option<String>,
    shopper_email: Option<String>,
    store_payment_method: Option<bool>,
    additional_data: Option<HashMap<String, String>>,
    browser_info: Option<BrowserInfo>,
    billing_address: Option<Address>,
    delivery_address: Option<Address>,
}

impl PaymentRequestBuilder {
    /// Create a new builder.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the payment amount.
    #[must_use]
    pub fn amount(mut self, amount: Amount) -> Self {
        self.amount = Some(amount);
        self
    }

    /// Set the merchant account.
    #[must_use]
    pub fn merchant_account(mut self, merchant_account: impl Into<String>) -> Self {
        self.merchant_account = Some(merchant_account.into());
        self
    }

    /// Set the payment reference.
    #[must_use]
    pub fn reference(mut self, reference: impl Into<String>) -> Self {
        self.reference = Some(reference.into());
        self
    }

    /// Set the return URL.
    #[must_use]
    pub fn return_url(mut self, return_url: impl Into<String>) -> Self {
        self.return_url = Some(return_url.into());
        self
    }

    /// Set the payment method details.
    #[must_use]
    pub fn payment_method(mut self, payment_method: PaymentMethodDetails) -> Self {
        self.payment_method = Some(payment_method);
        self
    }

    /// Set the sales channel.
    #[must_use]
    pub fn channel(mut self, channel: impl Into<String>) -> Self {
        self.channel = Some(channel.into());
        self
    }

    /// Set the origin URL.
    #[must_use]
    pub fn origin(mut self, origin: impl Into<String>) -> Self {
        self.origin = Some(origin.into());
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

    /// Set the shopper reference.
    #[must_use]
    pub fn shopper_reference(mut self, reference: impl Into<String>) -> Self {
        self.shopper_reference = Some(reference.into());
        self
    }

    /// Set the shopper's email.
    #[must_use]
    pub fn shopper_email(mut self, email: impl Into<String>) -> Self {
        self.shopper_email = Some(email.into());
        self
    }

    /// Set whether to store the payment method.
    #[must_use]
    pub fn store_payment_method(mut self, store: bool) -> Self {
        self.store_payment_method = Some(store);
        self
    }

    /// Set browser information.
    #[must_use]
    pub fn browser_info(mut self, browser_info: BrowserInfo) -> Self {
        self.browser_info = Some(browser_info);
        self
    }

    /// Set billing address.
    #[must_use]
    pub fn billing_address(mut self, address: Address) -> Self {
        self.billing_address = Some(address);
        self
    }

    /// Set delivery address.
    #[must_use]
    pub fn delivery_address(mut self, address: Address) -> Self {
        self.delivery_address = Some(address);
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

    /// Build the payment request.
    ///
    /// # Errors
    ///
    /// Returns an error if required fields are not set.
    pub fn build(self) -> Result<PaymentRequest> {
        let amount = self.amount
            .ok_or_else(|| AdyenError::config("amount is required"))?;
        let merchant_account = self.merchant_account
            .ok_or_else(|| AdyenError::config("merchant_account is required"))?;
        let reference = self.reference
            .ok_or_else(|| AdyenError::config("reference is required"))?;
        let return_url = self.return_url
            .ok_or_else(|| AdyenError::config("return_url is required"))?;

        Ok(PaymentRequest {
            amount,
            merchant_account,
            reference,
            return_url,
            payment_method: self.payment_method,
            channel: self.channel,
            origin: self.origin,
            country_code: self.country_code,
            shopper_locale: self.shopper_locale,
            shopper_reference: self.shopper_reference,
            shopper_email: self.shopper_email,
            store_payment_method: self.store_payment_method,
            additional_data: self.additional_data,
            browser_info: self.browser_info,
            billing_address: self.billing_address,
            delivery_address: self.delivery_address,
        })
    }
}

impl PaymentRequest {
    /// Create a new builder for payment requests.
    #[must_use]
    pub fn builder() -> PaymentRequestBuilder {
        PaymentRequestBuilder::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use adyen_core::{Amount, Currency};

    #[test]
    fn test_payment_request_builder() {
        let amount = Amount::from_major_units(100, Currency::EUR);

        let request = PaymentRequest::builder()
            .amount(amount.clone())
            .merchant_account("TestMerchant")
            .reference("Order-12345")
            .return_url("https://example.com/return")
            .channel("Web")
            .country_code("NL")
            .build()
            .unwrap();

        assert_eq!(request.amount, amount);
        assert_eq!(request.merchant_account, "TestMerchant");
        assert_eq!(request.reference, "Order-12345");
        assert_eq!(request.return_url, "https://example.com/return");
        assert_eq!(request.channel, Some("Web".to_string()));
        assert_eq!(request.country_code, Some("NL".to_string()));
    }

    #[test]
    fn test_payment_request_missing_required_fields() {
        assert!(PaymentRequest::builder().build().is_err());

        let amount = Amount::from_major_units(100, Currency::EUR);
        assert!(PaymentRequest::builder()
            .amount(amount)
            .build()
            .is_err());
    }

    #[test]
    fn test_payment_method_details_serialization() {
        let card = PaymentMethodDetails::Card {
            number: "4111111111111111".to_string(),
            expiry_month: "12".to_string(),
            expiry_year: "2025".to_string(),
            cvc: "123".to_string(),
            holder_name: Some("John Doe".to_string()),
        };

        let json = serde_json::to_string(&card).unwrap();
        assert!(json.contains("\"type\":\"scheme\""));
        assert!(json.contains("\"number\":\"4111111111111111\""));
    }

    #[test]
    fn test_payment_result_code_serialization() {
        assert_eq!(
            serde_json::to_string(&PaymentResultCode::Authorised).unwrap(),
            "\"Authorised\""
        );
        assert_eq!(
            serde_json::to_string(&PaymentResultCode::Refused).unwrap(),
            "\"Refused\""
        );
    }
}