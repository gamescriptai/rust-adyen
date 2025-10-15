//! Integration tests for the Adyen Recurring API v68.

use adyen_core::{Amount, ConfigBuilder, Currency, Environment};
use adyen_recurring::{
    Card, DisableRequest, NotifyShopperRequest, Recurring, RecurringApi, RecurringContract,
    RecurringDetailsRequest, ScheduleAccountUpdaterRequest,
};

fn create_test_config() -> adyen_core::Config {
    ConfigBuilder::new()
        .environment(Environment::test())
        .api_key("test_key_1234567890123456")
        .unwrap()
        .build()
        .unwrap()
}

#[cfg(test)]
mod request_building_tests {
    use super::*;

    #[test]
    fn test_recurring_details_request_builder() {
        let request = RecurringDetailsRequest::builder()
            .merchant_account("TestMerchant")
            .shopper_reference("shopper_12345")
            .build()
            .unwrap();

        assert_eq!(&*request.merchant_account, "TestMerchant");
        assert_eq!(&*request.shopper_reference, "shopper_12345");
        assert!(request.recurring.is_none());
    }

    #[test]
    fn test_recurring_details_request_with_recurring() {
        let recurring = Recurring {
            contract: RecurringContract::Oneclick,
            recurring_detail_name: Some("My Card".into()),
            recurring_expiry: None,
            recurring_frequency: None,
            token_service: None,
        };

        let request = RecurringDetailsRequest::builder()
            .merchant_account("TestMerchant")
            .shopper_reference("shopper_12345")
            .recurring(recurring)
            .build()
            .unwrap();

        assert!(request.recurring.is_some());
        let recurring = request.recurring.unwrap();
        assert!(matches!(recurring.contract, RecurringContract::Oneclick));
        assert_eq!(recurring.recurring_detail_name.as_deref(), Some("My Card"));
    }

    #[test]
    fn test_disable_request_builder() {
        let request = DisableRequest::builder()
            .merchant_account("TestMerchant")
            .shopper_reference("shopper_12345")
            .recurring_detail_reference("8415736344864224")
            .build()
            .unwrap();

        assert_eq!(&*request.merchant_account, "TestMerchant");
        assert_eq!(&*request.shopper_reference, "shopper_12345");
        assert_eq!(
            request.recurring_detail_reference.as_deref(),
            Some("8415736344864224")
        );
    }

    #[test]
    fn test_notify_shopper_request_creation() {
        let amount = Amount::from_minor_units(1000, Currency::EUR);

        let request = NotifyShopperRequest {
            amount,
            merchant_account: "TestMerchant".into(),
            reference: "notify_001".into(),
            shopper_reference: "shopper_12345".into(),
            billing_date: Some("2025-01-15".into()),
            stored_payment_method_id: Some("8415736344864224".into()),
        };

        assert_eq!(request.amount.minor_units(), 1000);
        assert_eq!(request.amount.currency(), Currency::EUR);
        assert_eq!(&*request.merchant_account, "TestMerchant");
        assert_eq!(&*request.reference, "notify_001");
    }

    #[test]
    fn test_schedule_account_updater_request() {
        let card = Card {
            number: Some("1111".into()),
            expiry_month: Some("12".into()),
            expiry_year: Some("2030".into()),
            holder_name: Some("John Doe".into()),
            summary: Some("1111".into()),
        };

        let request = ScheduleAccountUpdaterRequest {
            merchant_account: "TestMerchant".into(),
            reference: "updater_001".into(),
            shopper_reference: "shopper_12345".into(),
            card: Some(card),
        };

        assert_eq!(&*request.merchant_account, "TestMerchant");
        assert_eq!(&*request.reference, "updater_001");
        assert!(request.card.is_some());
    }

    #[test]
    fn test_recurring_contract_serialization() {
        let oneclick = RecurringContract::Oneclick;
        let recurring = RecurringContract::Recurring;
        let both = RecurringContract::OneclickRecurring;

        let oneclick_json = serde_json::to_string(&oneclick).unwrap();
        let recurring_json = serde_json::to_string(&recurring).unwrap();
        let both_json = serde_json::to_string(&both).unwrap();

        assert_eq!(oneclick_json, "\"ONECLICK\"");
        assert_eq!(recurring_json, "\"RECURRING\"");
        assert_eq!(both_json, "\"ONECLICK_RECURRING\"");
    }
}

#[cfg(test)]
mod api_tests {
    use super::*;

    #[test]
    fn test_recurring_api_creation() {
        let config = create_test_config();
        let _api = RecurringApi::new(config).unwrap();

        // Just verify the API was created successfully
        // In a real test environment, you would test actual API calls
    }

    #[tokio::test]
    async fn test_recurring_api_url_generation() {
        let config = create_test_config();
        let _api = RecurringApi::new(config).unwrap();

        // The actual API calls would be tested against a mock server
        // or Adyen's test environment with valid credentials
        // In a real implementation, we would verify URLs and authentication
    }
}

#[cfg(test)]
mod serialization_tests {
    use super::*;

    #[test]
    fn test_recurring_details_request_serialization() {
        let request = RecurringDetailsRequest::builder()
            .merchant_account("TestMerchant")
            .shopper_reference("shopper_12345")
            .build()
            .unwrap();

        let json = serde_json::to_string(&request).unwrap();
        let deserialized: RecurringDetailsRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(request.merchant_account, deserialized.merchant_account);
        assert_eq!(request.shopper_reference, deserialized.shopper_reference);
    }

    #[test]
    fn test_disable_request_serialization() {
        let request = DisableRequest::builder()
            .merchant_account("TestMerchant")
            .shopper_reference("shopper_12345")
            .recurring_detail_reference("8415736344864224")
            .build()
            .unwrap();

        let json = serde_json::to_string(&request).unwrap();
        let deserialized: DisableRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(request.merchant_account, deserialized.merchant_account);
        assert_eq!(request.shopper_reference, deserialized.shopper_reference);
        assert_eq!(
            request.recurring_detail_reference,
            deserialized.recurring_detail_reference
        );
    }

    #[test]
    fn test_notify_shopper_request_serialization() {
        let amount = Amount::from_minor_units(1000, Currency::EUR);
        let request = NotifyShopperRequest {
            amount,
            merchant_account: "TestMerchant".into(),
            reference: "notify_001".into(),
            shopper_reference: "shopper_12345".into(),
            billing_date: Some("2025-01-15".into()),
            stored_payment_method_id: Some("8415736344864224".into()),
        };

        let json = serde_json::to_string(&request).unwrap();
        let deserialized: NotifyShopperRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(
            request.amount.minor_units(),
            deserialized.amount.minor_units()
        );
        assert_eq!(request.merchant_account, deserialized.merchant_account);
        assert_eq!(request.reference, deserialized.reference);
    }

    #[test]
    fn test_schedule_account_updater_request_serialization() {
        let request = ScheduleAccountUpdaterRequest {
            merchant_account: "TestMerchant".into(),
            reference: "updater_001".into(),
            shopper_reference: "shopper_12345".into(),
            card: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        let deserialized: ScheduleAccountUpdaterRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(request.merchant_account, deserialized.merchant_account);
        assert_eq!(request.reference, deserialized.reference);
        assert_eq!(request.shopper_reference, deserialized.shopper_reference);
    }
}

#[cfg(test)]
mod validation_tests {
    use super::*;

    #[test]
    fn test_recurring_details_request_validation() {
        // Test missing merchant account
        let result = RecurringDetailsRequest::builder()
            .shopper_reference("shopper_12345")
            .build();
        assert!(result.is_err());

        // Test missing shopper reference
        let result = RecurringDetailsRequest::builder()
            .merchant_account("TestMerchant")
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn test_disable_request_validation() {
        // Test missing merchant account
        let result = DisableRequest::builder()
            .shopper_reference("shopper_12345")
            .build();
        assert!(result.is_err());

        // Test missing shopper reference
        let result = DisableRequest::builder()
            .merchant_account("TestMerchant")
            .build();
        assert!(result.is_err());
    }
}

#[cfg(test)]
mod workflow_tests {
    use super::*;

    /// Test the complete workflow for listing and managing stored payment methods:
    /// 1. List recurring details for a shopper
    /// 2. Disable a specific stored payment method
    /// 3. Schedule account updater for remaining methods
    #[test]
    fn test_stored_payment_management_workflow_types() {
        // List stored payment methods for a shopper
        let list_request = RecurringDetailsRequest::builder()
            .merchant_account("TestMerchant")
            .shopper_reference("shopper_12345")
            .recurring(Recurring {
                contract: RecurringContract::OneclickRecurring,
                recurring_detail_name: None,
                recurring_expiry: None,
                recurring_frequency: None,
                token_service: None,
            })
            .build()
            .unwrap();

        // Disable a specific stored payment method
        let disable_request = DisableRequest::builder()
            .merchant_account("TestMerchant")
            .shopper_reference("shopper_12345")
            .recurring_detail_reference("8415736344864224")
            .build()
            .unwrap();

        // Schedule account updater for automatic card updates
        let updater_request = ScheduleAccountUpdaterRequest {
            merchant_account: "TestMerchant".into(),
            reference: "updater_001".into(),
            shopper_reference: "shopper_12345".into(),
            card: Some(Card {
                number: Some("1111".into()),
                expiry_month: Some("12".into()),
                expiry_year: Some("2030".into()),
                holder_name: Some("John Doe".into()),
                summary: Some("1111".into()),
            }),
        };

        // Verify all requests are properly constructed
        assert_eq!(
            list_request.shopper_reference,
            disable_request.shopper_reference
        );
        assert_eq!(
            disable_request.shopper_reference,
            updater_request.shopper_reference
        );
        assert!(matches!(
            list_request.recurring.unwrap().contract,
            RecurringContract::OneclickRecurring
        ));
    }

    /// Test the shopper notification workflow for upcoming recurring payments:
    /// 1. Create notification request with payment details
    /// 2. Include billing date and stored payment method information
    #[test]
    fn test_shopper_notification_workflow_types() {
        let amount = Amount::from_minor_units(2500, Currency::GBP); // £25.00

        let notification_request = NotifyShopperRequest {
            amount,
            merchant_account: "TestMerchant".into(),
            reference: "recurring_payment_001".into(),
            shopper_reference: "shopper_12345".into(),
            billing_date: Some("2025-01-15".into()),
            stored_payment_method_id: Some("8415736344864224".into()),
        };

        // Verify notification request structure
        assert_eq!(notification_request.amount.minor_units(), 2500);
        assert_eq!(notification_request.amount.currency(), Currency::GBP);
        assert_eq!(&*notification_request.reference, "recurring_payment_001");
        assert_eq!(
            notification_request.stored_payment_method_id.as_deref(),
            Some("8415736344864224")
        );
        assert_eq!(
            notification_request.billing_date.as_deref(),
            Some("2025-01-15")
        );
    }
}
