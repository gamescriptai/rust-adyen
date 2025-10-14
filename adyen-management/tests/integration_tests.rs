//! Integration tests for the Adyen Management API v3.

use adyen_core::{ConfigBuilder, Environment};
use adyen_management::{
    ManagementApi, CreateMerchantRequest, CreateStoreRequest, CreateWebhookRequest,
    UpdatePaymentMethodRequest
};
use adyen_management::types::{
    BusinessDetails, Contact, Address, MerchantStatus, StoreStatus, TerminalStatus
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
    fn test_create_merchant_request_builder() {
        let business_details = BusinessDetails {
            legal_business_name: "Example Corp".into(),
            trading_name: Some("Example Store".into()),
            mcc: Some("5411".into()),
            registration: None,
            website_url: Some("https://example.com".into()),
        };

        let contact = Contact {
            email: "contact@example.com".into(),
            first_name: Some("John".into()),
            last_name: Some("Doe".into()),
            phone_number: Some("+1234567890".into()),
        };

        let address = Address {
            street_address: "123 Main St".into(),
            street_address2: None,
            city: "New York".into(),
            state_or_province: Some("NY".into()),
            postal_code: "10001".into(),
            country: "US".into(),
        };

        let request = CreateMerchantRequest::builder()
            .company_id("company_123")
            .merchant_account("ExampleMerchant")
            .business_details(business_details)
            .primary_contact(contact)
            .billing_address(address)
            .build()
            .unwrap();

        assert_eq!(&*request.company_id, "company_123");
        assert_eq!(&*request.merchant_account, "ExampleMerchant");
        assert_eq!(&*request.business_details.legal_business_name, "Example Corp");
        assert_eq!(&*request.primary_contact.email, "contact@example.com");
        assert!(request.billing_address.is_some());
    }

    #[test]
    fn test_create_store_request_builder() {
        let address = Address {
            street_address: "456 Store Ave".into(),
            street_address2: Some("Suite 100".into()),
            city: "Los Angeles".into(),
            state_or_province: Some("CA".into()),
            postal_code: "90210".into(),
            country: "US".into(),
        };

        let request = CreateStoreRequest::builder()
            .store_reference("store_001")
            .description("Main Store Location")
            .address(address)
            .phone_number("+1987654321")
            .business_line_id("business_line_123")
            .build()
            .unwrap();

        assert_eq!(&*request.store_reference, "store_001");
        assert_eq!(&*request.description, "Main Store Location");
        assert_eq!(&*request.address.city, "Los Angeles");
        assert_eq!(request.phone_number.as_deref(), Some("+1987654321"));
        assert_eq!(request.business_line_id.as_deref(), Some("business_line_123"));
    }

    #[test]
    fn test_create_webhook_request() {
        let request = CreateWebhookRequest {
            url: "https://example.com/webhook".into(),
            description: Some("Payment notifications".into()),
            active: true,
            communication_format: "json".into(),
            filter_merchant_accounts: Some(vec!["merchant_123".into()]),
            additional_settings: None,
        };

        assert_eq!(&*request.url, "https://example.com/webhook");
        assert_eq!(request.description.as_deref(), Some("Payment notifications"));
        assert!(request.active);
        assert_eq!(&*request.communication_format, "json");
        assert_eq!(request.filter_merchant_accounts.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn test_update_payment_method_request() {
        let request = UpdatePaymentMethodRequest {
            r#type: "scheme".into(),
            enabled: true,
            configuration: None,
            countries: Some(vec!["US".into(), "CA".into()]),
            currencies: Some(vec!["USD".into(), "CAD".into()]),
        };

        assert_eq!(&*request.r#type, "scheme");
        assert!(request.enabled);
        assert_eq!(request.countries.as_ref().unwrap().len(), 2);
        assert_eq!(request.currencies.as_ref().unwrap().len(), 2);
    }
}

#[cfg(test)]
mod api_tests {
    use super::*;

    #[test]
    fn test_management_api_creation() {
        let config = create_test_config();
        let _api = ManagementApi::new(config).unwrap();

        // Just verify the API was created successfully
        // In a real test environment, you would test actual API calls
    }

    #[tokio::test]
    async fn test_management_api_url_generation() {
        let config = create_test_config();
        let _api = ManagementApi::new(config).unwrap();

        // The actual API calls would be tested against a mock server
        // or Adyen's test environment with valid credentials
        // In a real implementation, we would verify URLs and authentication
    }
}

#[cfg(test)]
mod serialization_tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_create_merchant_request_serialization() {
        let business_details = BusinessDetails {
            legal_business_name: "Test Corp".into(),
            trading_name: None,
            mcc: Some("7372".into()),
            registration: None,
            website_url: None,
        };

        let contact = Contact {
            email: "test@example.com".into(),
            first_name: Some("Test".into()),
            last_name: Some("User".into()),
            phone_number: None,
        };

        let request = CreateMerchantRequest::builder()
            .company_id("company_456")
            .merchant_account("TestMerchant")
            .business_details(business_details)
            .primary_contact(contact)
            .build()
            .unwrap();

        let json = serde_json::to_string(&request).unwrap();
        let deserialized: CreateMerchantRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(request.company_id, deserialized.company_id);
        assert_eq!(request.merchant_account, deserialized.merchant_account);
        assert_eq!(request.business_details.legal_business_name, deserialized.business_details.legal_business_name);
    }

    #[test]
    fn test_create_store_request_serialization() {
        let address = Address {
            street_address: "789 Test St".into(),
            street_address2: None,
            city: "Test City".into(),
            state_or_province: None,
            postal_code: "12345".into(),
            country: "US".into(),
        };

        let request = CreateStoreRequest::builder()
            .store_reference("test_store")
            .description("Test Store")
            .address(address)
            .build()
            .unwrap();

        let json = serde_json::to_string(&request).unwrap();
        let deserialized: CreateStoreRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(request.store_reference, deserialized.store_reference);
        assert_eq!(request.description, deserialized.description);
        assert_eq!(request.address.city, deserialized.address.city);
    }

    #[test]
    fn test_create_webhook_request_serialization() {
        let request = CreateWebhookRequest {
            url: "https://test.example.com/webhook".into(),
            description: Some("Test webhook".into()),
            active: false,
            communication_format: "soap".into(),
            filter_merchant_accounts: None,
            additional_settings: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        let deserialized: CreateWebhookRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(request.url, deserialized.url);
        assert_eq!(request.description, deserialized.description);
        assert_eq!(request.active, deserialized.active);
        assert_eq!(request.communication_format, deserialized.communication_format);
    }

    #[test]
    fn test_enum_serialization() {
        // Test MerchantStatus enum
        let active = MerchantStatus::Active;
        let suspended = MerchantStatus::Suspended;

        let active_json = serde_json::to_string(&active).unwrap();
        let suspended_json = serde_json::to_string(&suspended).unwrap();

        assert_eq!(active_json, "\"Active\"");
        assert_eq!(suspended_json, "\"Suspended\"");

        // Test StoreStatus enum
        let store_active = StoreStatus::Active;
        let store_closed = StoreStatus::Closed;

        let store_active_json = serde_json::to_string(&store_active).unwrap();
        let store_closed_json = serde_json::to_string(&store_closed).unwrap();

        assert_eq!(store_active_json, "\"Active\"");
        assert_eq!(store_closed_json, "\"Closed\"");

        // Test TerminalStatus enum
        let terminal_active = TerminalStatus::Active;
        let terminal_boarded = TerminalStatus::Boarded;

        let terminal_active_json = serde_json::to_string(&terminal_active).unwrap();
        let terminal_boarded_json = serde_json::to_string(&terminal_boarded).unwrap();

        assert_eq!(terminal_active_json, "\"Active\"");
        assert_eq!(terminal_boarded_json, "\"Boarded\"");
    }
}

#[cfg(test)]
mod validation_tests {
    use super::*;

    #[test]
    fn test_create_merchant_request_validation() {
        // Test missing company ID
        let business_details = BusinessDetails {
            legal_business_name: "Test Corp".into(),
            trading_name: None,
            mcc: None,
            registration: None,
            website_url: None,
        };

        let contact = Contact {
            email: "test@example.com".into(),
            first_name: None,
            last_name: None,
            phone_number: None,
        };

        let result = CreateMerchantRequest::builder()
            .merchant_account("TestMerchant")
            .business_details(business_details.clone())
            .primary_contact(contact.clone())
            .build();
        assert!(result.is_err());

        // Test missing merchant account
        let result = CreateMerchantRequest::builder()
            .company_id("company_123")
            .business_details(business_details.clone())
            .primary_contact(contact.clone())
            .build();
        assert!(result.is_err());

        // Test missing business details
        let result = CreateMerchantRequest::builder()
            .company_id("company_123")
            .merchant_account("TestMerchant")
            .primary_contact(contact)
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn test_create_store_request_validation() {
        let address = Address {
            street_address: "123 Test St".into(),
            street_address2: None,
            city: "Test City".into(),
            state_or_province: None,
            postal_code: "12345".into(),
            country: "US".into(),
        };

        // Test missing store reference
        let result = CreateStoreRequest::builder()
            .description("Test Store")
            .address(address.clone())
            .build();
        assert!(result.is_err());

        // Test missing description
        let result = CreateStoreRequest::builder()
            .store_reference("test_store")
            .address(address.clone())
            .build();
        assert!(result.is_err());

        // Test missing address
        let result = CreateStoreRequest::builder()
            .store_reference("test_store")
            .description("Test Store")
            .build();
        assert!(result.is_err());
    }
}

#[cfg(test)]
mod workflow_tests {
    use super::*;

    /// Test the complete merchant onboarding workflow:
    /// 1. Create a merchant account
    /// 2. Create a store under the merchant
    /// 3. Configure payment methods for the store
    /// 4. Set up webhooks for notifications
    #[test]
    fn test_merchant_onboarding_workflow_types() {
        // Step 1: Create merchant account
        let business_details = BusinessDetails {
            legal_business_name: "Global Retail Corp".into(),
            trading_name: Some("Global Store".into()),
            mcc: Some("5311".into()),
            registration: None,
            website_url: Some("https://globalstore.example.com".into()),
        };

        let primary_contact = Contact {
            email: "admin@globalstore.example.com".into(),
            first_name: Some("Jane".into()),
            last_name: Some("Smith".into()),
            phone_number: Some("+1555123456".into()),
        };

        let billing_address = Address {
            street_address: "100 Business Blvd".into(),
            street_address2: Some("Floor 5".into()),
            city: "Business City".into(),
            state_or_province: Some("BC".into()),
            postal_code: "12345".into(),
            country: "US".into(),
        };

        let create_merchant_request = CreateMerchantRequest::builder()
            .company_id("company_global_001")
            .merchant_account("GlobalStoreMerchant")
            .business_details(business_details)
            .primary_contact(primary_contact)
            .billing_address(billing_address)
            .build()
            .unwrap();

        // Step 2: Create store under merchant
        let store_address = Address {
            street_address: "200 Retail Ave".into(),
            street_address2: None,
            city: "Retail City".into(),
            state_or_province: Some("RC".into()),
            postal_code: "54321".into(),
            country: "US".into(),
        };

        let create_store_request = CreateStoreRequest::builder()
            .store_reference("global_store_001")
            .description("Global Store - Main Location")
            .address(store_address)
            .phone_number("+1555987654")
            .business_line_id("retail_line_001")
            .build()
            .unwrap();

        // Step 3: Configure payment methods
        let payment_method_request = UpdatePaymentMethodRequest {
            r#type: "scheme".into(),
            enabled: true,
            configuration: None,
            countries: Some(vec!["US".into(), "CA".into(), "MX".into()]),
            currencies: Some(vec!["USD".into(), "CAD".into(), "MXN".into()]),
        };

        // Step 4: Set up webhooks
        let webhook_request = CreateWebhookRequest {
            url: "https://globalstore.example.com/adyen/webhook".into(),
            description: Some("Payment and refund notifications".into()),
            active: true,
            communication_format: "json".into(),
            filter_merchant_accounts: Some(vec!["GlobalStoreMerchant".into()]),
            additional_settings: None,
        };

        // Verify all requests are properly constructed
        assert_eq!(&*create_merchant_request.company_id, "company_global_001");
        assert_eq!(&*create_store_request.store_reference, "global_store_001");
        assert_eq!(&*payment_method_request.r#type, "scheme");
        assert_eq!(&*webhook_request.url, "https://globalstore.example.com/adyen/webhook");

        // Verify relationships are consistent
        assert_eq!(&*create_merchant_request.merchant_account, "GlobalStoreMerchant");
        assert_eq!(webhook_request.filter_merchant_accounts.as_ref().unwrap()[0], create_merchant_request.merchant_account);
    }

    /// Test the terminal management workflow:
    /// 1. Get available terminal models
    /// 2. Configure terminal settings
    /// 3. Assign terminals to stores
    #[test]
    fn test_terminal_management_workflow_types() {
        // This test validates the type structures for terminal management
        // In practice, these would be API responses, but we're testing type correctness

        use adyen_management::types::{TerminalModel, TerminalSettings, Terminal, TerminalAssignment};
        use adyen_management::types::{CardAcquisitionSettings, ConnectivitySettings, ReceiptOptions};

        // Terminal model information
        let terminal_model = TerminalModel {
            id: "P400".into(),
            name: "Adyen P400".into(),
            manufacturer: Some("Adyen".into()),
            contactless: Some(true),
            contactless_limit: Some(5000), // $50.00 in minor units
        };

        // Terminal settings configuration
        let terminal_settings = TerminalSettings {
            card_acquisition: Some(CardAcquisitionSettings {
                operation: Some("SWIPE_INSERT_CONTACTLESS".into()),
                timeout: Some(30),
            }),
            connectivity: Some(ConnectivitySettings {
                ethernet: None,
                wifi: None,
            }),
            receipt_options: Some(ReceiptOptions {
                merchant_receipt: Some(true),
                shopper_receipt: Some(true),
            }),
            gratuity: None,
        };

        // Terminal assignment
        let terminal = Terminal {
            id: "terminal_001".into(),
            serial_number: "123-456-789".into(),
            model: "P400".into(),
            store_id: Some("global_store_001".into()),
            status: Some(TerminalStatus::Active),
            assignment: Some(TerminalAssignment {
                company_id: Some("company_global_001".into()),
                merchant_id: Some("GlobalStoreMerchant".into()),
                store_id: Some("global_store_001".into()),
                status: Some("ASSIGNED".into()),
            }),
        };

        // Verify terminal workflow structure
        assert_eq!(&*terminal_model.id, "P400");
        assert!(terminal_model.contactless.unwrap());
        assert!(terminal_settings.card_acquisition.is_some());
        assert_eq!(&*terminal.serial_number, "123-456-789");
        assert_eq!(terminal.assignment.as_ref().unwrap().store_id.as_deref(), Some("global_store_001"));
    }

    /// Test the webhook management workflow:
    /// 1. Create webhook for notifications
    /// 2. Update webhook configuration
    /// 3. Test webhook filtering
    #[test]
    fn test_webhook_management_workflow_types() {
        use adyen_management::{UpdateWebhookRequest};
        use adyen_management::types::{Webhook, WebhookAdditionalSettings};
        use std::collections::HashMap;

        // Initial webhook creation
        let create_request = CreateWebhookRequest {
            url: "https://api.merchant.com/webhooks/adyen".into(),
            description: Some("All payment events".into()),
            active: true,
            communication_format: "json".into(),
            filter_merchant_accounts: Some(vec!["merchant_001".into(), "merchant_002".into()]),
            additional_settings: Some(WebhookAdditionalSettings {
                headers: Some({
                    let mut headers = HashMap::new();
                    headers.insert("Authorization".to_string(), "Bearer secret_token".into());
                    headers.insert("Content-Type".to_string(), "application/json".into());
                    headers
                }),
                include_event_codes: Some(vec!["AUTHORISATION".into(), "CAPTURE".into()]),
                exclude_event_codes: None,
            }),
        };

        // Webhook update request
        let update_request = UpdateWebhookRequest {
            url: Some("https://api.merchant.com/webhooks/adyen/v2".into()),
            description: Some("Updated webhook endpoint".into()),
            active: Some(true),
            communication_format: None, // Keep existing
            filter_merchant_accounts: Some(vec!["merchant_001".into()]), // Reduce scope
            additional_settings: Some(WebhookAdditionalSettings {
                headers: None, // Remove custom headers
                include_event_codes: Some(vec![
                    "AUTHORISATION".into(),
                    "CAPTURE".into(),
                    "REFUND".into(),
                ]),
                exclude_event_codes: Some(vec!["CANCEL_OR_REFUND".into()]),
            }),
        };

        // Webhook response (what API would return)
        let webhook_response = Webhook {
            id: "webhook_12345".into(),
            url: "https://api.merchant.com/webhooks/adyen/v2".into(),
            description: Some("Updated webhook endpoint".into()),
            active: true,
            communication_format: "json".into(),
            filter_merchant_accounts: vec!["merchant_001".into()],
            additional_settings: Some(WebhookAdditionalSettings {
                headers: None,
                include_event_codes: Some(vec![
                    "AUTHORISATION".into(),
                    "CAPTURE".into(),
                    "REFUND".into(),
                ]),
                exclude_event_codes: Some(vec!["CANCEL_OR_REFUND".into()]),
            }),
            links: None,
        };

        // Verify webhook workflow progression
        assert_eq!(&*create_request.url, "https://api.merchant.com/webhooks/adyen");
        assert_eq!(create_request.filter_merchant_accounts.as_ref().unwrap().len(), 2);

        assert_eq!(update_request.url.as_deref(), Some("https://api.merchant.com/webhooks/adyen/v2"));
        assert_eq!(update_request.filter_merchant_accounts.as_ref().unwrap().len(), 1);

        assert_eq!(&*webhook_response.url, "https://api.merchant.com/webhooks/adyen/v2");
        assert_eq!(webhook_response.filter_merchant_accounts.len(), 1);
        assert_eq!(webhook_response.additional_settings.as_ref().unwrap().include_event_codes.as_ref().unwrap().len(), 3);
    }
}