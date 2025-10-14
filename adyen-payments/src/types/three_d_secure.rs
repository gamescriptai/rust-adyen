//! 3D Secure authentication types for Classic Payments API.

use adyen_core::{Amount, AdyenError, Result};
use serde::{Deserialize, Serialize};

/// Request for 3D Secure 1.0 authentication.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentRequest3d {
    /// The merchant account identifier.
    pub merchant_account: String,

    /// Browser information from the initial payment request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_info: Option<super::payment::BrowserInfo>,

    /// The MD parameter from the issuer response.
    pub md: String,

    /// The PaRes (Payer Authentication Response) from the issuer.
    pub pa_response: String,

    /// The shopper's IP address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shopper_ip: Option<String>,
}

/// Request for 3D Secure 2.0 authentication.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentRequest3ds2 {
    /// The merchant account identifier.
    pub merchant_account: String,

    /// The 3D Secure 2.0 result data.
    pub three_ds2_result: ThreeDS2Result,

    /// Browser information from the initial payment request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_info: Option<super::payment::BrowserInfo>,

    /// The shopper's IP address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shopper_ip: Option<String>,
}

/// 3D Secure data for payment requests.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThreeDSecureData {
    /// The authentication method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_method: Option<String>,

    /// The cavv (Cardholder Authentication Verification Value).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cavv: Option<String>,

    /// The cavv algorithm used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cavv_algorithm: Option<String>,

    /// The directory response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_response: Option<String>,

    /// The ECI (Electronic Commerce Indicator).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eci: Option<String>,

    /// The XID (Transaction Identifier).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xid: Option<String>,

    /// The 3D Secure version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub three_ds_version: Option<String>,
}

/// 3D Secure 2.0 request data.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThreeDS2RequestData {
    /// The device channel (browser, app, 3ri).
    pub device_channel: DeviceChannel,

    /// The message version.
    pub message_version: String,

    /// The notification URL for challenge completion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_url: Option<String>,

    /// The three DS requestor ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub three_ds_requestor_id: Option<String>,

    /// The three DS requestor name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub three_ds_requestor_name: Option<String>,

    /// The three DS requestor URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub three_ds_requestor_url: Option<String>,

    /// Account information for risk analysis.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acct_info: Option<AcctInfo>,

    /// Merchant risk indicator.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_risk_indicator: Option<MerchantRiskIndicator>,
}

/// Device channel for 3D Secure 2.0.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DeviceChannel {
    /// Browser-based authentication.
    Browser,
    /// App-based authentication.
    App,
    /// 3-RI (3DS Requestor Initiated) authentication.
    #[serde(rename = "3RI")]
    ThreeRi,
}

/// Account information for 3D Secure 2.0 risk analysis.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AcctInfo {
    /// Account age indicator.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acct_age_ind: Option<AccountAgeIndicator>,

    /// Account change indicator.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acct_change_ind: Option<AccountChangeIndicator>,

    /// Password change indicator.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_change_ind: Option<PasswordChangeIndicator>,

    /// Payment account age indicator.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_acct_age: Option<PaymentAccountAgeIndicator>,

    /// Shipping address usage indicator.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ship_address_usage_ind: Option<ShippingAddressUsageIndicator>,

    /// Number of transactions in last 24 hours.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txn_activity_day: Option<String>,

    /// Number of transactions in last year.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txn_activity_year: Option<String>,

    /// Number of add card attempts in last 24 hours.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nb_purchase_account: Option<String>,

    /// Suspicious account activity indicator.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suspicious_acct_activity: Option<SuspiciousAccountActivity>,
}

/// Account age indicators for 3D Secure 2.0.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AccountAgeIndicator {
    /// No account (guest checkout).
    NoAccount,
    /// Created during this transaction.
    CreatedDuringTransaction,
    /// Less than 30 days.
    LessThan30Days,
    /// 30-60 days.
    From30To60Days,
    /// More than 60 days.
    MoreThan60Days,
}

/// Account change indicators for 3D Secure 2.0.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AccountChangeIndicator {
    /// Changed during this transaction.
    ChangedDuringTransaction,
    /// Less than 30 days.
    LessThan30Days,
    /// 30-60 days.
    From30To60Days,
    /// More than 60 days.
    MoreThan60Days,
}

/// Password change indicators for 3D Secure 2.0.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PasswordChangeIndicator {
    /// No change.
    NoChange,
    /// Changed during this transaction.
    ChangedDuringTransaction,
    /// Less than 30 days.
    LessThan30Days,
    /// 30-60 days.
    From30To60Days,
    /// More than 60 days.
    MoreThan60Days,
}

/// Payment account age indicators for 3D Secure 2.0.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PaymentAccountAgeIndicator {
    /// No account.
    NoAccount,
    /// Added during this transaction.
    AddedDuringTransaction,
    /// Less than 30 days.
    LessThan30Days,
    /// 30-60 days.
    From30To60Days,
    /// More than 60 days.
    MoreThan60Days,
}

/// Shipping address usage indicators for 3D Secure 2.0.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ShippingAddressUsageIndicator {
    /// First time used.
    FirstTimeUsed,
    /// Less than 30 days.
    LessThan30Days,
    /// 30-60 days.
    From30To60Days,
    /// More than 60 days.
    MoreThan60Days,
}

/// Suspicious account activity indicators.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SuspiciousAccountActivity {
    /// No suspicious activity.
    NoSuspiciousActivity,
    /// Suspicious activity observed.
    SuspiciousActivityObserved,
}

/// Merchant risk indicator for 3D Secure 2.0.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MerchantRiskIndicator {
    /// Shipping indicator.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ship_indicator: Option<ShippingIndicator>,

    /// Delivery timeframe.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_timeframe: Option<DeliveryTimeframe>,

    /// Delivery email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_email_address: Option<String>,

    /// Reorder items indicator.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reorder_items_ind: Option<ReorderItemsIndicator>,

    /// Pre-order purchase indicator.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_order_purchase_ind: Option<PreOrderPurchaseIndicator>,

    /// Gift card amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gift_card_amount: Option<Amount>,

    /// Gift card count.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gift_card_count: Option<String>,
}

/// Shipping indicators for 3D Secure 2.0.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ShippingIndicator {
    /// Ship to cardholder billing address.
    ShipToBillingAddress,
    /// Ship to another verified address.
    ShipToVerifiedAddress,
    /// Ship to different address.
    ShipToDifferentAddress,
    /// Ship to store.
    ShipToStore,
    /// Digital goods.
    DigitalGoods,
    /// Travel and event tickets.
    TravelAndEventTickets,
    /// Other.
    Other,
}

/// Delivery timeframe indicators.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DeliveryTimeframe {
    /// Electronic delivery.
    ElectronicDelivery,
    /// Same day shipping.
    SameDayShipping,
    /// Overnight shipping.
    OvernightShipping,
    /// Two day or more shipping.
    TwoDayOrMoreShipping,
}

/// Reorder items indicators.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ReorderItemsIndicator {
    /// First time ordered.
    FirstTimeOrdered,
    /// Reordered.
    Reordered,
}

/// Pre-order purchase indicators.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PreOrderPurchaseIndicator {
    /// Merchandise available.
    MerchandiseAvailable,
    /// Future availability.
    FutureAvailability,
}

/// 3D Secure 2.0 result data.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThreeDS2Result {
    /// The challenge result.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cres: Option<String>,

    /// The directory server transaction ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ds_trans_id: Option<String>,

    /// The message version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_version: Option<String>,

    /// The three DS server transaction ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub three_ds_server_trans_id: Option<String>,

    /// The transaction status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trans_status: Option<String>,

    /// The ECI (Electronic Commerce Indicator).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eci: Option<String>,

    /// The cavv (Cardholder Authentication Verification Value).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cavv: Option<String>,

    /// The authentication value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_value: Option<String>,
}

/// Authentication result request for 3D Secure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticationResultRequest {
    /// The merchant account identifier.
    pub merchant_account: String,

    /// The PSP reference of the initial payment.
    pub psp_reference: String,
}

/// Authentication result response from 3D Secure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticationResultResponse {
    /// The 3D Secure 1.0 result.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub three_ds1_result: Option<ThreeDS1Result>,

    /// The 3D Secure 2.0 result.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub three_ds2_result: Option<ThreeDS2Result>,
}

/// 3D Secure 1.0 result data.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThreeDS1Result {
    /// The cavv (Cardholder Authentication Verification Value).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cavv: Option<String>,

    /// The cavv algorithm.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cavv_algorithm: Option<String>,

    /// The ECI (Electronic Commerce Indicator).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eci: Option<String>,

    /// The XID (Transaction Identifier).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xid: Option<String>,

    /// The enrolled status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enrolled: Option<String>,

    /// The signature verification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature_verification: Option<String>,
}

/// Builder for 3D Secure 1.0 requests.
#[derive(Debug, Clone, Default)]
pub struct PaymentRequest3dBuilder {
    merchant_account: Option<String>,
    browser_info: Option<super::payment::BrowserInfo>,
    md: Option<String>,
    pa_response: Option<String>,
    shopper_ip: Option<String>,
}

impl PaymentRequest3dBuilder {
    /// Create a new builder.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the merchant account.
    #[must_use]
    pub fn merchant_account(mut self, merchant_account: impl Into<String>) -> Self {
        self.merchant_account = Some(merchant_account.into());
        self
    }

    /// Set browser information.
    #[must_use]
    pub fn browser_info(mut self, browser_info: super::payment::BrowserInfo) -> Self {
        self.browser_info = Some(browser_info);
        self
    }

    /// Set the MD parameter.
    #[must_use]
    pub fn md(mut self, md: impl Into<String>) -> Self {
        self.md = Some(md.into());
        self
    }

    /// Set the PaRes parameter.
    #[must_use]
    pub fn pa_response(mut self, pa_response: impl Into<String>) -> Self {
        self.pa_response = Some(pa_response.into());
        self
    }

    /// Set the shopper IP.
    #[must_use]
    pub fn shopper_ip(mut self, shopper_ip: impl Into<String>) -> Self {
        self.shopper_ip = Some(shopper_ip.into());
        self
    }

    /// Build the 3D Secure 1.0 request.
    ///
    /// # Errors
    ///
    /// Returns an error if required fields are not set.
    pub fn build(self) -> Result<PaymentRequest3d> {
        let merchant_account = self.merchant_account
            .ok_or_else(|| AdyenError::config("merchant_account is required"))?;
        let md = self.md
            .ok_or_else(|| AdyenError::config("md is required"))?;
        let pa_response = self.pa_response
            .ok_or_else(|| AdyenError::config("pa_response is required"))?;

        Ok(PaymentRequest3d {
            merchant_account,
            browser_info: self.browser_info,
            md,
            pa_response,
            shopper_ip: self.shopper_ip,
        })
    }
}

impl PaymentRequest3d {
    /// Create a new builder for 3D Secure 1.0 requests.
    #[must_use]
    pub fn builder() -> PaymentRequest3dBuilder {
        PaymentRequest3dBuilder::new()
    }
}

/// Builder for 3D Secure 2.0 requests.
#[derive(Debug, Clone, Default)]
pub struct PaymentRequest3ds2Builder {
    merchant_account: Option<String>,
    three_ds2_result: Option<ThreeDS2Result>,
    browser_info: Option<super::payment::BrowserInfo>,
    shopper_ip: Option<String>,
}

impl PaymentRequest3ds2Builder {
    /// Create a new builder.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the merchant account.
    #[must_use]
    pub fn merchant_account(mut self, merchant_account: impl Into<String>) -> Self {
        self.merchant_account = Some(merchant_account.into());
        self
    }

    /// Set the 3D Secure 2.0 result.
    #[must_use]
    pub fn three_ds2_result(mut self, result: ThreeDS2Result) -> Self {
        self.three_ds2_result = Some(result);
        self
    }

    /// Set browser information.
    #[must_use]
    pub fn browser_info(mut self, browser_info: super::payment::BrowserInfo) -> Self {
        self.browser_info = Some(browser_info);
        self
    }

    /// Set the shopper IP.
    #[must_use]
    pub fn shopper_ip(mut self, shopper_ip: impl Into<String>) -> Self {
        self.shopper_ip = Some(shopper_ip.into());
        self
    }

    /// Build the 3D Secure 2.0 request.
    ///
    /// # Errors
    ///
    /// Returns an error if required fields are not set.
    pub fn build(self) -> Result<PaymentRequest3ds2> {
        let merchant_account = self.merchant_account
            .ok_or_else(|| AdyenError::config("merchant_account is required"))?;
        let three_ds2_result = self.three_ds2_result
            .ok_or_else(|| AdyenError::config("three_ds2_result is required"))?;

        Ok(PaymentRequest3ds2 {
            merchant_account,
            three_ds2_result,
            browser_info: self.browser_info,
            shopper_ip: self.shopper_ip,
        })
    }
}

impl PaymentRequest3ds2 {
    /// Create a new builder for 3D Secure 2.0 requests.
    #[must_use]
    pub fn builder() -> PaymentRequest3ds2Builder {
        PaymentRequest3ds2Builder::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_payment_request_3d_builder() {
        let request = PaymentRequest3d::builder()
            .merchant_account("TestMerchant")
            .md("test_md_value")
            .pa_response("test_pares")
            .shopper_ip("192.168.1.1")
            .build()
            .unwrap();

        assert_eq!(request.merchant_account, "TestMerchant");
        assert_eq!(request.md, "test_md_value");
        assert_eq!(request.pa_response, "test_pares");
        assert_eq!(request.shopper_ip, Some("192.168.1.1".to_string()));
    }

    #[test]
    fn test_payment_request_3ds2_builder() {
        let result = ThreeDS2Result {
            cres: Some("test_cres".to_string()),
            ds_trans_id: Some("test_ds_trans_id".to_string()),
            message_version: Some("2.1.0".to_string()),
            three_ds_server_trans_id: Some("test_server_trans_id".to_string()),
            trans_status: Some("Y".to_string()),
            eci: Some("05".to_string()),
            cavv: Some("test_cavv".to_string()),
            authentication_value: Some("test_auth_value".to_string()),
        };

        let request = PaymentRequest3ds2::builder()
            .merchant_account("TestMerchant")
            .three_ds2_result(result.clone())
            .shopper_ip("192.168.1.1")
            .build()
            .unwrap();

        assert_eq!(request.merchant_account, "TestMerchant");
        assert_eq!(request.three_ds2_result, result);
        assert_eq!(request.shopper_ip, Some("192.168.1.1".to_string()));
    }

    #[test]
    fn test_device_channel_serialization() {
        assert_eq!(serde_json::to_string(&DeviceChannel::Browser).unwrap(), "\"browser\"");
        assert_eq!(serde_json::to_string(&DeviceChannel::App).unwrap(), "\"app\"");
        assert_eq!(serde_json::to_string(&DeviceChannel::ThreeRi).unwrap(), "\"3RI\"");
    }

    #[test]
    fn test_account_age_indicator_serialization() {
        assert_eq!(
            serde_json::to_string(&AccountAgeIndicator::NoAccount).unwrap(),
            "\"noAccount\""
        );
        assert_eq!(
            serde_json::to_string(&AccountAgeIndicator::LessThan30Days).unwrap(),
            "\"lessThan30Days\""
        );
    }

    #[test]
    fn test_shipping_indicator_serialization() {
        assert_eq!(
            serde_json::to_string(&ShippingIndicator::ShipToBillingAddress).unwrap(),
            "\"shipToBillingAddress\""
        );
        assert_eq!(
            serde_json::to_string(&ShippingIndicator::DigitalGoods).unwrap(),
            "\"digitalGoods\""
        );
    }
}