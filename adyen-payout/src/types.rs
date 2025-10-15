//! Types for the Adyen Payout API v68.
//!
//! This module contains all request and response types for payout operations,
//! including instant payouts, batch processing, and review workflows.

use adyen_core::{AdyenError, Amount};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Builder for creating submit payout requests.
#[derive(Debug, Clone, Default)]
pub struct SubmitRequestBuilder {
    amount: Option<Amount>,
    merchant_account: Option<Box<str>>,
    reference: Option<Box<str>>,
    shopper_email: Option<Box<str>>,
    shopper_reference: Option<Box<str>>,
    payout_method_details: Option<PayoutMethodDetails>,
    billing_address: Option<Address>,
    date_of_birth: Option<Box<str>>,
    entity_type: Option<EntityType>,
    nationality: Option<Box<str>>,
    shopper_name: Option<Name>,
}

impl SubmitRequestBuilder {
    /// Create a new submit request builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the payout amount.
    pub fn amount(mut self, amount: Amount) -> Self {
        self.amount = Some(amount);
        self
    }

    /// Set the merchant account identifier.
    pub fn merchant_account(mut self, merchant_account: impl Into<Box<str>>) -> Self {
        self.merchant_account = Some(merchant_account.into());
        self
    }

    /// Set the payout reference.
    pub fn reference(mut self, reference: impl Into<Box<str>>) -> Self {
        self.reference = Some(reference.into());
        self
    }

    /// Set the shopper email address.
    pub fn shopper_email(mut self, email: impl Into<Box<str>>) -> Self {
        self.shopper_email = Some(email.into());
        self
    }

    /// Set the shopper reference.
    pub fn shopper_reference(mut self, reference: impl Into<Box<str>>) -> Self {
        self.shopper_reference = Some(reference.into());
        self
    }

    /// Set the payout method details.
    pub fn payout_method_details(mut self, details: PayoutMethodDetails) -> Self {
        self.payout_method_details = Some(details);
        self
    }

    /// Set the billing address.
    pub fn billing_address(mut self, address: Address) -> Self {
        self.billing_address = Some(address);
        self
    }

    /// Set the date of birth (YYYY-MM-DD format).
    pub fn date_of_birth(mut self, date: impl Into<Box<str>>) -> Self {
        self.date_of_birth = Some(date.into());
        self
    }

    /// Set the entity type.
    pub fn entity_type(mut self, entity_type: EntityType) -> Self {
        self.entity_type = Some(entity_type);
        self
    }

    /// Set the nationality (ISO 3166-1 alpha-2 country code).
    pub fn nationality(mut self, nationality: impl Into<Box<str>>) -> Self {
        self.nationality = Some(nationality.into());
        self
    }

    /// Set the shopper name.
    pub fn shopper_name(mut self, name: Name) -> Self {
        self.shopper_name = Some(name);
        self
    }

    /// Build the submit request.
    pub fn build(self) -> Result<SubmitRequest, AdyenError> {
        Ok(SubmitRequest {
            amount: self
                .amount
                .ok_or_else(|| AdyenError::config("Missing required field: amount"))?,
            merchant_account: self
                .merchant_account
                .ok_or_else(|| AdyenError::config("Missing required field: merchant_account"))?,
            reference: self
                .reference
                .ok_or_else(|| AdyenError::config("Missing required field: reference"))?,
            shopper_email: self
                .shopper_email
                .ok_or_else(|| AdyenError::config("Missing required field: shopper_email"))?,
            shopper_reference: self
                .shopper_reference
                .ok_or_else(|| AdyenError::config("Missing required field: shopper_reference"))?,
            payout_method_details: self.payout_method_details.ok_or_else(|| {
                AdyenError::config("Missing required field: payout_method_details")
            })?,
            billing_address: self.billing_address,
            date_of_birth: self.date_of_birth,
            entity_type: self.entity_type,
            nationality: self.nationality,
            shopper_name: self.shopper_name,
        })
    }
}

/// Request to submit a payout.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubmitRequest {
    /// The payout amount.
    pub amount: Amount,
    /// The merchant account identifier.
    pub merchant_account: Box<str>,
    /// Your reference for the payout.
    pub reference: Box<str>,
    /// The shopper's email address.
    pub shopper_email: Box<str>,
    /// Your reference for the shopper.
    pub shopper_reference: Box<str>,
    /// Details about the payout method.
    pub payout_method_details: PayoutMethodDetails,
    /// The billing address (required for some payout methods).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<Address>,
    /// The shopper's date of birth in YYYY-MM-DD format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<Box<str>>,
    /// The type of entity for the payout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<EntityType>,
    /// The shopper's nationality as an ISO 3166-1 alpha-2 country code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nationality: Option<Box<str>>,
    /// The shopper's name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shopper_name: Option<Name>,
}

impl SubmitRequest {
    /// Create a new submit request builder.
    pub fn builder() -> SubmitRequestBuilder {
        SubmitRequestBuilder::new()
    }
}

/// Response from submitting a payout.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubmitResponse {
    /// A unique reference for the payout.
    pub psp_reference: Box<str>,
    /// The result code indicating the outcome of the payout submission.
    pub result_code: PayoutResultCode,
    /// Additional information about the payout (if available).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<HashMap<Box<str>, Box<str>>>,
    /// The merchant reference as sent in the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_reference: Option<Box<str>>,
    /// Refusal reason in case the payout was refused.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refusal_reason: Option<Box<str>>,
}

/// Builder for creating confirm payout requests.
#[derive(Debug, Clone, Default)]
pub struct ConfirmRequestBuilder {
    merchant_account: Option<Box<str>>,
    original_reference: Option<Box<str>>,
}

impl ConfirmRequestBuilder {
    /// Create a new confirm request builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the merchant account identifier.
    pub fn merchant_account(mut self, merchant_account: impl Into<Box<str>>) -> Self {
        self.merchant_account = Some(merchant_account.into());
        self
    }

    /// Set the original payout reference.
    pub fn original_reference(mut self, reference: impl Into<Box<str>>) -> Self {
        self.original_reference = Some(reference.into());
        self
    }

    /// Build the confirm request.
    pub fn build(self) -> Result<ConfirmRequest, AdyenError> {
        Ok(ConfirmRequest {
            merchant_account: self
                .merchant_account
                .ok_or_else(|| AdyenError::config("Missing required field: merchant_account"))?,
            original_reference: self
                .original_reference
                .ok_or_else(|| AdyenError::config("Missing required field: original_reference"))?,
        })
    }
}

/// Request to confirm a payout.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfirmRequest {
    /// The merchant account identifier.
    pub merchant_account: Box<str>,
    /// The PSP reference of the payout to confirm.
    pub original_reference: Box<str>,
}

impl ConfirmRequest {
    /// Create a new confirm request builder.
    pub fn builder() -> ConfirmRequestBuilder {
        ConfirmRequestBuilder::new()
    }
}

/// Response from confirming a payout.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfirmResponse {
    /// A unique reference for the confirmation.
    pub psp_reference: Box<str>,
    /// The result code indicating the outcome of the confirmation.
    pub response: Box<str>,
}

/// Builder for creating review payout requests.
#[derive(Debug, Clone, Default)]
pub struct ReviewPayoutRequestBuilder {
    merchant_account: Option<Box<str>>,
    psp_reference: Option<Box<str>>,
}

impl ReviewPayoutRequestBuilder {
    /// Create a new review payout request builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the merchant account identifier.
    pub fn merchant_account(mut self, merchant_account: impl Into<Box<str>>) -> Self {
        self.merchant_account = Some(merchant_account.into());
        self
    }

    /// Set the PSP reference of the payout to review.
    pub fn psp_reference(mut self, reference: impl Into<Box<str>>) -> Self {
        self.psp_reference = Some(reference.into());
        self
    }

    /// Build the review request.
    pub fn build(self) -> Result<ReviewPayoutRequest, AdyenError> {
        Ok(ReviewPayoutRequest {
            merchant_account: self
                .merchant_account
                .ok_or_else(|| AdyenError::config("Missing required field: merchant_account"))?,
            psp_reference: self
                .psp_reference
                .ok_or_else(|| AdyenError::config("Missing required field: psp_reference"))?,
        })
    }
}

/// Request to review a payout that requires manual approval.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReviewPayoutRequest {
    /// The merchant account identifier.
    pub merchant_account: Box<str>,
    /// The PSP reference of the payout to review.
    pub psp_reference: Box<str>,
}

impl ReviewPayoutRequest {
    /// Create a new review payout request builder.
    pub fn builder() -> ReviewPayoutRequestBuilder {
        ReviewPayoutRequestBuilder::new()
    }
}

/// Builder for creating decline payout requests.
#[derive(Debug, Clone, Default)]
pub struct DeclinePayoutRequestBuilder {
    merchant_account: Option<Box<str>>,
    psp_reference: Option<Box<str>>,
}

impl DeclinePayoutRequestBuilder {
    /// Create a new decline payout request builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the merchant account identifier.
    pub fn merchant_account(mut self, merchant_account: impl Into<Box<str>>) -> Self {
        self.merchant_account = Some(merchant_account.into());
        self
    }

    /// Set the PSP reference of the payout to decline.
    pub fn psp_reference(mut self, reference: impl Into<Box<str>>) -> Self {
        self.psp_reference = Some(reference.into());
        self
    }

    /// Build the decline request.
    pub fn build(self) -> Result<DeclinePayoutRequest, AdyenError> {
        Ok(DeclinePayoutRequest {
            merchant_account: self
                .merchant_account
                .ok_or_else(|| AdyenError::config("Missing required field: merchant_account"))?,
            psp_reference: self
                .psp_reference
                .ok_or_else(|| AdyenError::config("Missing required field: psp_reference"))?,
        })
    }
}

/// Request to decline a payout that requires manual review.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeclinePayoutRequest {
    /// The merchant account identifier.
    pub merchant_account: Box<str>,
    /// The PSP reference of the payout to decline.
    pub psp_reference: Box<str>,
}

impl DeclinePayoutRequest {
    /// Create a new decline payout request builder.
    pub fn builder() -> DeclinePayoutRequestBuilder {
        DeclinePayoutRequestBuilder::new()
    }
}

/// Generic payout response for review and decline operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PayoutResponse {
    /// A unique reference for the operation.
    pub psp_reference: Box<str>,
    /// The result code indicating the outcome.
    pub response: Box<str>,
}

/// Details about the payout method.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum PayoutMethodDetails {
    /// Bank account payout.
    #[serde(rename = "bankAccount")]
    BankAccount(BankAccount),
    /// Card payout (where supported).
    #[serde(rename = "card")]
    Card(Card),
}

/// Bank account details for payouts.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BankAccount {
    /// The bank account number.
    pub account_number: Box<str>,
    /// The bank identifier code (BIC/SWIFT).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bic: Option<Box<str>>,
    /// The country code of the bank.
    pub country_code: Box<str>,
    /// The name of the account holder.
    pub owner_name: Box<str>,
    /// The International Bank Account Number (IBAN).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iban: Option<Box<str>>,
    /// The bank account type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account_type: Option<BankAccountType>,
}

/// Card details for payouts.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Card {
    /// The card number.
    pub number: Box<str>,
    /// The card expiry month.
    pub expiry_month: Box<str>,
    /// The card expiry year.
    pub expiry_year: Box<str>,
    /// The name of the cardholder.
    pub holder_name: Box<str>,
}

/// Address information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    /// The city name.
    pub city: Box<str>,
    /// The country code (ISO 3166-1 alpha-2).
    pub country: Box<str>,
    /// The house number or name.
    pub house_number_or_name: Box<str>,
    /// The postal code.
    pub postal_code: Box<str>,
    /// The state or province code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_or_province: Option<Box<str>>,
    /// The street name.
    pub street: Box<str>,
}

/// Name information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Name {
    /// The first name.
    pub first_name: Box<str>,
    /// The last name.
    pub last_name: Box<str>,
}

/// The result code for payout operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum PayoutResultCode {
    /// The payout was received and will be processed.
    Received,
    /// The payout was refused.
    Refused,
}

/// The type of entity making the payout.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum EntityType {
    /// Natural person.
    NaturalPerson,
    /// Company.
    Company,
}

/// The type of bank account.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BankAccountType {
    /// Checking account.
    Checking,
    /// Savings account.
    Savings,
}

#[cfg(test)]
mod tests {
    use super::*;
    use adyen_core::{Amount, Currency};

    #[test]
    fn test_submit_request_builder() {
        let amount = Amount::from_minor_units(1000, Currency::EUR);
        let bank_account = BankAccount {
            account_number: "1234567890".into(),
            bic: Some("ABNANL2A".into()),
            country_code: "NL".into(),
            owner_name: "John Doe".into(),
            iban: Some("NL91ABNA0417164300".into()),
            bank_account_type: Some(BankAccountType::Checking),
        };

        let request = SubmitRequest::builder()
            .amount(amount)
            .merchant_account("TestMerchant")
            .reference("payout-123")
            .shopper_email("john@example.com")
            .shopper_reference("shopper-123")
            .payout_method_details(PayoutMethodDetails::BankAccount(bank_account))
            .build()
            .unwrap();

        assert_eq!(request.amount.minor_units(), 1000);
        assert_eq!(request.amount.currency(), Currency::EUR);
        assert_eq!(&*request.merchant_account, "TestMerchant");
        assert_eq!(&*request.reference, "payout-123");
    }

    #[test]
    fn test_confirm_request_builder() {
        let request = ConfirmRequest::builder()
            .merchant_account("TestMerchant")
            .original_reference("psp-123")
            .build()
            .unwrap();

        assert_eq!(&*request.merchant_account, "TestMerchant");
        assert_eq!(&*request.original_reference, "psp-123");
    }

    #[test]
    fn test_review_payout_request_builder() {
        let request = ReviewPayoutRequest::builder()
            .merchant_account("TestMerchant")
            .psp_reference("psp-123")
            .build()
            .unwrap();

        assert_eq!(&*request.merchant_account, "TestMerchant");
        assert_eq!(&*request.psp_reference, "psp-123");
    }

    #[test]
    fn test_decline_payout_request_builder() {
        let request = DeclinePayoutRequest::builder()
            .merchant_account("TestMerchant")
            .psp_reference("psp-123")
            .build()
            .unwrap();

        assert_eq!(&*request.merchant_account, "TestMerchant");
        assert_eq!(&*request.psp_reference, "psp-123");
    }

    #[test]
    fn test_submit_request_builder_missing_fields() {
        let result = SubmitRequest::builder().build();
        assert!(result.is_err());
    }

    #[test]
    fn test_confirm_request_builder_missing_fields() {
        let result = ConfirmRequest::builder().build();
        assert!(result.is_err());
    }

    #[test]
    fn test_serialization() {
        let amount = Amount::from_minor_units(1000, Currency::EUR);
        let bank_account = BankAccount {
            account_number: "1234567890".into(),
            bic: Some("ABNANL2A".into()),
            country_code: "NL".into(),
            owner_name: "John Doe".into(),
            iban: Some("NL91ABNA0417164300".into()),
            bank_account_type: Some(BankAccountType::Checking),
        };

        let request = SubmitRequest::builder()
            .amount(amount)
            .merchant_account("TestMerchant")
            .reference("payout-123")
            .shopper_email("john@example.com")
            .shopper_reference("shopper-123")
            .payout_method_details(PayoutMethodDetails::BankAccount(bank_account))
            .build()
            .unwrap();

        let json = serde_json::to_string(&request).unwrap();
        let _deserialized: SubmitRequest = serde_json::from_str(&json).unwrap();
    }

    #[test]
    fn test_payout_result_code_serialization() {
        let received = PayoutResultCode::Received;
        let json = serde_json::to_string(&received).unwrap();
        assert_eq!(json, "\"Received\"");

        let refused = PayoutResultCode::Refused;
        let json = serde_json::to_string(&refused).unwrap();
        assert_eq!(json, "\"Refused\"");
    }

    #[test]
    fn test_entity_type_serialization() {
        let person = EntityType::NaturalPerson;
        let json = serde_json::to_string(&person).unwrap();
        assert_eq!(json, "\"NaturalPerson\"");

        let company = EntityType::Company;
        let json = serde_json::to_string(&company).unwrap();
        assert_eq!(json, "\"Company\"");
    }
}
