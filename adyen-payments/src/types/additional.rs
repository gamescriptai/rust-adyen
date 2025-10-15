//! Additional types for extended Classic Payments API functionality.

use adyen_core::Amount;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Request to adjust an authorization amount.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdjustAuthorisationRequest {
    /// The merchant account identifier.
    pub merchant_account: String,
    /// The amount to adjust the authorization to.
    pub modification_amount: Amount,
    /// The original PSP reference of the payment to modify.
    pub original_reference: String,
    /// Additional data for the adjustment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<HashMap<String, String>>,
    /// 3D Secure data for the adjustment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpi_data: Option<serde_json::Value>,
    /// The original merchant reference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_merchant_reference: Option<String>,
}

/// Request to make a donation.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DonateRequest {
    /// The merchant account identifier.
    pub merchant_account: String,
    /// The donation amount.
    pub modification_amount: Amount,
    /// The original PSP reference of the payment.
    pub original_reference: String,
    /// The donation account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub donation_account: Option<String>,
    /// Additional data for the donation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<HashMap<String, String>>,
}

// AuthenticationResultRequest and AuthenticationResultResponse are already defined in three_d_secure.rs

/// Request to retrieve 3DS2 result.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThreeDSResultRequest {
    /// The merchant account identifier.
    pub merchant_account: String,
    /// The PSP reference of the authentication.
    pub psp_reference: String,
}

/// Response from 3DS2 result request.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThreeDSResultResponse {
    /// The PSP reference.
    pub psp_reference: String,
    /// The 3DS2 result.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub three_ds2_result: Option<serde_json::Value>,
    /// The authentication value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_value: Option<String>,
}

/// Request to perform technical cancel.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TechnicalCancelRequest {
    /// The merchant account identifier.
    pub merchant_account: String,
    /// The original PSP reference of the payment to cancel.
    pub original_reference: String,
    /// Additional data for the cancellation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<HashMap<String, String>>,
}

/// Request to void a pending refund.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VoidPendingRefundRequest {
    /// The merchant account identifier.
    pub merchant_account: String,
    /// The original PSP reference of the refund to void.
    pub original_reference: String,
    /// Additional data for the void operation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<HashMap<String, String>>,
}