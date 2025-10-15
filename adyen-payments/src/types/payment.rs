//! Classic payment request and response types.

use adyen_core::{AdyenError, Amount, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Request to create a payment authorization.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentRequest {
    /// The payment amount and currency.
    pub amount: Amount,

    /// The merchant account identifier.
    pub merchant_account: String,

    /// Your reference for the payment.
    pub reference: String,

    /// Payment method details.
    #[serde(flatten)]
    pub payment_method: PaymentMethod,

    /// The sales channel for the transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,

    /// The shopper's country code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,

    /// The shopper's locale.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shopper_locale: Option<String>,

    /// The shopper reference for recurring payments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shopper_reference: Option<String>,

    /// The shopper's email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shopper_email: Option<String>,

    /// The shopper's IP address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shopper_ip: Option<String>,

    /// Recurring payment configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring: Option<RecurringType>,

    /// The URL to return to after 3D Secure authentication.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,

    /// Browser information for 3D Secure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_info: Option<BrowserInfo>,

    /// Application information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_info: Option<ApplicationInfo>,

    /// Additional data for the payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<HashMap<String, String>>,

    /// Session validity in minutes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_validity: Option<String>,

    /// Billing address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<Address>,

    /// Delivery address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_address: Option<Address>,

    /// Installment configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installments: Option<Installments>,
}

/// Payment method details for different payment types.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PaymentMethod {
    /// Credit/debit card payment.
    Card {
        /// Card details.
        card: Card,
    },

    /// Stored payment method.
    Stored {
        /// The stored payment method ID.
        selected_recurring_detail_reference: String,
    },

    /// Alternative payment method.
    Alternative {
        /// Additional payment method data.
        #[serde(flatten)]
        data: HashMap<String, serde_json::Value>,
    },
}

/// Credit/debit card details.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Card {
    /// The card number.
    pub number: String,

    /// The expiry month (MM).
    pub expiry_month: String,

    /// The expiry year (YYYY).
    pub expiry_year: String,

    /// The card security code.
    pub cvc: String,

    /// The cardholder name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub holder_name: Option<String>,
}

/// Recurring payment configuration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecurringType {
    /// The type of recurring contract.
    pub contract: RecurringContract,

    /// Token service for network tokenization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_service: Option<String>,
}

/// Recurring contract types.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RecurringContract {
    /// One-click payments.
    Oneclick,
    /// Recurring payments.
    Recurring,
    /// Both one-click and recurring.
    OneclickRecurring,
}

/// Browser information for 3D Secure authentication.
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

/// Application information.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationInfo {
    /// Information about the merchant application.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_application: Option<MerchantApplication>,

    /// Information about the external platform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_platform: Option<ExternalPlatform>,
}

/// Merchant application information.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MerchantApplication {
    /// The application name.
    pub name: String,

    /// The application version.
    pub version: String,
}

/// External platform information.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalPlatform {
    /// The platform name.
    pub name: String,

    /// The platform version.
    pub version: String,

    /// The platform integrator.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integrator: Option<String>,
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

/// Installment configuration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Installments {
    /// The number of installments.
    pub value: u32,
}

/// Response from a payment authorization request.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentResult {
    /// The result of the payment request.
    pub result_code: PaymentResultCode,

    /// Adyen's 16-character string reference associated with the transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub psp_reference: Option<String>,

    /// The merchant reference for the payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_reference: Option<String>,

    /// The authorization code returned by the card issuer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_code: Option<String>,

    /// The reason for the payment result.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refusal_reason: Option<String>,

    /// Fraud detection results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fraud_result: Option<FraudResult>,

    /// Additional data returned by Adyen.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<HashMap<String, String>>,

    /// 3D Secure redirect URL (for `result_code` = `RedirectShopper`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer_url: Option<String>,

    /// 3D Secure form data (for `result_code` = `RedirectShopper`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub md: Option<String>,

    /// 3D Secure `PaReq` data (for `result_code` = `RedirectShopper`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pa_request: Option<String>,
}

/// The result code of a payment request.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum PaymentResultCode {
    /// The payment was successful.
    Authorised,
    /// The payment was declined.
    Refused,
    /// The payment was cancelled.
    Cancelled,
    /// An error occurred.
    Error,
    /// The payment requires 3D Secure authentication.
    RedirectShopper,
    /// The payment is pending additional verification.
    Received,
    /// The payment is pending.
    Pending,
}

/// Fraud detection results.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FraudResult {
    /// The fraud score.
    pub account_score: u32,

    /// The fraud check results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<crate::types::fraud::FraudCheckResult>>,
}

/// Builder for creating payment requests.
#[derive(Debug, Clone, Default)]
pub struct PaymentRequestBuilder {
    amount: Option<Amount>,
    merchant_account: Option<String>,
    reference: Option<String>,
    payment_method: Option<PaymentMethod>,
    channel: Option<String>,
    country_code: Option<String>,
    shopper_locale: Option<String>,
    shopper_reference: Option<String>,
    shopper_email: Option<String>,
    shopper_ip: Option<String>,
    recurring: Option<RecurringType>,
    return_url: Option<String>,
    browser_info: Option<BrowserInfo>,
    application_info: Option<ApplicationInfo>,
    additional_data: Option<HashMap<String, String>>,
    session_validity: Option<String>,
    billing_address: Option<Address>,
    delivery_address: Option<Address>,
    installments: Option<Installments>,
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

    /// Set card payment method.
    #[must_use]
    pub fn card(mut self, card: Card) -> Self {
        self.payment_method = Some(PaymentMethod::Card { card });
        self
    }

    /// Set stored payment method.
    #[must_use]
    pub fn stored_payment_method(mut self, reference: impl Into<String>) -> Self {
        self.payment_method = Some(PaymentMethod::Stored {
            selected_recurring_detail_reference: reference.into(),
        });
        self
    }

    /// Set the sales channel.
    #[must_use]
    pub fn channel(mut self, channel: impl Into<String>) -> Self {
        self.channel = Some(channel.into());
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

    /// Set the shopper's IP address.
    #[must_use]
    pub fn shopper_ip(mut self, ip: impl Into<String>) -> Self {
        self.shopper_ip = Some(ip.into());
        self
    }

    /// Set recurring payment configuration.
    #[must_use]
    pub fn recurring(mut self, recurring: RecurringType) -> Self {
        self.recurring = Some(recurring);
        self
    }

    /// Set the return URL for 3D Secure.
    #[must_use]
    pub fn return_url(mut self, return_url: impl Into<String>) -> Self {
        self.return_url = Some(return_url.into());
        self
    }

    /// Set browser information.
    #[must_use]
    pub fn browser_info(mut self, browser_info: BrowserInfo) -> Self {
        self.browser_info = Some(browser_info);
        self
    }

    /// Set application information.
    #[must_use]
    pub fn application_info(mut self, application_info: ApplicationInfo) -> Self {
        self.application_info = Some(application_info);
        self
    }

    /// Set session validity.
    #[must_use]
    pub fn session_validity(mut self, validity: impl Into<String>) -> Self {
        self.session_validity = Some(validity.into());
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

    /// Set installments.
    #[must_use]
    pub fn installments(mut self, installments: Installments) -> Self {
        self.installments = Some(installments);
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
        let amount = self
            .amount
            .ok_or_else(|| AdyenError::config("amount is required"))?;
        let merchant_account = self
            .merchant_account
            .ok_or_else(|| AdyenError::config("merchant_account is required"))?;
        let reference = self
            .reference
            .ok_or_else(|| AdyenError::config("reference is required"))?;
        let payment_method = self
            .payment_method
            .ok_or_else(|| AdyenError::config("payment_method is required"))?;

        Ok(PaymentRequest {
            amount,
            merchant_account,
            reference,
            payment_method,
            channel: self.channel,
            country_code: self.country_code,
            shopper_locale: self.shopper_locale,
            shopper_reference: self.shopper_reference,
            shopper_email: self.shopper_email,
            shopper_ip: self.shopper_ip,
            recurring: self.recurring,
            return_url: self.return_url,
            browser_info: self.browser_info,
            application_info: self.application_info,
            additional_data: self.additional_data,
            session_validity: self.session_validity,
            billing_address: self.billing_address,
            delivery_address: self.delivery_address,
            installments: self.installments,
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

impl Card {
    /// Create a new card with the basic required information.
    #[must_use]
    pub fn new(
        number: impl Into<String>,
        expiry_month: impl Into<String>,
        expiry_year: impl Into<String>,
        cvc: impl Into<String>,
    ) -> Self {
        Self {
            number: number.into(),
            expiry_month: expiry_month.into(),
            expiry_year: expiry_year.into(),
            cvc: cvc.into(),
            holder_name: None,
        }
    }

    /// Set the cardholder name.
    #[must_use]
    pub fn with_holder_name(mut self, holder_name: impl Into<String>) -> Self {
        self.holder_name = Some(holder_name.into());
        self
    }
}

impl RecurringType {
    /// Create a new recurring configuration for one-click payments.
    #[must_use]
    pub fn oneclick() -> Self {
        Self {
            contract: RecurringContract::Oneclick,
            token_service: None,
        }
    }

    /// Create a new recurring configuration for recurring payments.
    #[must_use]
    pub fn recurring() -> Self {
        Self {
            contract: RecurringContract::Recurring,
            token_service: None,
        }
    }

    /// Create a new recurring configuration for both one-click and recurring.
    #[must_use]
    pub fn oneclick_recurring() -> Self {
        Self {
            contract: RecurringContract::OneclickRecurring,
            token_service: None,
        }
    }

    /// Set the token service.
    #[must_use]
    pub fn with_token_service(mut self, token_service: impl Into<String>) -> Self {
        self.token_service = Some(token_service.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use adyen_core::{Amount, Currency};

    #[test]
    fn test_payment_request_builder() {
        let amount = Amount::from_major_units(100, Currency::EUR);
        let card = Card::new("4111111111111111", "12", "2025", "123");

        let request = PaymentRequest::builder()
            .amount(amount.clone())
            .merchant_account("TestMerchant")
            .reference("Order-12345")
            .card(card.clone())
            .country_code("NL")
            .build()
            .unwrap();

        assert_eq!(request.amount, amount);
        assert_eq!(request.merchant_account, "TestMerchant");
        assert_eq!(request.reference, "Order-12345");
        assert_eq!(request.country_code, Some("NL".to_string()));

        if let PaymentMethod::Card { card: request_card } = request.payment_method {
            assert_eq!(request_card, card);
        } else {
            panic!("Expected card payment method");
        }
    }

    #[test]
    fn test_card_creation() {
        let card = Card::new("4111111111111111", "12", "2025", "123").with_holder_name("John Doe");

        assert_eq!(card.number, "4111111111111111");
        assert_eq!(card.expiry_month, "12");
        assert_eq!(card.expiry_year, "2025");
        assert_eq!(card.cvc, "123");
        assert_eq!(card.holder_name, Some("John Doe".to_string()));
    }

    #[test]
    fn test_recurring_types() {
        let oneclick = RecurringType::oneclick();
        assert_eq!(oneclick.contract, RecurringContract::Oneclick);

        let recurring = RecurringType::recurring().with_token_service("VISA");
        assert_eq!(recurring.contract, RecurringContract::Recurring);
        assert_eq!(recurring.token_service, Some("VISA".to_string()));
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
        assert_eq!(
            serde_json::to_string(&PaymentResultCode::RedirectShopper).unwrap(),
            "\"RedirectShopper\""
        );
    }

    #[test]
    fn test_payment_request_missing_required_fields() {
        assert!(PaymentRequest::builder().build().is_err());

        let amount = Amount::from_major_units(100, Currency::EUR);
        assert!(PaymentRequest::builder().amount(amount).build().is_err());
    }
}
