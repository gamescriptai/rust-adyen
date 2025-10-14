//! Integration tests for Classic Payments API flows.

use adyen_core::{Amount, Currency, Config, ConfigBuilder, Environment};
use adyen_payments::{
    PaymentsApi, ModificationsApi, PaymentRequest, Card,
    CaptureRequest, CancelRequest, RefundRequest, CancelOrRefundRequest,
    PaymentRequest3d,
};
use adyen_payments::types::BrowserInfo;
use adyen_payments::types::fraud::{
    DeviceFingerprint, RiskData, FraudResult, FraudCheckResult,
    RiskLevel, FraudAction,
};
use std::collections::HashMap;

/// Create a test configuration for integration tests.
fn create_test_config() -> Config {
    ConfigBuilder::new()
        .environment(Environment::test())
        .api_key("test_key_1234567890123456")
        .unwrap()
        .build()
        .unwrap()
}

/// Create a test card for payment requests.
fn create_test_card() -> Card {
    Card {
        number: "4111111111111111".to_string(),
        expiry_month: "12".to_string(),
        expiry_year: "2025".to_string(),
        cvc: "123".to_string(),
        holder_name: None,
    }
}

/// Create a basic payment request for testing.
fn create_basic_payment_request() -> PaymentRequest {
    PaymentRequest::builder()
        .amount(Amount::from_major_units(100, Currency::EUR))
        .merchant_account("TestMerchantAccount")
        .reference("test-payment-001")
        .card(create_test_card())
        .return_url("https://your-company.com/checkout/return")
        .build()
        .unwrap()
}

#[cfg(test)]
mod payment_tests {
    use super::*;

    #[test]
    fn test_payment_request_creation_complete() {
        let payment_request = PaymentRequest::builder()
            .amount(Amount::from_major_units(250, Currency::USD))
            .merchant_account("TestMerchantAccount")
            .reference("complete-payment-001")
            .card(create_test_card())
            .return_url("https://your-company.com/checkout/return")
            .country_code("US")
            .shopper_locale("en_US")
            .shopper_email("test@example.com")
            .shopper_reference("SHOPPER_001")
            .additional_data("customField1", "value1")
            .additional_data("customField2", "value2")
            .build()
            .unwrap();

        assert_eq!(payment_request.amount.minor_units(), 25000);
        assert_eq!(payment_request.amount.currency(), Currency::USD);
        assert_eq!(payment_request.merchant_account, "TestMerchantAccount");
        assert_eq!(payment_request.reference, "complete-payment-001");
        assert_eq!(payment_request.country_code.as_deref(), Some("US"));
        assert_eq!(payment_request.shopper_locale.as_deref(), Some("en_US"));
        assert_eq!(payment_request.shopper_email.as_deref(), Some("test@example.com"));
        assert_eq!(payment_request.shopper_reference.as_deref(), Some("SHOPPER_001"));

        let additional_data = payment_request.additional_data.unwrap();
        assert_eq!(additional_data.get("customField1").map(|s| s.as_str()), Some("value1"));
        assert_eq!(additional_data.get("customField2").map(|s| s.as_str()), Some("value2"));
    }

    #[test]
    fn test_card_creation_and_validation() {
        let card = create_test_card();
        assert_eq!(card.number, "4111111111111111");
        assert_eq!(card.expiry_month, "12");
        assert_eq!(card.expiry_year, "2025");
        assert_eq!(card.cvc, "123");
        assert_eq!(card.holder_name, None);

        let card_with_holder = Card {
            number: "4111111111111111".to_string(),
            expiry_month: "12".to_string(),
            expiry_year: "2025".to_string(),
            cvc: "123".to_string(),
            holder_name: Some("John Doe".to_string()),
        };
        assert_eq!(card_with_holder.holder_name.as_deref(), Some("John Doe"));
    }

    #[test]
    fn test_payments_api_creation() {
        let config = create_test_config();
        let api = PaymentsApi::new(config);
        assert!(api.is_ok());
    }

    #[test]
    fn test_modifications_api_creation() {
        let config = create_test_config();
        let api = ModificationsApi::new(config);
        assert!(api.is_ok());
    }
}

#[cfg(test)]
mod three_d_secure_tests {
    use super::*;

    #[test]
    fn test_3ds1_payment_request() {
        let browser_info = BrowserInfo {
            accept_header: "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8".to_string(),
            user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36".to_string(),
            color_depth: 24,
            java_enabled: false,
            java_script_enabled: true,
            language: "en-US".to_string(),
            screen_height: 1080,
            screen_width: 1920,
            time_zone_offset: -480,
        };

        let request = PaymentRequest3d::builder()
            .merchant_account("TestMerchantAccount")
            .browser_info(browser_info)
            .md("md_value_from_issuer")
            .pa_response("pa_response_from_issuer")
            .build()
            .unwrap();

        assert_eq!(request.merchant_account, "TestMerchantAccount");
        assert_eq!(request.md, "md_value_from_issuer");
        assert_eq!(request.pa_response, "pa_response_from_issuer");
        assert!(request.browser_info.is_some());
    }
}

#[cfg(test)]
mod modification_tests {
    use super::*;

    #[test]
    fn test_capture_request_complete() {
        let mut additional_data = HashMap::new();
        additional_data.insert("invoiceNumber".into(), "INV-2023-001".into());

        let request = CaptureRequest::builder()
            .merchant_account("TestMerchantAccount")
            .modification_amount(Amount::from_major_units(75, Currency::EUR))
            .original_reference("8515131751004933")
            .reference("capture-001")
            .additional_data(additional_data)
            .build()
            .unwrap();

        assert_eq!(request.merchant_account.as_ref(), "TestMerchantAccount");
        assert_eq!(request.modification_amount.minor_units(), 7500);
        assert_eq!(request.modification_amount.currency(), Currency::EUR);
        assert_eq!(request.original_reference.as_ref(), "8515131751004933");
        assert_eq!(request.reference.as_deref(), Some("capture-001"));

        let additional_data = request.additional_data.unwrap();
        assert_eq!(additional_data.get("invoiceNumber").map(|s| s.as_ref()), Some("INV-2023-001"));
    }

    #[test]
    fn test_cancel_request_complete() {
        let mut additional_data = HashMap::new();
        additional_data.insert("cancelReason".into(), "customer_request".into());

        let request = CancelRequest::builder()
            .merchant_account("TestMerchantAccount")
            .original_reference("8515131751004933")
            .reference("cancel-001")
            .additional_data(additional_data)
            .build()
            .unwrap();

        assert_eq!(request.merchant_account.as_ref(), "TestMerchantAccount");
        assert_eq!(request.original_reference.as_ref(), "8515131751004933");
        assert_eq!(request.reference.as_deref(), Some("cancel-001"));

        let additional_data = request.additional_data.unwrap();
        assert_eq!(additional_data.get("cancelReason").map(|s| s.as_ref()), Some("customer_request"));
    }

    #[test]
    fn test_refund_request_complete() {
        let mut additional_data = HashMap::new();
        additional_data.insert("refundReason".into(), "defective_product".into());
        additional_data.insert("customerNotificationSent".into(), "true".into());

        let request = RefundRequest::builder()
            .merchant_account("TestMerchantAccount")
            .modification_amount(Amount::from_major_units(50, Currency::USD))
            .original_reference("8515131751004933")
            .reference("refund-001")
            .additional_data(additional_data)
            .build()
            .unwrap();

        assert_eq!(request.merchant_account.as_ref(), "TestMerchantAccount");
        assert_eq!(request.modification_amount.minor_units(), 5000);
        assert_eq!(request.modification_amount.currency(), Currency::USD);
        assert_eq!(request.original_reference.as_ref(), "8515131751004933");
        assert_eq!(request.reference.as_deref(), Some("refund-001"));

        let additional_data = request.additional_data.unwrap();
        assert_eq!(additional_data.get("refundReason").map(|s| s.as_ref()), Some("defective_product"));
        assert_eq!(additional_data.get("customerNotificationSent").map(|s| s.as_ref()), Some("true"));
    }

    #[test]
    fn test_cancel_or_refund_request() {
        let request = CancelOrRefundRequest::builder()
            .merchant_account("TestMerchantAccount")
            .original_reference("8515131751004933")
            .reference("cancel-or-refund-001")
            .build()
            .unwrap();

        assert_eq!(request.merchant_account.as_ref(), "TestMerchantAccount");
        assert_eq!(request.original_reference.as_ref(), "8515131751004933");
        assert_eq!(request.reference.as_deref(), Some("cancel-or-refund-001"));
    }

    #[test]
    fn test_modification_request_validation() {
        // Test missing merchant account
        let result = CaptureRequest::builder()
            .modification_amount(Amount::from_major_units(100, Currency::EUR))
            .original_reference("8515131751004933")
            .build();
        assert!(result.is_err());

        // Test missing amount
        let result = CaptureRequest::builder()
            .merchant_account("TestMerchantAccount")
            .original_reference("8515131751004933")
            .build();
        assert!(result.is_err());

        // Test missing original reference
        let result = CaptureRequest::builder()
            .merchant_account("TestMerchantAccount")
            .modification_amount(Amount::from_major_units(100, Currency::EUR))
            .build();
        assert!(result.is_err());
    }
}

#[cfg(test)]
mod fraud_detection_tests {
    use super::*;

    #[test]
    fn test_fraud_result_comprehensive() {
        let fraud_result = FraudResult {
            account_score: Some(75),
            check_id: Some(12345),
            name: Some("AdyenFraudDetection".into()),
            risk_level: Some(RiskLevel::High),
            action: Some(FraudAction::Challenge),
            fraud_score: Some(85),
            results: None,
            reason_codes: Some(vec![
                "HIGH_VELOCITY".into(),
                "NEW_CARD".into(),
                "UNUSUAL_SPENDING_PATTERN".into(),
            ]),
        };

        assert_eq!(fraud_result.account_score, Some(75));
        assert_eq!(fraud_result.check_id, Some(12345));
        assert_eq!(fraud_result.name.as_deref(), Some("AdyenFraudDetection"));
        assert_eq!(fraud_result.risk_level, Some(RiskLevel::High));
        assert_eq!(fraud_result.action, Some(FraudAction::Challenge));
        assert_eq!(fraud_result.fraud_score, Some(85));

        let reason_codes = fraud_result.reason_codes.unwrap();
        assert_eq!(reason_codes.len(), 3);
        assert!(reason_codes.contains(&"HIGH_VELOCITY".into()));
        assert!(reason_codes.contains(&"NEW_CARD".into()));
        assert!(reason_codes.contains(&"UNUSUAL_SPENDING_PATTERN".into()));
    }

    #[test]
    fn test_fraud_check_result_with_metadata() {
        let mut metadata = HashMap::new();
        metadata.insert("provider_version".into(), serde_json::Value::String("2.1".to_string()));
        metadata.insert("check_duration_ms".into(), serde_json::Value::Number(serde_json::Number::from(150)));
        metadata.insert("confidence_score".into(), serde_json::Value::Number(serde_json::Number::from_f64(0.95).unwrap()));

        let fraud_check = FraudCheckResult {
            name: "CyberSourceFraud".into(),
            check_id: 67890,
            account_score: 60,
            risk_level: RiskLevel::Medium,
            action: FraudAction::Review,
            metadata: Some(metadata),
        };

        assert_eq!(fraud_check.name.as_ref(), "CyberSourceFraud");
        assert_eq!(fraud_check.check_id, 67890);
        assert_eq!(fraud_check.account_score, 60);
        assert_eq!(fraud_check.risk_level, RiskLevel::Medium);
        assert_eq!(fraud_check.action, FraudAction::Review);

        let metadata = fraud_check.metadata.unwrap();
        assert_eq!(metadata.get("provider_version"), Some(&serde_json::Value::String("2.1".to_string())));
        assert!(metadata.contains_key("check_duration_ms"));
        assert!(metadata.contains_key("confidence_score"));
    }

    #[test]
    fn test_device_fingerprint_comprehensive() {
        let fingerprint = DeviceFingerprint::builder()
            .device_id("device_12345")
            .device_type("mobile")
            .operating_system("iOS 15.0")
            .browser("Safari 15.0")
            .screen_resolution("1920x1080")
            .timezone_offset(-480) // PST
            .language("en-US")
            .color_depth(24)
            .java_enabled(false)
            .cookies_enabled(true)
            .build();

        assert_eq!(fingerprint.device_id.as_deref(), Some("device_12345"));
        assert_eq!(fingerprint.device_type.as_deref(), Some("mobile"));
        assert_eq!(fingerprint.operating_system.as_deref(), Some("iOS 15.0"));
        assert_eq!(fingerprint.browser.as_deref(), Some("Safari 15.0"));
        assert_eq!(fingerprint.screen_resolution.as_deref(), Some("1920x1080"));
        assert_eq!(fingerprint.timezone_offset, Some(-480));
        assert_eq!(fingerprint.language.as_deref(), Some("en-US"));
        assert_eq!(fingerprint.color_depth, Some(24));
        assert_eq!(fingerprint.java_enabled, Some(false));
        assert_eq!(fingerprint.cookies_enabled, Some(true));
    }

    #[test]
    fn test_risk_data_comprehensive() {
        let mut custom_fields = HashMap::new();
        custom_fields.insert("merchant_category".into(), "retail".into());
        custom_fields.insert("customer_tier".into(), "premium".into());
        custom_fields.insert("transaction_context".into(), "recurring".into());

        let risk_data = RiskData::builder()
            .client_data("encrypted_client_data_base64")
            .custom_fields(custom_fields)
            .fraud_offset(100)
            .profile_reference("risk_profile_001")
            .skip_fraud(false)
            .build();

        assert_eq!(risk_data.client_data.as_deref(), Some("encrypted_client_data_base64"));
        assert_eq!(risk_data.fraud_offset, Some(100));
        assert_eq!(risk_data.profile_reference.as_deref(), Some("risk_profile_001"));
        assert_eq!(risk_data.skip_fraud, Some(false));

        let custom_fields = risk_data.custom_fields.unwrap();
        assert_eq!(custom_fields.get("merchant_category").map(|s| s.as_ref()), Some("retail"));
        assert_eq!(custom_fields.get("customer_tier").map(|s| s.as_ref()), Some("premium"));
        assert_eq!(custom_fields.get("transaction_context").map(|s| s.as_ref()), Some("recurring"));
    }
}

#[cfg(test)]
mod end_to_end_flow_tests {
    use super::*;

    /// Simulate a complete payment flow: authorization -> capture
    #[test]
    fn test_payment_authorization_capture_flow() {
        // Step 1: Create payment authorization
        let payment_request = create_basic_payment_request();
        assert_eq!(payment_request.amount.minor_units(), 10000);
        assert_eq!(payment_request.merchant_account, "TestMerchantAccount");

        // Step 2: Simulate successful authorization (would return PSP reference)
        let psp_reference = "8515131751004933";

        // Step 3: Create capture request for part of the authorized amount
        let capture_request = CaptureRequest::builder()
            .merchant_account("TestMerchantAccount")
            .modification_amount(Amount::from_major_units(75, Currency::EUR))
            .original_reference(psp_reference)
            .reference("capture-partial-001")
            .build()
            .unwrap();

        assert_eq!(capture_request.modification_amount.minor_units(), 7500);
        assert_eq!(capture_request.original_reference.as_ref(), psp_reference);
    }

    /// Simulate a complete payment flow: authorization -> cancel
    #[test]
    fn test_payment_authorization_cancel_flow() {
        // Step 1: Create payment authorization
        let _payment_request = create_basic_payment_request();

        // Step 2: Simulate successful authorization
        let psp_reference = "8515131751004933";

        // Step 3: Create cancel request
        let cancel_request = CancelRequest::builder()
            .merchant_account("TestMerchantAccount")
            .original_reference(psp_reference)
            .reference("cancel-customer-request-001")
            .build()
            .unwrap();

        assert_eq!(cancel_request.original_reference.as_ref(), psp_reference);
        assert_eq!(cancel_request.reference.as_deref(), Some("cancel-customer-request-001"));
    }

    /// Simulate a 3D Secure flow: authorization -> 3D Secure challenge -> completion
    #[test]
    fn test_3d_secure_flow() {
        // Step 1: Initial payment request
        let _initial_payment = create_basic_payment_request();

        // Step 2: Simulate 3D Secure challenge required
        // (In real flow, this would be returned by the initial authorization)

        // Step 3: Create 3D Secure authentication request
        let browser_info = BrowserInfo {
            accept_header: "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8".to_string(),
            user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36".to_string(),
            color_depth: 24,
            java_enabled: false,
            java_script_enabled: true,
            language: "en-US".to_string(),
            screen_height: 1080,
            screen_width: 1920,
            time_zone_offset: -480,
        };

        let three_ds_request = PaymentRequest3d::builder()
            .merchant_account("TestMerchantAccount")
            .browser_info(browser_info)
            .md("md_from_initial_auth")
            .pa_response("pa_response_from_issuer")
            .build()
            .unwrap();

        assert_eq!(three_ds_request.merchant_account, "TestMerchantAccount");
        assert_eq!(three_ds_request.md, "md_from_initial_auth");
    }

    /// Simulate refund after capture flow
    #[test]
    fn test_capture_refund_flow() {
        // Step 1: Assume we have a captured payment
        let captured_psp_reference = "8515131751004934";

        // Step 2: Create partial refund request
        let refund_request = RefundRequest::builder()
            .merchant_account("TestMerchantAccount")
            .modification_amount(Amount::from_major_units(25, Currency::EUR))
            .original_reference(captured_psp_reference)
            .reference("refund-partial-001")
            .build()
            .unwrap();

        assert_eq!(refund_request.modification_amount.minor_units(), 2500);
        assert_eq!(refund_request.original_reference.as_ref(), captured_psp_reference);
    }
}