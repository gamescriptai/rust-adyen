//! Core webhook types for Adyen webhook processing.
//!
//! This module contains the fundamental types used across all Adyen webhook implementations,
//! including the base webhook structure, notification items, and common event types.

use adyen_core::Currency;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Amount representation for webhooks.
///
/// This structure matches the amount format used in Adyen webhooks,
/// which uses `value` (minor units) and `currency` fields.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize))]
pub struct Amount {
    /// The payment amount in minor units (e.g., cents).
    pub value: i64,
    /// The three-character ISO currency code.
    pub currency: String,
}

impl Amount {
    /// Create a new amount from minor units and currency.
    pub fn new(value: i64, currency: impl Into<String>) -> Self {
        Self {
            value,
            currency: currency.into(),
        }
    }

    /// Create an amount from major units (e.g., dollars).
    pub fn from_major_units(major_units: i64, currency: Currency) -> Self {
        Self {
            value: major_units * currency.minor_unit_multiplier() as i64,
            currency: currency.to_string(),
        }
    }

    /// Get the amount in minor units.
    pub fn minor_units(&self) -> i64 {
        self.value
    }

    /// Get the currency as a string.
    pub fn currency_string(&self) -> &str {
        &self.currency
    }

    /// Convert to adyen_core::Amount if possible.
    pub fn to_core_amount(&self) -> Result<adyen_core::Amount, String> {
        let currency = self.currency.parse::<Currency>()
            .map_err(|e| format!("Invalid currency: {}", e))?;

        Ok(adyen_core::Amount::from_minor_units(self.value as u64, currency))
    }
}

/// Main webhook payload containing one or more notification items.
///
/// All Adyen webhooks follow this structure, containing a `live` field indicating
/// the environment and an array of notification items with the actual event data.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize))]
#[serde(rename_all = "camelCase")]
pub struct Webhook {
    /// Indicates whether this webhook was sent from the live environment.
    /// "true" for live, "false" for test environment.
    pub live: String,
    /// Array of notification items containing the actual webhook events.
    #[serde(default)]
    pub notification_items: Vec<NotificationItem>,
}

/// Container for a single notification request item.
///
/// This is a wrapper around `NotificationRequestItem` that matches Adyen's webhook structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize))]
pub struct NotificationItem {
    /// The actual notification request item containing event data.
    #[serde(rename = "NotificationRequestItem")]
    pub notification_request_item: NotificationRequestItem,
}

/// Core notification request item containing webhook event data.
///
/// This structure contains all the essential information about a webhook event,
/// including references, amounts, event details, and additional data.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize))]
#[serde(rename_all = "camelCase")]
pub struct NotificationRequestItem {
    /// Additional data provided with the webhook event.
    /// May contain HMAC signature and other event-specific information.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "rkyv", rkyv(skip))]
    pub additional_data: Option<HashMap<String, serde_json::Value>>,
    /// The payment amount associated with this event.
    pub amount: Amount,
    /// The type of event that triggered this webhook.
    pub event_code: String,
    /// The date and time when the event occurred.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "rkyv", rkyv(skip))]
    pub event_date: Option<DateTime<Utc>>,
    /// The merchant account identifier.
    pub merchant_account_code: String,
    /// The merchant's reference for this payment/transaction.
    pub merchant_reference: String,
    /// Array of operations that can be performed on this payment.
    #[serde(default)]
    pub operations: Vec<String>,
    /// Reference to the original request (for modifications).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_reference: Option<String>,
    /// The payment method used.
    pub payment_method: String,
    /// Adyen's unique reference for this payment/transaction.
    pub psp_reference: String,
    /// Reason for the event (e.g., fraud check results).
    pub reason: String,
    /// Indicates whether the operation was successful.
    /// "true" for successful, "false" for failed operations.
    pub success: String,
}

/// Event codes that can be sent via Adyen webhooks.
///
/// This enum contains all the possible event types that Adyen can send,
/// covering payments, modifications, disputes, and administrative events.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize))]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EventCode {
    /// ACH notification of change event.
    AchNotificationOfChange,
    /// Initial payment authorization.
    Authorisation,
    /// Adjustment to a previous authorization.
    AuthorisationAdjustment,
    /// Automatic rescue attempt for failed payment.
    Autorescue,
    /// Next attempt in automatic rescue sequence.
    AutorescueNextAttempt,
    /// Payment cancellation.
    Cancellation,
    /// Cancellation of automatic rescue.
    CancelAutorescue,
    /// Cancel or refund operation.
    CancelOrRefund,
    /// Payment capture (settlement).
    Capture,
    /// Failed capture attempt.
    CaptureFailed,
    /// Chargeback initiated.
    Chargeback,
    /// Chargeback reversed.
    ChargebackReversed,
    /// Payment expired.
    Expire,
    /// Comments from the issuing bank.
    IssuerComments,
    /// Event handled externally.
    HandledExternally,
    /// Manual review accepted.
    ManualReviewAccept,
    /// Manual review rejected.
    ManualReviewReject,
    /// Notification of chargeback.
    NotificationOfChargeback,
    /// Notification of fraud.
    NotificationOfFraud,
    /// Offer closed.
    OfferClosed,
    /// Payout reversed.
    PaidoutReversed,
    /// Payout declined.
    PayoutDecline,
    /// Payout expired.
    PayoutExpire,
    /// Third-party payout.
    PayoutThirdparty,
    /// Postponed refund.
    PostponedRefund,
    /// Pre-arbitration lost.
    PrearbitrationLost,
    /// Pre-arbitration won.
    PrearbitrationWon,
    /// Recurring contract established.
    RecurringContract,
    /// Payment refund.
    Refund,
    /// Failed refund attempt.
    RefundFailed,
    /// Refund with additional data.
    RefundWithData,
    /// Refund reversed.
    RefundedReversed,
    /// Report available for download.
    ReportAvailable,
    /// Request for information from issuer.
    RequestForInformation,
    /// Second chargeback.
    SecondChargeback,
    /// Technical cancellation.
    TechnicalCancel,
    /// Void pending refund.
    VoidPendingRefund,
    /// Order closed.
    OrderClosed,
    /// Order opened.
    OrderOpened,
}

impl Webhook {
    /// Get all notification request items from this webhook.
    ///
    /// This convenience method extracts all `NotificationRequestItem` objects
    /// from the notification items array for easier processing.
    pub fn get_notification_items(&self) -> Vec<&NotificationRequestItem> {
        self.notification_items
            .iter()
            .map(|item| &item.notification_request_item)
            .collect()
    }

    /// Check if this webhook is from the live environment.
    pub fn is_live(&self) -> bool {
        self.live == "true"
    }

    /// Check if this webhook is from the test environment.
    pub fn is_test(&self) -> bool {
        self.live == "false"
    }
}

impl NotificationRequestItem {
    /// Check if this notification represents a successful operation.
    pub fn is_success(&self) -> bool {
        self.success == "true"
    }

    /// Check if this notification represents a failed operation.
    pub fn is_failure(&self) -> bool {
        self.success == "false"
    }

    /// Get the HMAC signature from additional data, if present.
    pub fn hmac_signature(&self) -> Option<String> {
        self.additional_data
            .as_ref()?
            .get("hmacSignature")?
            .as_str()
            .map(String::from)
    }

    /// Get a specific value from additional data.
    pub fn get_additional_data(&self, key: &str) -> Option<&serde_json::Value> {
        self.additional_data.as_ref()?.get(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_webhook_parsing() {
        let webhook_json = r#"{
            "live": "false",
            "notificationItems": [
                {
                    "NotificationRequestItem": {
                        "amount": {
                            "currency": "EUR",
                            "value": 1000
                        },
                        "eventCode": "AUTHORISATION",
                        "merchantAccountCode": "TestMerchant",
                        "merchantReference": "test-payment-123",
                        "operations": ["CAPTURE", "REFUND"],
                        "paymentMethod": "visa",
                        "pspReference": "8515131751004933",
                        "reason": "Approved",
                        "success": "true"
                    }
                }
            ]
        }"#;

        let webhook: Webhook = serde_json::from_str(webhook_json).unwrap();
        assert!(!webhook.is_live());
        assert!(webhook.is_test());
        assert_eq!(webhook.notification_items.len(), 1);

        let item = &webhook.notification_items[0].notification_request_item;
        assert!(item.is_success());
        assert!(!item.is_failure());
        assert_eq!(item.event_code, "AUTHORISATION");
        assert_eq!(item.merchant_reference, "test-payment-123");
        assert_eq!(item.amount.minor_units(), 1000);
        assert_eq!(item.amount.currency_string(), "EUR");
    }

    #[test]
    fn test_event_code_serialization() {
        let event = EventCode::Authorisation;
        let json = serde_json::to_string(&event).unwrap();
        assert_eq!(json, r#""AUTHORISATION""#);

        let parsed: EventCode = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, EventCode::Authorisation);
    }

    #[test]
    fn test_notification_item_additional_data() {
        let mut additional_data = HashMap::new();
        additional_data.insert(
            "hmacSignature".to_string(),
            serde_json::Value::String("test_signature".to_string()),
        );
        additional_data.insert(
            "totalFraudScore".to_string(),
            serde_json::Value::Number(serde_json::Number::from(75)),
        );

        let item = NotificationRequestItem {
            additional_data: Some(additional_data),
            amount: Amount::new(1000, "EUR"),
            event_code: "AUTHORISATION".to_string(),
            event_date: None,
            merchant_account_code: "TestMerchant".to_string(),
            merchant_reference: "test-ref".to_string(),
            operations: vec![],
            original_reference: None,
            payment_method: "visa".to_string(),
            psp_reference: "12345".to_string(),
            reason: "test".to_string(),
            success: "true".to_string(),
        };

        assert_eq!(item.hmac_signature(), Some("test_signature".to_string()));
        assert_eq!(
            item.get_additional_data("totalFraudScore"),
            Some(&serde_json::Value::Number(serde_json::Number::from(75)))
        );
        assert_eq!(item.get_additional_data("nonexistent"), None);
    }
}