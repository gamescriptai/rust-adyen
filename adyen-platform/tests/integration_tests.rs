//! Integration tests for the Adyen Balance Platform API v2.

use adyen_core::{ConfigBuilder, Environment};
use adyen_platform::types::*;
use adyen_platform::{
    BalancePlatformApi, CreateAccountHolderRequest, CreateBalanceAccountRequest,
    CreatePaymentInstrumentRequest, CreateTransactionRuleRequest,
};
use std::collections::HashMap;

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
    fn test_create_balance_account_request_builder() {
        let mut metadata = HashMap::new();
        metadata.insert("purpose".to_string(), "marketplace".to_string());

        let request = CreateBalanceAccountRequest::builder()
            .account_holder_id("AH12345")
            .description("Main balance account")
            .reference("BA_001")
            .default_currency_code("EUR")
            .metadata(metadata)
            .time_zone("Europe/Amsterdam")
            .build()
            .unwrap();

        assert_eq!(request.account_holder_id.as_ref(), "AH12345");
        assert_eq!(
            request.description.as_ref().unwrap().as_ref(),
            "Main balance account"
        );
        assert_eq!(request.reference.as_ref().unwrap().as_ref(), "BA_001");
        assert_eq!(
            request.default_currency_code.as_ref().unwrap().as_ref(),
            "EUR"
        );
        assert!(request.metadata.is_some());
        assert_eq!(
            request.time_zone.as_ref().unwrap().as_ref(),
            "Europe/Amsterdam"
        );
    }

    #[test]
    fn test_create_account_holder_request() {
        let contact_details = ContactDetails {
            email: Some("test@example.com".into()),
            phone: Some(Phone {
                number: "+31234567890".into(),
                r#type: PhoneType::Mobile,
            }),
            address: Some(Address {
                country: "NL".into(),
                city: Some("Amsterdam".into()),
                postal_code: Some("1012AB".into()),
                state_or_province: Some("Noord-Holland".into()),
                street_address: Some("Dam Square 1".into()),
                street_address2: None,
            }),
            website_url: Some("https://example.com".into()),
        };

        let request = CreateAccountHolderRequest {
            legal_entity_id: "LE12345".into(),
            reference: Some("AH_001".into()),
            description: Some("Test account holder".into()),
            contact_details: Some(contact_details),
            time_zone: Some("Europe/Amsterdam".into()),
            metadata: None,
        };

        assert_eq!(request.legal_entity_id.as_ref(), "LE12345");
        assert_eq!(request.reference.as_ref().unwrap().as_ref(), "AH_001");
        assert!(request.contact_details.is_some());
    }

    #[test]
    fn test_create_payment_instrument_request() {
        let card_request = CreateCardRequest {
            brand: "visa".into(),
            brand_variant: Some("credit".into()),
            form_factor: Some(CardFormFactor::Virtual),
            currency: Some("EUR".into()),
        };

        let request = CreatePaymentInstrumentRequest {
            balance_account_id: "BA12345".into(),
            r#type: PaymentInstrumentType::Card,
            description: Some("Virtual Visa card".into()),
            reference: Some("PI_001".into()),
            card: Some(card_request),
            bank_account: None,
        };

        assert_eq!(request.balance_account_id.as_ref(), "BA12345");
        assert!(matches!(request.r#type, PaymentInstrumentType::Card));
        assert!(request.card.is_some());
        assert!(request.bank_account.is_none());
    }

    #[test]
    fn test_create_transaction_rule_request() {
        let restrictions = TransactionRuleRestrictions {
            max_amount: Some(Amount {
                currency: "EUR".into(),
                value: 50000, // €500.00
            }),
            velocity: Some(VelocityRestriction {
                max_amount: Some(Amount {
                    currency: "EUR".into(),
                    value: 100000, // €1000.00
                }),
                time_period: TimePeriod::Daily,
            }),
            processing_types: None,
            time_period: None,
        };

        let entity_key = EntityKey {
            entity_type: EntityType::BalanceAccount,
            entity_reference: "BA12345".into(),
        };

        let request = CreateTransactionRuleRequest {
            description: Some("Daily spending limit".into()),
            reference: Some("TR_001".into()),
            r#type: TransactionRuleType::Velocity,
            rule_restrictions: restrictions,
            entity_key,
            outcome_type: OutcomeType::HardBlock,
        };

        assert_eq!(
            request.description.as_ref().unwrap().as_ref(),
            "Daily spending limit"
        );
        assert!(matches!(request.r#type, TransactionRuleType::Velocity));
        assert!(matches!(request.outcome_type, OutcomeType::HardBlock));
    }
}

#[cfg(test)]
mod serialization_tests {
    use super::*;

    #[test]
    fn test_create_balance_account_request_serialization() {
        let request = CreateBalanceAccountRequest::builder()
            .account_holder_id("AH12345")
            .description("Test account")
            .default_currency_code("EUR")
            .build()
            .unwrap();

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"accountHolderId\":\"AH12345\""));
        assert!(json.contains("\"description\":\"Test account\""));
        assert!(json.contains("\"defaultCurrencyCode\":\"EUR\""));

        let _deserialized: CreateBalanceAccountRequest = serde_json::from_str(&json).unwrap();
    }

    #[test]
    fn test_balance_account_serialization() {
        let balance_account = BalanceAccount {
            id: "BA12345".into(),
            description: Some("Test account".into()),
            default_currency_code: Some("EUR".into()),
            account_holder: AccountHolder {
                id: "AH12345".into(),
                reference: Some("AH_001".into()),
                legal_entity_id: "LE12345".into(),
                description: Some("Test holder".into()),
                status: AccountHolderStatus::Active,
                capabilities: None,
                contact_details: None,
                time_zone: None,
                metadata: None,
            },
            balances: Some(vec![Balance {
                currency: "EUR".into(),
                available: 100000,
                pending: Some(5000),
                reserved: Some(2000),
            }]),
            metadata: None,
            status: BalanceAccountStatus::Active,
            time_zone: Some("Europe/Amsterdam".into()),
        };

        let json = serde_json::to_string(&balance_account).unwrap();
        let _deserialized: BalanceAccount = serde_json::from_str(&json).unwrap();
    }

    #[test]
    fn test_payment_instrument_serialization() {
        let payment_instrument = PaymentInstrument {
            id: "PI12345".into(),
            r#type: PaymentInstrumentType::Card,
            status: PaymentInstrumentStatus::Active,
            balance_account_id: "BA12345".into(),
            description: Some("Virtual card".into()),
            reference: Some("PI_001".into()),
            card: Some(Card {
                brand: "visa".into(),
                brand_variant: Some("credit".into()),
                expiry_month: 12,
                expiry_year: 2025,
                form_factor: Some(CardFormFactor::Virtual),
                last_four: "1234".into(),
                bin: Some("411111".into()),
            }),
            bank_account: None,
        };

        let json = serde_json::to_string(&payment_instrument).unwrap();
        let _deserialized: PaymentInstrument = serde_json::from_str(&json).unwrap();
    }

    #[test]
    fn test_enum_serialization() {
        assert_eq!(
            serde_json::to_string(&BalanceAccountStatus::Active).unwrap(),
            "\"active\""
        );
        assert_eq!(
            serde_json::to_string(&PaymentInstrumentType::Card).unwrap(),
            "\"card\""
        );
        assert_eq!(
            serde_json::to_string(&CardFormFactor::Virtual).unwrap(),
            "\"virtual\""
        );
        assert_eq!(
            serde_json::to_string(&TransactionRuleType::Velocity).unwrap(),
            "\"velocity\""
        );
        assert_eq!(
            serde_json::to_string(&OutcomeType::HardBlock).unwrap(),
            "\"hardBlock\""
        );
    }
}

#[cfg(test)]
mod validation_tests {
    use super::*;

    #[test]
    fn test_create_balance_account_request_validation() {
        // Missing required account_holder_id should fail
        let result = CreateBalanceAccountRequest::builder().build();
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().as_ref(),
            "account_holder_id is required"
        );
    }

    #[test]
    fn test_valid_balance_account_request() {
        let request = CreateBalanceAccountRequest::builder()
            .account_holder_id("AH12345")
            .build()
            .unwrap();

        assert_eq!(request.account_holder_id.as_ref(), "AH12345");
        assert!(request.description.is_none());
        assert!(request.metadata.is_none());
    }
}

#[cfg(test)]
mod workflow_tests {
    use super::*;

    #[test]
    fn test_marketplace_onboarding_workflow_types() {
        // 1. Create Account Holder
        let account_holder = AccountHolder {
            id: "AH12345".into(),
            reference: Some("marketplace_seller_001".into()),
            legal_entity_id: "LE12345".into(),
            description: Some("Marketplace seller".into()),
            status: AccountHolderStatus::Active,
            capabilities: None,
            contact_details: Some(ContactDetails {
                email: Some("seller@example.com".into()),
                phone: Some(Phone {
                    number: "+31234567890".into(),
                    r#type: PhoneType::Mobile,
                }),
                address: Some(Address {
                    country: "NL".into(),
                    city: Some("Amsterdam".into()),
                    postal_code: Some("1012AB".into()),
                    state_or_province: Some("Noord-Holland".into()),
                    street_address: Some("Seller Street 1".into()),
                    street_address2: None,
                }),
                website_url: Some("https://seller.example.com".into()),
            }),
            time_zone: Some("Europe/Amsterdam".into()),
            metadata: None,
        };

        // 2. Create Balance Account
        let balance_account = BalanceAccount {
            id: "BA12345".into(),
            description: Some("Seller balance account".into()),
            default_currency_code: Some("EUR".into()),
            account_holder: account_holder.clone(),
            balances: Some(vec![Balance {
                currency: "EUR".into(),
                available: 0,
                pending: None,
                reserved: None,
            }]),
            metadata: None,
            status: BalanceAccountStatus::Active,
            time_zone: Some("Europe/Amsterdam".into()),
        };

        // 3. Create Payment Instrument (Virtual Card)
        let payment_instrument = PaymentInstrument {
            id: "PI12345".into(),
            r#type: PaymentInstrumentType::Card,
            status: PaymentInstrumentStatus::Active,
            balance_account_id: balance_account.id.clone(),
            description: Some("Seller virtual card".into()),
            reference: Some("seller_card_001".into()),
            card: Some(Card {
                brand: "visa".into(),
                brand_variant: Some("debit".into()),
                expiry_month: 12,
                expiry_year: 2025,
                form_factor: Some(CardFormFactor::Virtual),
                last_four: "5678".into(),
                bin: Some("411111".into()),
            }),
            bank_account: None,
        };

        // 4. Create Transaction Rule (Spending Limits)
        let transaction_rule = TransactionRule {
            id: "TR12345".into(),
            description: Some("Daily spending limit for seller".into()),
            reference: Some("seller_daily_limit".into()),
            status: TransactionRuleStatus::Active,
            r#type: TransactionRuleType::Velocity,
            rule_restrictions: TransactionRuleRestrictions {
                max_amount: Some(Amount {
                    currency: "EUR".into(),
                    value: 500000, // €5000.00 per transaction
                }),
                velocity: Some(VelocityRestriction {
                    max_amount: Some(Amount {
                        currency: "EUR".into(),
                        value: 2000000, // €20000.00 per day
                    }),
                    time_period: TimePeriod::Daily,
                }),
                processing_types: None,
                time_period: None,
            },
            entity_key: EntityKey {
                entity_type: EntityType::BalanceAccount,
                entity_reference: balance_account.id.clone(),
            },
            outcome_type: OutcomeType::HardBlock,
        };

        // Verify the workflow relationships
        assert_eq!(balance_account.account_holder.id, account_holder.id);
        assert_eq!(payment_instrument.balance_account_id, balance_account.id);
        assert_eq!(
            transaction_rule.entity_key.entity_reference,
            balance_account.id
        );
        assert!(matches!(
            payment_instrument.r#type,
            PaymentInstrumentType::Card
        ));
        assert!(matches!(
            transaction_rule.outcome_type,
            OutcomeType::HardBlock
        ));
    }

    #[test]
    fn test_payment_processing_workflow_types() {
        // Bank Account Payment Instrument
        let bank_account_instrument = PaymentInstrument {
            id: "PI_BANK_001".into(),
            r#type: PaymentInstrumentType::BankAccount,
            status: PaymentInstrumentStatus::Active,
            balance_account_id: "BA12345".into(),
            description: Some("Business bank account".into()),
            reference: Some("bank_account_001".into()),
            card: None,
            bank_account: Some(BankAccount {
                account_number: "1234567890".into(),
                routing_number: Some("021000021".into()),
                iban: Some("NL91ABNA0417164300".into()),
                bic: Some("ABNANL2A".into()),
                account_holder_name: "Business Corp".into(),
                bank_name: Some("ABN AMRO Bank".into()),
                country_code: "NL".into(),
                currency: "EUR".into(),
            }),
        };

        // Multi-currency Balance Account
        let balance_account = BalanceAccount {
            id: "BA_MULTI_001".into(),
            description: Some("Multi-currency balance account".into()),
            default_currency_code: Some("EUR".into()),
            account_holder: AccountHolder {
                id: "AH_MULTI_001".into(),
                reference: Some("multi_currency_holder".into()),
                legal_entity_id: "LE_MULTI_001".into(),
                description: Some("Multi-currency account holder".into()),
                status: AccountHolderStatus::Active,
                capabilities: None,
                contact_details: None,
                time_zone: Some("Europe/Amsterdam".into()),
                metadata: None,
            },
            balances: Some(vec![
                Balance {
                    currency: "EUR".into(),
                    available: 100000,
                    pending: Some(5000),
                    reserved: Some(2000),
                },
                Balance {
                    currency: "USD".into(),
                    available: 150000,
                    pending: Some(7500),
                    reserved: Some(3000),
                },
                Balance {
                    currency: "GBP".into(),
                    available: 80000,
                    pending: Some(4000),
                    reserved: Some(1500),
                },
            ]),
            metadata: None,
            status: BalanceAccountStatus::Active,
            time_zone: Some("Europe/Amsterdam".into()),
        };

        // Geographic Transaction Rule
        let geo_rule = TransactionRule {
            id: "TR_GEO_001".into(),
            description: Some("Restrict transactions to EU".into()),
            reference: Some("eu_only_rule".into()),
            status: TransactionRuleStatus::Active,
            r#type: TransactionRuleType::AllowList,
            rule_restrictions: TransactionRuleRestrictions {
                max_amount: None,
                velocity: None,
                processing_types: Some(ProcessingTypesRestriction {
                    types: vec![ProcessingType::Ecommerce, ProcessingType::Pos],
                    operation: RestrictionOperation::Include,
                }),
                time_period: None,
            },
            entity_key: EntityKey {
                entity_type: EntityType::BalanceAccount,
                entity_reference: balance_account.id.clone(),
            },
            outcome_type: OutcomeType::HardBlock,
        };

        // Verify multi-currency and geographic controls
        assert!(bank_account_instrument.bank_account.is_some());
        assert_eq!(balance_account.balances.as_ref().unwrap().len(), 3);
        assert!(matches!(geo_rule.r#type, TransactionRuleType::AllowList));
        assert!(geo_rule.rule_restrictions.processing_types.is_some());
    }

    #[test]
    fn test_risk_management_workflow_types() {
        // Comprehensive Transaction Rule with multiple restrictions
        let comprehensive_rule = TransactionRule {
            id: "TR_RISK_001".into(),
            description: Some("Comprehensive risk management rule".into()),
            reference: Some("risk_rule_001".into()),
            status: TransactionRuleStatus::Active,
            r#type: TransactionRuleType::Velocity,
            rule_restrictions: TransactionRuleRestrictions {
                max_amount: Some(Amount {
                    currency: "EUR".into(),
                    value: 100000, // €1000.00 max per transaction
                }),
                velocity: Some(VelocityRestriction {
                    max_amount: Some(Amount {
                        currency: "EUR".into(),
                        value: 500000, // €5000.00 per day
                    }),
                    time_period: TimePeriod::Daily,
                }),
                processing_types: Some(ProcessingTypesRestriction {
                    types: vec![ProcessingType::Ecommerce, ProcessingType::Moto],
                    operation: RestrictionOperation::Include,
                }),
                time_period: Some(TimePeriodRestriction {
                    start_time: Some("09:00".into()),
                    end_time: Some("17:00".into()),
                    time_zone: Some("Europe/Amsterdam".into()),
                }),
            },
            entity_key: EntityKey {
                entity_type: EntityType::PaymentInstrument,
                entity_reference: "PI12345".into(),
            },
            outcome_type: OutcomeType::AdviseOnly,
        };

        // Verify comprehensive restrictions
        assert!(comprehensive_rule.rule_restrictions.max_amount.is_some());
        assert!(comprehensive_rule.rule_restrictions.velocity.is_some());
        assert!(comprehensive_rule
            .rule_restrictions
            .processing_types
            .is_some());
        assert!(comprehensive_rule.rule_restrictions.time_period.is_some());
        assert!(matches!(
            comprehensive_rule.outcome_type,
            OutcomeType::AdviseOnly
        ));
    }
}

#[cfg(test)]
mod api_tests {
    use super::*;

    #[test]
    fn test_balance_platform_api_creation() {
        let config = create_test_config();
        let _api = BalancePlatformApi::new(config).unwrap();
        // API created successfully indicates proper configuration
    }
}
