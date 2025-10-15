//! Payment modification types for captures, refunds, cancels, etc.

use adyen_core::Amount;
use serde::{Deserialize, Serialize};

/// Request to capture a payment.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CaptureRequest {
    /// The merchant account identifier.
    pub merchant_account: String,
    /// The amount to capture.
    pub amount: Amount,
    /// The reference for the capture.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
}

/// Response from a capture request.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CaptureResponse {
    /// The PSP reference of the capture.
    pub psp_reference: String,
    /// The status of the capture.
    pub status: String,
    /// The merchant account.
    pub merchant_account: String,
    /// The amount that was captured.
    pub amount: Amount,
}

/// Request to refund a payment.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RefundRequest {
    /// The merchant account identifier.
    pub merchant_account: String,
    /// The amount to refund.
    pub amount: Amount,
    /// The reference for the refund.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
}

/// Response from a refund request.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RefundResponse {
    /// The PSP reference of the refund.
    pub psp_reference: String,
    /// The status of the refund.
    pub status: String,
    /// The merchant account.
    pub merchant_account: String,
    /// The amount that was refunded.
    pub amount: Amount,
}

/// Request to cancel a payment.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelRequest {
    /// The merchant account identifier.
    pub merchant_account: String,
    /// The reference for the cancellation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
}

/// Response from a cancel request.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelResponse {
    /// The PSP reference of the cancellation.
    pub psp_reference: String,
    /// The status of the cancellation.
    pub status: String,
    /// The merchant account.
    pub merchant_account: String,
}

/// Request to reverse a payment.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReversalRequest {
    /// The merchant account identifier.
    pub merchant_account: String,
    /// The reference for the reversal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
}

/// Response from a reversal request.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReversalResponse {
    /// The PSP reference of the reversal.
    pub psp_reference: String,
    /// The status of the reversal.
    pub status: String,
    /// The merchant account.
    pub merchant_account: String,
}

/// Request to update the amount of a payment.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AmountUpdateRequest {
    /// The merchant account identifier.
    pub merchant_account: String,
    /// The new amount for the payment.
    pub amount: Amount,
    /// The reference for the amount update.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
}

/// Response from an amount update request.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AmountUpdateResponse {
    /// The PSP reference of the amount update.
    pub psp_reference: String,
    /// The status of the amount update.
    pub status: String,
    /// The merchant account.
    pub merchant_account: String,
    /// The updated amount.
    pub amount: Amount,
}
