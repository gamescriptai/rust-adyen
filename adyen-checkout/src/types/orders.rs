//! Order-related types for Checkout API.

use adyen_core::Amount;
use serde::{Deserialize, Serialize};

/// Request to create an order.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateOrderRequest {
    /// The merchant account identifier.
    pub merchant_account: String,
    /// The amount for the order.
    pub amount: Amount,
    /// The reference for the order.
    pub reference: String,
}

/// Response from creating an order.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateOrderResponse {
    /// The PSP reference for the order.
    pub psp_reference: String,
    /// The order data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_data: Option<String>,
    /// The amount for the order.
    pub amount: Amount,
    /// The remaining amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remaining_amount: Option<Amount>,
}

/// Request to cancel an order.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderRequest {
    /// The merchant account identifier.
    pub merchant_account: String,
    /// The order to cancel.
    pub order: OrderCancelData,
}

/// Order cancellation data.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderCancelData {
    /// The PSP reference of the order.
    pub psp_reference: String,
    /// The order data.
    pub order_data: String,
}

/// Response from canceling an order.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderResponse {
    /// The PSP reference for the cancellation.
    pub psp_reference: String,
    /// The result code.
    pub result_code: String,
}

/// Donation request.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DonationRequest {
    /// The merchant account identifier.
    pub merchant_account: String,
    /// The donation amount.
    pub donation_amount: Amount,
    /// The original PSP reference.
    pub original_psp_reference: String,
    /// The donation account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub donation_account: Option<String>,
}

/// Donation response.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DonationResponse {
    /// The PSP reference for the donation.
    pub psp_reference: String,
    /// The donation status.
    pub status: String,
    /// The donation amount.
    pub amount: Amount,
}

/// Request for donation campaigns.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DonationCampaignsRequest {
    /// The merchant account identifier.
    pub merchant_account: String,
}

/// Response with donation campaigns.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DonationCampaignsResponse {
    /// List of available donation campaigns.
    #[serde(default)]
    pub donation_campaigns: Vec<DonationCampaign>,
}

/// Donation campaign information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DonationCampaign {
    /// The campaign identifier.
    pub id: String,
    /// The campaign name.
    pub name: String,
    /// The campaign description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The campaign URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// `PayPal` update order request.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PayPalUpdateOrderRequest {
    /// The merchant account identifier.
    pub merchant_account: String,
    /// The PSP reference of the payment.
    pub psp_reference: String,
    /// `PayPal` order ID.
    pub paypal_order_id: String,
}

/// `PayPal` update order response.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PayPalUpdateOrderResponse {
    /// The PSP reference.
    pub psp_reference: String,
    /// The status of the update.
    pub status: String,
}
