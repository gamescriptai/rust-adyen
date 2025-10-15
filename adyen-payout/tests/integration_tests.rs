//! Integration tests for the Adyen Payout API.
//!
//! These tests verify the complete payout workflows including instant payouts,
//! batch processing, review workflows, and error handling scenarios.

use adyen_core::{Amount, ConfigBuilder, Currency, Environment};
use adyen_payout::{
    Address, BankAccount, BankAccountType, Card, ConfirmRequest, DeclinePayoutRequest, EntityType,
    Name, PayoutApi, PayoutMethodDetails, ReviewPayoutRequest, SubmitRequest,
};

/// Helper function to create a test configuration.
fn create_test_config() -> adyen_core::Config {
    ConfigBuilder::new()
        .environment(Environment::test())
        .api_key("test_key_1234567890123456")
        .unwrap()
        .build()
        .unwrap()
}

/// Helper function to create a test bank account.
fn create_test_bank_account() -> BankAccount {
    BankAccount {
        account_number: "1234567890".into(),
        bic: Some("ABNANL2A".into()),
        country_code: "NL".into(),
        owner_name: "John Doe".into(),
        iban: Some("NL91ABNA0417164300".into()),
        bank_account_type: Some(BankAccountType::Checking),
    }
}

/// Helper function to create a test card.
fn create_test_card() -> Card {
    Card {
        number: "4111111111111111".into(),
        expiry_month: "12".into(),
        expiry_year: "2030".into(),
        holder_name: "John Doe".into(),
    }
}

/// Helper function to create a test address.
fn create_test_address() -> Address {
    Address {
        city: "Amsterdam".into(),
        country: "NL".into(),
        house_number_or_name: "123".into(),
        postal_code: "1012AB".into(),
        state_or_province: None,
        street: "Main Street".into(),
    }
}

/// Helper function to create a test name.
fn create_test_name() -> Name {
    Name {
        first_name: "John".into(),
        last_name: "Doe".into(),
    }
}

#[cfg(test)]
mod payout_api_tests {
    use super::*;

    #[test]
    fn test_payout_api_creation() {
        let config = create_test_config();
        let _api = PayoutApi::new(config).unwrap();
        // We can't directly access the client field, but creating the API successfully indicates it's working
    }

    #[test]
    fn test_submit_request_with_bank_account() {
        let bank_account = create_test_bank_account();
        let amount = Amount::from_minor_units(10000, Currency::EUR); // €100.00

        let request = SubmitRequest::builder()
            .amount(amount)
            .merchant_account("TestMerchant")
            .reference("payout-test-001")
            .shopper_email("test@example.com")
            .shopper_reference("shopper-001")
            .payout_method_details(PayoutMethodDetails::BankAccount(bank_account))
            .billing_address(create_test_address())
            .shopper_name(create_test_name())
            .entity_type(EntityType::NaturalPerson)
            .nationality("NL")
            .date_of_birth("1980-01-01")
            .build()
            .unwrap();

        assert_eq!(request.amount.minor_units(), 10000);
        assert_eq!(request.amount.currency(), Currency::EUR);
        assert_eq!(&*request.merchant_account, "TestMerchant");
        assert_eq!(&*request.reference, "payout-test-001");
        assert_eq!(&*request.shopper_email, "test@example.com");
        assert!(request.billing_address.is_some());
        assert!(request.shopper_name.is_some());
        assert!(matches!(
            request.entity_type,
            Some(EntityType::NaturalPerson)
        ));
    }

    #[test]
    fn test_submit_request_with_card() {
        let card = create_test_card();
        let amount = Amount::from_minor_units(5000, Currency::USD); // $50.00

        let request = SubmitRequest::builder()
            .amount(amount)
            .merchant_account("TestMerchant")
            .reference("payout-card-001")
            .shopper_email("card@example.com")
            .shopper_reference("shopper-card-001")
            .payout_method_details(PayoutMethodDetails::Card(card))
            .build()
            .unwrap();

        assert_eq!(request.amount.minor_units(), 5000);
        assert_eq!(request.amount.currency(), Currency::USD);
        assert_eq!(&*request.reference, "payout-card-001");
        assert!(matches!(
            request.payout_method_details,
            PayoutMethodDetails::Card(_)
        ));
    }

    #[test]
    fn test_instant_payout_request_with_card() {
        let card = create_test_card();
        let amount = Amount::from_minor_units(2500, Currency::EUR); // €25.00

        let request = SubmitRequest::builder()
            .amount(amount)
            .merchant_account("TestMerchant")
            .reference("instant-payout-001")
            .shopper_email("instant@example.com")
            .shopper_reference("instant-shopper-001")
            .payout_method_details(PayoutMethodDetails::Card(card))
            .build()
            .unwrap();

        assert_eq!(request.amount.minor_units(), 2500);
        assert_eq!(request.amount.currency(), Currency::EUR);
        assert_eq!(&*request.reference, "instant-payout-001");
        assert!(matches!(
            request.payout_method_details,
            PayoutMethodDetails::Card(_)
        ));
    }

    #[test]
    fn test_confirm_request_builder() {
        let request = ConfirmRequest::builder()
            .merchant_account("TestMerchant")
            .original_reference("8515131751004933")
            .build()
            .unwrap();

        assert_eq!(&*request.merchant_account, "TestMerchant");
        assert_eq!(&*request.original_reference, "8515131751004933");
    }

    #[test]
    fn test_review_payout_request_builder() {
        let request = ReviewPayoutRequest::builder()
            .merchant_account("TestMerchant")
            .psp_reference("8515131751004933")
            .build()
            .unwrap();

        assert_eq!(&*request.merchant_account, "TestMerchant");
        assert_eq!(&*request.psp_reference, "8515131751004933");
    }

    #[test]
    fn test_decline_payout_request_builder() {
        let request = DeclinePayoutRequest::builder()
            .merchant_account("TestMerchant")
            .psp_reference("8515131751004933")
            .build()
            .unwrap();

        assert_eq!(&*request.merchant_account, "TestMerchant");
        assert_eq!(&*request.psp_reference, "8515131751004933");
    }

    #[test]
    fn test_submit_request_missing_required_fields() {
        let result = SubmitRequest::builder()
            .amount(Amount::from_minor_units(1000, Currency::EUR))
            .merchant_account("TestMerchant")
            // Missing required fields
            .build();

        assert!(result.is_err());
    }

    #[test]
    fn test_confirm_request_missing_required_fields() {
        let result = ConfirmRequest::builder()
            .merchant_account("TestMerchant")
            // Missing original_reference
            .build();

        assert!(result.is_err());
    }

    #[test]
    fn test_review_request_missing_required_fields() {
        let result = ReviewPayoutRequest::builder()
            .merchant_account("TestMerchant")
            // Missing psp_reference
            .build();

        assert!(result.is_err());
    }

    #[test]
    fn test_decline_request_missing_required_fields() {
        let result = DeclinePayoutRequest::builder()
            .merchant_account("TestMerchant")
            // Missing psp_reference
            .build();

        assert!(result.is_err());
    }
}

#[cfg(test)]
mod serialization_tests {
    use super::*;

    #[test]
    fn test_submit_request_serialization() {
        let bank_account = create_test_bank_account();
        let amount = Amount::from_minor_units(10000, Currency::EUR);

        let request = SubmitRequest::builder()
            .amount(amount)
            .merchant_account("TestMerchant")
            .reference("payout-001")
            .shopper_email("test@example.com")
            .shopper_reference("shopper-001")
            .payout_method_details(PayoutMethodDetails::BankAccount(bank_account))
            .build()
            .unwrap();

        let json = serde_json::to_string(&request).unwrap();
        let deserialized: SubmitRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(
            request.amount.minor_units(),
            deserialized.amount.minor_units()
        );
        assert_eq!(request.merchant_account, deserialized.merchant_account);
        assert_eq!(request.reference, deserialized.reference);
    }

    #[test]
    fn test_bank_account_serialization() {
        let bank_account = create_test_bank_account();
        let json = serde_json::to_string(&bank_account).unwrap();
        let deserialized: BankAccount = serde_json::from_str(&json).unwrap();

        assert_eq!(bank_account.account_number, deserialized.account_number);
        assert_eq!(bank_account.owner_name, deserialized.owner_name);
        assert_eq!(bank_account.country_code, deserialized.country_code);
    }

    #[test]
    fn test_card_serialization() {
        let card = create_test_card();
        let json = serde_json::to_string(&card).unwrap();
        let deserialized: Card = serde_json::from_str(&json).unwrap();

        assert_eq!(card.number, deserialized.number);
        assert_eq!(card.holder_name, deserialized.holder_name);
        assert_eq!(card.expiry_month, deserialized.expiry_month);
        assert_eq!(card.expiry_year, deserialized.expiry_year);
    }

    #[test]
    fn test_payout_method_details_bank_account_serialization() {
        let bank_account = create_test_bank_account();
        let details = PayoutMethodDetails::BankAccount(bank_account);

        let json = serde_json::to_string(&details).unwrap();
        assert!(json.contains("\"type\":\"bankAccount\""));

        let deserialized: PayoutMethodDetails = serde_json::from_str(&json).unwrap();
        match deserialized {
            PayoutMethodDetails::BankAccount(_) => {}
            _ => panic!("Expected BankAccount variant"),
        }
    }

    #[test]
    fn test_payout_method_details_card_serialization() {
        let card = create_test_card();
        let details = PayoutMethodDetails::Card(card);

        let json = serde_json::to_string(&details).unwrap();
        assert!(json.contains("\"type\":\"card\""));

        let deserialized: PayoutMethodDetails = serde_json::from_str(&json).unwrap();
        match deserialized {
            PayoutMethodDetails::Card(_) => {}
            _ => panic!("Expected Card variant"),
        }
    }

    #[test]
    fn test_address_serialization() {
        let address = create_test_address();
        let json = serde_json::to_string(&address).unwrap();
        let deserialized: Address = serde_json::from_str(&json).unwrap();

        assert_eq!(address.city, deserialized.city);
        assert_eq!(address.country, deserialized.country);
        assert_eq!(address.street, deserialized.street);
        assert_eq!(address.postal_code, deserialized.postal_code);
    }

    #[test]
    fn test_name_serialization() {
        let name = create_test_name();
        let json = serde_json::to_string(&name).unwrap();
        let deserialized: Name = serde_json::from_str(&json).unwrap();

        assert_eq!(name.first_name, deserialized.first_name);
        assert_eq!(name.last_name, deserialized.last_name);
    }

    #[test]
    fn test_entity_type_serialization() {
        let person = EntityType::NaturalPerson;
        let json = serde_json::to_string(&person).unwrap();
        assert_eq!(json, "\"NaturalPerson\"");

        let company = EntityType::Company;
        let json = serde_json::to_string(&company).unwrap();
        assert_eq!(json, "\"Company\"");
    }

    #[test]
    fn test_bank_account_type_serialization() {
        let checking = BankAccountType::Checking;
        let json = serde_json::to_string(&checking).unwrap();
        assert_eq!(json, "\"checking\"");

        let savings = BankAccountType::Savings;
        let json = serde_json::to_string(&savings).unwrap();
        assert_eq!(json, "\"savings\"");
    }
}

#[cfg(test)]
mod workflow_tests {
    use super::*;

    /// Test the complete instant payout workflow:
    /// 1. Submit payout request
    /// 2. Confirm the payout
    #[test]
    fn test_instant_payout_workflow_types() {
        // Step 1: Create submit request
        let bank_account = create_test_bank_account();
        let amount = Amount::from_minor_units(10000, Currency::EUR);

        let submit_request = SubmitRequest::builder()
            .amount(amount)
            .merchant_account("TestMerchant")
            .reference("instant-payout-001")
            .shopper_email("instant@example.com")
            .shopper_reference("instant-shopper-001")
            .payout_method_details(PayoutMethodDetails::BankAccount(bank_account))
            .build()
            .unwrap();

        // Verify submit request
        assert_eq!(submit_request.amount.minor_units(), 10000);
        assert_eq!(&*submit_request.reference, "instant-payout-001");

        // Step 2: Create confirm request (using mock PSP reference)
        let confirm_request = ConfirmRequest::builder()
            .merchant_account("TestMerchant")
            .original_reference("8515131751004933") // Mock PSP reference from submit
            .build()
            .unwrap();

        // Verify confirm request
        assert_eq!(&*confirm_request.merchant_account, "TestMerchant");
        assert_eq!(&*confirm_request.original_reference, "8515131751004933");
    }

    /// Test the review workflow for payouts requiring manual approval:
    /// 1. Submit payout request (flagged for review)
    /// 2. Review and approve OR decline the payout
    #[test]
    fn test_review_workflow_types() {
        // Step 1: Create submit request that might be flagged for review
        let bank_account = create_test_bank_account();
        let amount = Amount::from_minor_units(100000, Currency::EUR); // Large amount likely to trigger review

        let submit_request = SubmitRequest::builder()
            .amount(amount)
            .merchant_account("TestMerchant")
            .reference("review-payout-001")
            .shopper_email("review@example.com")
            .shopper_reference("review-shopper-001")
            .payout_method_details(PayoutMethodDetails::BankAccount(bank_account))
            .build()
            .unwrap();

        assert_eq!(submit_request.amount.minor_units(), 100000);

        // Step 2a: Create review (approve) request
        let review_request = ReviewPayoutRequest::builder()
            .merchant_account("TestMerchant")
            .psp_reference("8515131751004933") // Mock PSP reference from submit
            .build()
            .unwrap();

        assert_eq!(&*review_request.psp_reference, "8515131751004933");

        // Step 2b: Create decline request (alternative to approval)
        let decline_request = DeclinePayoutRequest::builder()
            .merchant_account("TestMerchant")
            .psp_reference("8515131751004933")
            .build()
            .unwrap();

        assert_eq!(&*decline_request.psp_reference, "8515131751004933");
    }

    /// Test the store detail workflow for setting up future payouts:
    /// 1. Store payout details without immediate payout
    /// 2. Later submit payout using stored details
    #[test]
    fn test_store_detail_workflow_types() {
        // Step 1: Store details without payout (amount = 0)
        let bank_account = create_test_bank_account();
        let store_request = SubmitRequest::builder()
            .amount(Amount::from_minor_units(0, Currency::EUR)) // Amount 0 for storing only
            .merchant_account("TestMerchant")
            .reference("store-detail-001")
            .shopper_email("store@example.com")
            .shopper_reference("store-shopper-001")
            .payout_method_details(PayoutMethodDetails::BankAccount(bank_account))
            .build()
            .unwrap();

        assert_eq!(store_request.amount.minor_units(), 0);
        assert_eq!(&*store_request.reference, "store-detail-001");

        // Step 2: Later use stored details for actual payout
        let bank_account = create_test_bank_account();
        let payout_request = SubmitRequest::builder()
            .amount(Amount::from_minor_units(5000, Currency::EUR))
            .merchant_account("TestMerchant")
            .reference("stored-payout-001")
            .shopper_email("store@example.com")
            .shopper_reference("store-shopper-001") // Same shopper reference
            .payout_method_details(PayoutMethodDetails::BankAccount(bank_account))
            .build()
            .unwrap();

        assert_eq!(payout_request.amount.minor_units(), 5000);
        assert_eq!(
            payout_request.shopper_reference,
            store_request.shopper_reference
        );
    }

    /// Test the instant card payout workflow for immediate card payouts:
    /// 1. Create instant payout request for card
    /// 2. Verify request structure for instant processing
    #[test]
    fn test_instant_card_payout_workflow_types() {
        // Create instant payout request for card (funds available within 30 minutes)
        let card = create_test_card();
        let amount = Amount::from_minor_units(7500, Currency::GBP); // £75.00

        let instant_request = SubmitRequest::builder()
            .amount(amount)
            .merchant_account("TestMerchant")
            .reference("instant-card-payout-001")
            .shopper_email("instant@example.com")
            .shopper_reference("instant-card-shopper-001")
            .payout_method_details(PayoutMethodDetails::Card(card))
            .build()
            .unwrap();

        // Verify instant payout request structure
        assert_eq!(instant_request.amount.minor_units(), 7500);
        assert_eq!(instant_request.amount.currency(), Currency::GBP);
        assert_eq!(&*instant_request.reference, "instant-card-payout-001");
        assert!(matches!(
            instant_request.payout_method_details,
            PayoutMethodDetails::Card(_)
        ));

        // Instant payouts are typically for cards and don't require confirmation workflow
        if let PayoutMethodDetails::Card(ref card_details) = instant_request.payout_method_details {
            assert_eq!(&*card_details.holder_name, "John Doe");
            assert_eq!(&*card_details.number, "4111111111111111");
        }
    }
}

#[cfg(test)]
mod error_handling_tests {
    use super::*;
    use adyen_core::AdyenError;

    #[test]
    fn test_submit_request_validation() {
        // Test missing amount
        let result = SubmitRequest::builder()
            .merchant_account("TestMerchant")
            .reference("test-ref")
            .shopper_email("test@example.com")
            .shopper_reference("shopper-ref")
            .payout_method_details(PayoutMethodDetails::BankAccount(create_test_bank_account()))
            .build();

        assert!(matches!(result, Err(AdyenError::Config(_))));

        // Test missing merchant account
        let result = SubmitRequest::builder()
            .amount(Amount::from_minor_units(1000, Currency::EUR))
            .reference("test-ref")
            .shopper_email("test@example.com")
            .shopper_reference("shopper-ref")
            .payout_method_details(PayoutMethodDetails::BankAccount(create_test_bank_account()))
            .build();

        assert!(matches!(result, Err(AdyenError::Config(_))));
    }

    #[test]
    fn test_confirm_request_validation() {
        // Test missing merchant account
        let result = ConfirmRequest::builder()
            .original_reference("8515131751004933")
            .build();

        assert!(matches!(result, Err(AdyenError::Config(_))));

        // Test missing original reference
        let result = ConfirmRequest::builder()
            .merchant_account("TestMerchant")
            .build();

        assert!(matches!(result, Err(AdyenError::Config(_))));
    }

    #[test]
    fn test_review_request_validation() {
        // Test missing PSP reference
        let result = ReviewPayoutRequest::builder()
            .merchant_account("TestMerchant")
            .build();

        assert!(matches!(result, Err(AdyenError::Config(_))));
    }

    #[test]
    fn test_decline_request_validation() {
        // Test missing PSP reference
        let result = DeclinePayoutRequest::builder()
            .merchant_account("TestMerchant")
            .build();

        assert!(matches!(result, Err(AdyenError::Config(_))));
    }
}
