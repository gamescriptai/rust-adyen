//! Payment modification types for capture, cancel, refund, and adjustment operations.

#![allow(clippy::type_complexity)]
#![allow(clippy::return_self_not_must_use)]

use crate::types::ApplicationInfo;
use adyen_core::Amount;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Type alias for additional data maps to reduce complexity warnings.
pub type AdditionalData = HashMap<Box<str>, Box<str>>;

/// Request to capture an authorized payment.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CaptureRequest {
    /// The merchant account identifier.
    pub merchant_account: Box<str>,

    /// The amount to capture (must not exceed the authorized amount).
    pub modification_amount: Amount,

    /// The PSP reference of the payment to capture.
    pub original_reference: Box<str>,

    /// Your reference for this capture request.
    pub reference: Option<Box<str>>,

    /// Additional data for the capture.
    pub additional_data: Option<HashMap<Box<str>, Box<str>>>,

    /// Application information.
    pub application_info: Option<ApplicationInfo>,
}

impl CaptureRequest {
    /// Create a new capture request builder.
    #[must_use]
    pub fn builder() -> CaptureRequestBuilder {
        CaptureRequestBuilder::default()
    }
}

/// Builder for capture requests.
#[derive(Debug, Default)]
pub struct CaptureRequestBuilder {
    merchant_account: Option<Box<str>>,
    modification_amount: Option<Amount>,
    original_reference: Option<Box<str>>,
    reference: Option<Box<str>>,
    additional_data: Option<HashMap<Box<str>, Box<str>>>,
    application_info: Option<ApplicationInfo>,
}

impl CaptureRequestBuilder {
    /// Set the merchant account.
    pub fn merchant_account<S: Into<Box<str>>>(mut self, account: S) -> Self {
        self.merchant_account = Some(account.into());
        self
    }

    /// Set the modification amount.
    #[must_use]
    pub fn modification_amount(mut self, amount: Amount) -> Self {
        self.modification_amount = Some(amount);
        self
    }

    /// Set the original payment reference.
    pub fn original_reference<S: Into<Box<str>>>(mut self, reference: S) -> Self {
        self.original_reference = Some(reference.into());
        self
    }

    /// Set the capture reference.
    pub fn reference<S: Into<Box<str>>>(mut self, reference: S) -> Self {
        self.reference = Some(reference.into());
        self
    }

    /// Add additional data.
    #[must_use]
    pub fn additional_data(mut self, data: HashMap<Box<str>, Box<str>>) -> Self {
        self.additional_data = Some(data);
        self
    }

    /// Set application info.
    #[must_use]
    pub fn application_info(mut self, info: ApplicationInfo) -> Self {
        self.application_info = Some(info);
        self
    }

    /// Build the capture request.
    ///
    /// # Errors
    ///
    /// Returns an error if required fields are missing.
    pub fn build(self) -> Result<CaptureRequest, Box<str>> {
        Ok(CaptureRequest {
            merchant_account: self
                .merchant_account
                .ok_or("merchant_account is required")?,
            modification_amount: self
                .modification_amount
                .ok_or("modification_amount is required")?,
            original_reference: self
                .original_reference
                .ok_or("original_reference is required")?,
            reference: self.reference,
            additional_data: self.additional_data,
            application_info: self.application_info,
        })
    }
}

/// Request to cancel an authorized payment.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelRequest {
    /// The merchant account identifier.
    pub merchant_account: Box<str>,

    /// The PSP reference of the payment to cancel.
    pub original_reference: Box<str>,

    /// Your reference for this cancel request.
    pub reference: Option<Box<str>>,

    /// Additional data for the cancellation.
    pub additional_data: Option<HashMap<Box<str>, Box<str>>>,

    /// Application information.
    pub application_info: Option<ApplicationInfo>,
}

impl CancelRequest {
    /// Create a new cancel request builder.
    #[must_use]
    pub fn builder() -> CancelRequestBuilder {
        CancelRequestBuilder::default()
    }
}

/// Builder for cancel requests.
#[derive(Debug, Default)]
pub struct CancelRequestBuilder {
    merchant_account: Option<Box<str>>,
    original_reference: Option<Box<str>>,
    reference: Option<Box<str>>,
    additional_data: Option<HashMap<Box<str>, Box<str>>>,
    application_info: Option<ApplicationInfo>,
}

impl CancelRequestBuilder {
    /// Set the merchant account.
    pub fn merchant_account<S: Into<Box<str>>>(mut self, account: S) -> Self {
        self.merchant_account = Some(account.into());
        self
    }

    /// Set the original payment reference.
    pub fn original_reference<S: Into<Box<str>>>(mut self, reference: S) -> Self {
        self.original_reference = Some(reference.into());
        self
    }

    /// Set the cancel reference.
    pub fn reference<S: Into<Box<str>>>(mut self, reference: S) -> Self {
        self.reference = Some(reference.into());
        self
    }

    /// Add additional data.
    #[must_use]
    pub fn additional_data(mut self, data: HashMap<Box<str>, Box<str>>) -> Self {
        self.additional_data = Some(data);
        self
    }

    /// Set application info.
    #[must_use]
    pub fn application_info(mut self, info: ApplicationInfo) -> Self {
        self.application_info = Some(info);
        self
    }

    /// Build the cancel request.
    ///
    /// # Errors
    ///
    /// Returns an error if required fields are missing.
    pub fn build(self) -> Result<CancelRequest, Box<str>> {
        Ok(CancelRequest {
            merchant_account: self
                .merchant_account
                .ok_or("merchant_account is required")?,
            original_reference: self
                .original_reference
                .ok_or("original_reference is required")?,
            reference: self.reference,
            additional_data: self.additional_data,
            application_info: self.application_info,
        })
    }
}

/// Request to refund a captured payment.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RefundRequest {
    /// The merchant account identifier.
    pub merchant_account: Box<str>,

    /// The amount to refund (must not exceed the captured amount).
    pub modification_amount: Amount,

    /// The PSP reference of the payment to refund.
    pub original_reference: Box<str>,

    /// Your reference for this refund request.
    pub reference: Option<Box<str>>,

    /// Additional data for the refund.
    pub additional_data: Option<HashMap<Box<str>, Box<str>>>,

    /// Application information.
    pub application_info: Option<ApplicationInfo>,
}

impl RefundRequest {
    /// Create a new refund request builder.
    #[must_use]
    pub fn builder() -> RefundRequestBuilder {
        RefundRequestBuilder::default()
    }
}

/// Builder for refund requests.
#[derive(Debug, Default)]
pub struct RefundRequestBuilder {
    merchant_account: Option<Box<str>>,
    modification_amount: Option<Amount>,
    original_reference: Option<Box<str>>,
    reference: Option<Box<str>>,
    additional_data: Option<HashMap<Box<str>, Box<str>>>,
    application_info: Option<ApplicationInfo>,
}

impl RefundRequestBuilder {
    /// Set the merchant account.
    pub fn merchant_account<S: Into<Box<str>>>(mut self, account: S) -> Self {
        self.merchant_account = Some(account.into());
        self
    }

    /// Set the modification amount.
    #[must_use]
    pub fn modification_amount(mut self, amount: Amount) -> Self {
        self.modification_amount = Some(amount);
        self
    }

    /// Set the original payment reference.
    pub fn original_reference<S: Into<Box<str>>>(mut self, reference: S) -> Self {
        self.original_reference = Some(reference.into());
        self
    }

    /// Set the refund reference.
    pub fn reference<S: Into<Box<str>>>(mut self, reference: S) -> Self {
        self.reference = Some(reference.into());
        self
    }

    /// Add additional data.
    #[must_use]
    pub fn additional_data(mut self, data: HashMap<Box<str>, Box<str>>) -> Self {
        self.additional_data = Some(data);
        self
    }

    /// Set application info.
    #[must_use]
    pub fn application_info(mut self, info: ApplicationInfo) -> Self {
        self.application_info = Some(info);
        self
    }

    /// Build the refund request.
    ///
    /// # Errors
    ///
    /// Returns an error if required fields are missing.
    pub fn build(self) -> Result<RefundRequest, Box<str>> {
        Ok(RefundRequest {
            merchant_account: self
                .merchant_account
                .ok_or("merchant_account is required")?,
            modification_amount: self
                .modification_amount
                .ok_or("modification_amount is required")?,
            original_reference: self
                .original_reference
                .ok_or("original_reference is required")?,
            reference: self.reference,
            additional_data: self.additional_data,
            application_info: self.application_info,
        })
    }
}

/// Request to cancel or refund a payment automatically.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrRefundRequest {
    /// The merchant account identifier.
    pub merchant_account: Box<str>,

    /// The PSP reference of the payment to cancel or refund.
    pub original_reference: Box<str>,

    /// Your reference for this request.
    pub reference: Option<Box<str>>,

    /// Additional data for the operation.
    pub additional_data: Option<HashMap<Box<str>, Box<str>>>,

    /// Application information.
    pub application_info: Option<ApplicationInfo>,
}

impl CancelOrRefundRequest {
    /// Create a new cancel or refund request builder.
    #[must_use]
    pub fn builder() -> CancelOrRefundRequestBuilder {
        CancelOrRefundRequestBuilder::default()
    }
}

/// Builder for cancel or refund requests.
#[derive(Debug, Default)]
pub struct CancelOrRefundRequestBuilder {
    merchant_account: Option<Box<str>>,
    original_reference: Option<Box<str>>,
    reference: Option<Box<str>>,
    additional_data: Option<HashMap<Box<str>, Box<str>>>,
    application_info: Option<ApplicationInfo>,
}

impl CancelOrRefundRequestBuilder {
    /// Set the merchant account.
    pub fn merchant_account<S: Into<Box<str>>>(mut self, account: S) -> Self {
        self.merchant_account = Some(account.into());
        self
    }

    /// Set the original payment reference.
    pub fn original_reference<S: Into<Box<str>>>(mut self, reference: S) -> Self {
        self.original_reference = Some(reference.into());
        self
    }

    /// Set the request reference.
    pub fn reference<S: Into<Box<str>>>(mut self, reference: S) -> Self {
        self.reference = Some(reference.into());
        self
    }

    /// Add additional data.
    #[must_use]
    pub fn additional_data(mut self, data: HashMap<Box<str>, Box<str>>) -> Self {
        self.additional_data = Some(data);
        self
    }

    /// Set application info.
    #[must_use]
    pub fn application_info(mut self, info: ApplicationInfo) -> Self {
        self.application_info = Some(info);
        self
    }

    /// Build the cancel or refund request.
    ///
    /// # Errors
    ///
    /// Returns an error if required fields are missing.
    pub fn build(self) -> Result<CancelOrRefundRequest, Box<str>> {
        Ok(CancelOrRefundRequest {
            merchant_account: self
                .merchant_account
                .ok_or("merchant_account is required")?,
            original_reference: self
                .original_reference
                .ok_or("original_reference is required")?,
            reference: self.reference,
            additional_data: self.additional_data,
            application_info: self.application_info,
        })
    }
}

/// Response codes for modification operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ModificationResponse {
    /// The modification was received successfully.
    #[serde(rename = "[capture-received]")]
    CaptureReceived,
    /// The modification was received successfully.
    #[serde(rename = "[cancel-received]")]
    CancelReceived,
    /// The modification was received successfully.
    #[serde(rename = "[refund-received]")]
    RefundReceived,
    /// The modification was received successfully.
    #[serde(rename = "[cancelOrRefund-received]")]
    CancelOrRefundReceived,
}

/// Result of a payment modification operation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModificationResult {
    /// Additional data returned by Adyen.
    pub additional_data: Option<HashMap<Box<str>, Box<str>>>,

    /// The PSP reference for this modification.
    pub psp_reference: Box<str>,

    /// The response indicating the result of the modification.
    pub response: ModificationResponse,
}

/// Request to adjust an authorized amount.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdjustAuthorizationRequest {
    /// The merchant account identifier.
    pub merchant_account: Box<str>,

    /// The new authorized amount.
    pub modification_amount: Amount,

    /// The PSP reference of the payment to adjust.
    pub original_reference: Box<str>,

    /// Your reference for this adjustment request.
    pub reference: Option<Box<str>>,

    /// Additional data for the adjustment.
    pub additional_data: Option<HashMap<Box<str>, Box<str>>>,

    /// Application information.
    pub application_info: Option<ApplicationInfo>,
}

impl AdjustAuthorizationRequest {
    /// Create a new adjust authorization request builder.
    #[must_use]
    pub fn builder() -> AdjustAuthorizationRequestBuilder {
        AdjustAuthorizationRequestBuilder::default()
    }
}

/// Builder for adjust authorization requests.
#[derive(Debug, Default)]
pub struct AdjustAuthorizationRequestBuilder {
    merchant_account: Option<Box<str>>,
    modification_amount: Option<Amount>,
    original_reference: Option<Box<str>>,
    reference: Option<Box<str>>,
    additional_data: Option<HashMap<Box<str>, Box<str>>>,
    application_info: Option<ApplicationInfo>,
}

impl AdjustAuthorizationRequestBuilder {
    /// Set the merchant account.
    pub fn merchant_account<S: Into<Box<str>>>(mut self, account: S) -> Self {
        self.merchant_account = Some(account.into());
        self
    }

    /// Set the modification amount.
    #[must_use]
    pub fn modification_amount(mut self, amount: Amount) -> Self {
        self.modification_amount = Some(amount);
        self
    }

    /// Set the original payment reference.
    pub fn original_reference<S: Into<Box<str>>>(mut self, reference: S) -> Self {
        self.original_reference = Some(reference.into());
        self
    }

    /// Set the adjustment reference.
    pub fn reference<S: Into<Box<str>>>(mut self, reference: S) -> Self {
        self.reference = Some(reference.into());
        self
    }

    /// Add additional data.
    #[must_use]
    pub fn additional_data(mut self, data: HashMap<Box<str>, Box<str>>) -> Self {
        self.additional_data = Some(data);
        self
    }

    /// Set application info.
    #[must_use]
    pub fn application_info(mut self, info: ApplicationInfo) -> Self {
        self.application_info = Some(info);
        self
    }

    /// Build the adjust authorization request.
    ///
    /// # Errors
    ///
    /// Returns an error if required fields are missing.
    pub fn build(self) -> Result<AdjustAuthorizationRequest, Box<str>> {
        Ok(AdjustAuthorizationRequest {
            merchant_account: self
                .merchant_account
                .ok_or("merchant_account is required")?,
            modification_amount: self
                .modification_amount
                .ok_or("modification_amount is required")?,
            original_reference: self
                .original_reference
                .ok_or("original_reference is required")?,
            reference: self.reference,
            additional_data: self.additional_data,
            application_info: self.application_info,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use adyen_core::{Amount, Currency};
    use serde_json;

    #[test]
    fn test_capture_request_builder() {
        let request = CaptureRequest::builder()
            .merchant_account("TestMerchant")
            .modification_amount(Amount::from_major_units(100, Currency::EUR))
            .original_reference("8515131751004933")
            .reference("capture-123")
            .build()
            .unwrap();

        assert_eq!(request.merchant_account.as_ref(), "TestMerchant");
        assert_eq!(request.modification_amount.minor_units(), 10000);
        assert_eq!(request.original_reference.as_ref(), "8515131751004933");
        assert_eq!(request.reference.as_deref(), Some("capture-123"));
    }

    #[test]
    fn test_cancel_request_builder() {
        let request = CancelRequest::builder()
            .merchant_account("TestMerchant")
            .original_reference("8515131751004933")
            .reference("cancel-123")
            .build()
            .unwrap();

        assert_eq!(request.merchant_account.as_ref(), "TestMerchant");
        assert_eq!(request.original_reference.as_ref(), "8515131751004933");
        assert_eq!(request.reference.as_deref(), Some("cancel-123"));
    }

    #[test]
    fn test_refund_request_builder() {
        let request = RefundRequest::builder()
            .merchant_account("TestMerchant")
            .modification_amount(Amount::from_major_units(50, Currency::USD))
            .original_reference("8515131751004933")
            .reference("refund-123")
            .build()
            .unwrap();

        assert_eq!(request.merchant_account.as_ref(), "TestMerchant");
        assert_eq!(request.modification_amount.minor_units(), 5000);
        assert_eq!(request.modification_amount.currency(), Currency::USD);
        assert_eq!(request.original_reference.as_ref(), "8515131751004933");
        assert_eq!(request.reference.as_deref(), Some("refund-123"));
    }

    #[test]
    fn test_cancel_or_refund_request_builder() {
        let request = CancelOrRefundRequest::builder()
            .merchant_account("TestMerchant")
            .original_reference("8515131751004933")
            .reference("cancel-or-refund-123")
            .build()
            .unwrap();

        assert_eq!(request.merchant_account.as_ref(), "TestMerchant");
        assert_eq!(request.original_reference.as_ref(), "8515131751004933");
        assert_eq!(request.reference.as_deref(), Some("cancel-or-refund-123"));
    }

    #[test]
    fn test_adjust_authorization_request_builder() {
        let request = AdjustAuthorizationRequest::builder()
            .merchant_account("TestMerchant")
            .modification_amount(Amount::from_major_units(150, Currency::GBP))
            .original_reference("8515131751004933")
            .reference("adjust-123")
            .build()
            .unwrap();

        assert_eq!(request.merchant_account.as_ref(), "TestMerchant");
        assert_eq!(request.modification_amount.minor_units(), 15000);
        assert_eq!(request.modification_amount.currency(), Currency::GBP);
        assert_eq!(request.original_reference.as_ref(), "8515131751004933");
        assert_eq!(request.reference.as_deref(), Some("adjust-123"));
    }

    #[test]
    fn test_modification_result_serialization() {
        let mut additional_data = HashMap::new();
        additional_data.insert("authCode".into(), "123456".into());

        let result = ModificationResult {
            additional_data: Some(additional_data),
            psp_reference: "8515131751004934".into(),
            response: ModificationResponse::CaptureReceived,
        };

        let json = serde_json::to_string(&result).unwrap();
        let deserialized: ModificationResult = serde_json::from_str(&json).unwrap();
        assert_eq!(result, deserialized);
    }

    #[test]
    fn test_modification_response_serialization() {
        assert_eq!(
            serde_json::to_string(&ModificationResponse::CaptureReceived).unwrap(),
            "\"[capture-received]\""
        );
        assert_eq!(
            serde_json::to_string(&ModificationResponse::CancelReceived).unwrap(),
            "\"[cancel-received]\""
        );
        assert_eq!(
            serde_json::to_string(&ModificationResponse::RefundReceived).unwrap(),
            "\"[refund-received]\""
        );
        assert_eq!(
            serde_json::to_string(&ModificationResponse::CancelOrRefundReceived).unwrap(),
            "\"[cancelOrRefund-received]\""
        );
    }

    #[test]
    fn test_builder_validation() {
        // Test missing merchant account
        let result = CaptureRequest::builder()
            .modification_amount(Amount::from_major_units(100, Currency::EUR))
            .original_reference("8515131751004933")
            .build();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("merchant_account is required"));

        // Test missing amount
        let result = CaptureRequest::builder()
            .merchant_account("TestMerchant")
            .original_reference("8515131751004933")
            .build();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("modification_amount is required"));

        // Test missing original reference
        let result = CaptureRequest::builder()
            .merchant_account("TestMerchant")
            .modification_amount(Amount::from_major_units(100, Currency::EUR))
            .build();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("original_reference is required"));
    }
}
