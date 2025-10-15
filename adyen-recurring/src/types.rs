//! Types for the Adyen Recurring API v68.
//!
//! This module contains all request and response types for recurring payment operations.

use adyen_core::{Amount, AdyenError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Request to retrieve stored payment details for a shopper.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecurringDetailsRequest {
    /// The merchant account identifier.
    pub merchant_account: Box<str>,
    /// The shopper's reference, a unique identifier for this shopper.
    pub shopper_reference: Box<str>,
    /// Optional recurring details configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring: Option<Recurring>,
}

/// Result containing the stored payment details for a shopper.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecurringDetailsResult {
    /// Details of the recurring payments.
    #[serde(default)]
    pub details: Vec<RecurringDetail>,
    /// The date when the recurring details were last updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_known_shopper_email: Option<Box<str>>,
    /// The shopper reference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shopper_reference: Option<Box<str>>,
}

/// Request to disable stored payment details.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DisableRequest {
    /// The merchant account identifier.
    pub merchant_account: Box<str>,
    /// The shopper's reference.
    pub shopper_reference: Box<str>,
    /// The recurring detail reference to disable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring_detail_reference: Option<Box<str>>,
}

/// Result of disabling stored payment details.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DisableResult {
    /// Indicates whether the recurring detail was successfully disabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response: Option<Box<str>>,
}

/// Request to notify the shopper about an upcoming recurring payment.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotifyShopperRequest {
    /// The payment amount.
    pub amount: Amount,
    /// The merchant account identifier.
    pub merchant_account: Box<str>,
    /// Reference for this request.
    pub reference: Box<str>,
    /// The shopper's reference.
    pub shopper_reference: Box<str>,
    /// The date when the recurring payment will be processed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_date: Option<Box<str>>,
    /// The stored payment method to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stored_payment_method_id: Option<Box<str>>,
}

/// Result of the shopper notification request.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotifyShopperResult {
    /// The PSP reference for this request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub psp_reference: Option<Box<str>>,
    /// Response message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response: Option<Box<str>>,
    /// The result code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_code: Option<Box<str>>,
}

/// Request to schedule the Account Updater for stored payment methods.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScheduleAccountUpdaterRequest {
    /// The merchant account identifier.
    pub merchant_account: Box<str>,
    /// Reference for this request.
    pub reference: Box<str>,
    /// The shopper's reference.
    pub shopper_reference: Box<str>,
    /// Specific card details to update.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<Card>,
}

/// Result of scheduling the Account Updater.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScheduleAccountUpdaterResult {
    /// The PSP reference for this request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub psp_reference: Option<Box<str>>,
    /// Response message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response: Option<Box<str>>,
    /// The result code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_code: Option<Box<str>>,
}

/// Stored payment method details.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecurringDetail {
    /// Reference for this recurring detail.
    pub recurring_detail_reference: Box<str>,
    /// Payment method variant (e.g., "visa", "mc").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant: Option<Box<str>>,
    /// Supported contract types for this payment method.
    #[serde(default)]
    pub contract_types: Vec<Box<str>>,
    /// Card details if this is a card payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<Card>,
    /// Bank account details if this is a bank payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank: Option<BankAccount>,
    /// Name associated with this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Box<str>>,
    /// The date this payment method was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<Box<str>>,
    /// Additional details about the payment method.
    #[serde(flatten)]
    pub additional_data: HashMap<String, serde_json::Value>,
}

/// Configuration for recurring payments.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Recurring {
    /// The type of recurring contract.
    pub contract: RecurringContract,
    /// Name for this recurring detail.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring_detail_name: Option<Box<str>>,
    /// Date when the recurring contract expires.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring_expiry: Option<Box<str>>,
    /// Frequency of recurring payments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring_frequency: Option<Box<str>>,
    /// Token service configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_service: Option<Box<str>>,
}

/// Types of recurring contracts.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RecurringContract {
    /// One-click payments - shopper needs to confirm each payment.
    Oneclick,
    /// Fully recurring payments - no shopper interaction required.
    Recurring,
    /// Both one-click and recurring contract types.
    OneclickRecurring,
}

/// Card details for stored payment methods.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Card {
    /// Masked card number showing only the last 4 digits.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<Box<str>>,
    /// Card expiry month.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_month: Option<Box<str>>,
    /// Card expiry year.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_year: Option<Box<str>>,
    /// Cardholder name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub holder_name: Option<Box<str>>,
    /// Summary of the card (e.g., "1111" for cards ending in 1111).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<Box<str>>,
}

/// Bank account details for stored payment methods.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BankAccount {
    /// Bank identifier code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bic: Option<Box<str>>,
    /// Country code of the bank.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<Box<str>>,
    /// IBAN of the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iban: Option<Box<str>>,
    /// Name of the account owner.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_name: Option<Box<str>>,
    /// Bank name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<Box<str>>,
}

// Builder implementations for easier API usage
impl RecurringDetailsRequest {
    /// Create a new builder for RecurringDetailsRequest.
    pub fn builder() -> RecurringDetailsRequestBuilder {
        RecurringDetailsRequestBuilder::default()
    }
}

/// Builder for RecurringDetailsRequest.
#[derive(Default)]
pub struct RecurringDetailsRequestBuilder {
    merchant_account: Option<Box<str>>,
    shopper_reference: Option<Box<str>>,
    recurring: Option<Recurring>,
}

impl RecurringDetailsRequestBuilder {
    /// Set the merchant account.
    pub fn merchant_account(mut self, merchant_account: impl Into<Box<str>>) -> Self {
        self.merchant_account = Some(merchant_account.into());
        self
    }

    /// Set the shopper reference.
    pub fn shopper_reference(mut self, shopper_reference: impl Into<Box<str>>) -> Self {
        self.shopper_reference = Some(shopper_reference.into());
        self
    }

    /// Set the recurring configuration.
    pub fn recurring(mut self, recurring: Recurring) -> Self {
        self.recurring = Some(recurring);
        self
    }

    /// Build the RecurringDetailsRequest.
    pub fn build(self) -> Result<RecurringDetailsRequest> {
        let merchant_account = self.merchant_account
            .ok_or_else(|| AdyenError::generic("merchant_account is required"))?;
        let shopper_reference = self.shopper_reference
            .ok_or_else(|| AdyenError::generic("shopper_reference is required"))?;

        Ok(RecurringDetailsRequest {
            merchant_account,
            shopper_reference,
            recurring: self.recurring,
        })
    }
}

impl DisableRequest {
    /// Create a new builder for DisableRequest.
    pub fn builder() -> DisableRequestBuilder {
        DisableRequestBuilder::default()
    }
}

/// Builder for DisableRequest.
#[derive(Default)]
pub struct DisableRequestBuilder {
    merchant_account: Option<Box<str>>,
    shopper_reference: Option<Box<str>>,
    recurring_detail_reference: Option<Box<str>>,
}

impl DisableRequestBuilder {
    /// Set the merchant account.
    pub fn merchant_account(mut self, merchant_account: impl Into<Box<str>>) -> Self {
        self.merchant_account = Some(merchant_account.into());
        self
    }

    /// Set the shopper reference.
    pub fn shopper_reference(mut self, shopper_reference: impl Into<Box<str>>) -> Self {
        self.shopper_reference = Some(shopper_reference.into());
        self
    }

    /// Set the recurring detail reference to disable.
    pub fn recurring_detail_reference(mut self, reference: impl Into<Box<str>>) -> Self {
        self.recurring_detail_reference = Some(reference.into());
        self
    }

    /// Build the DisableRequest.
    pub fn build(self) -> Result<DisableRequest> {
        let merchant_account = self.merchant_account
            .ok_or_else(|| AdyenError::generic("merchant_account is required"))?;
        let shopper_reference = self.shopper_reference
            .ok_or_else(|| AdyenError::generic("shopper_reference is required"))?;

        Ok(DisableRequest {
            merchant_account,
            shopper_reference,
            recurring_detail_reference: self.recurring_detail_reference,
        })
    }
}

// ============================================================================
// Permit Management Types
// ============================================================================

/// Request to create permits for a recurring contract.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePermitRequest {
    /// The merchant account identifier.
    pub merchant_account: Box<str>,
    /// The permits to create for this recurring contract.
    pub permits: Vec<Permit>,
    /// The recurring contract the new permits will use.
    pub recurring_detail_reference: Box<str>,
}

/// Result of creating permits.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePermitResult {
    /// Details of the created permits.
    #[serde(default)]
    pub permit_result_list: Vec<PermitResult>,
    /// PSP reference for tracking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub psp_reference: Option<Box<str>>,
}

/// Request to disable a permit.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DisablePermitRequest {
    /// The merchant account identifier.
    pub merchant_account: Box<str>,
    /// The token to disable.
    pub token: Box<str>,
}

/// Result of disabling a permit.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DisablePermitResult {
    /// Response status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response: Option<Box<str>>,
    /// PSP reference for tracking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub psp_reference: Option<Box<str>>,
}

/// Permit configuration for recurring contracts.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Permit {
    /// The partner that will receive permits for this recurring contract.
    pub partner: Box<str>,
    /// Optional restrictions for the permit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restriction: Option<PermitRestriction>,
}

/// Result of permit creation.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PermitResult {
    /// The partner.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner: Option<Box<str>>,
    /// Result code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_code: Option<Box<str>>,
    /// The token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<Box<str>>,
}

/// Restrictions for permits.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PermitRestriction {
    /// Maximum amount restriction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_amount: Option<Amount>,
    /// Single use restriction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_use: Option<bool>,
    /// Valid until date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_until: Option<Box<str>>,
}