//! Integration tests for the Adyen Legal Entity API v3.

use adyen_core::{ConfigBuilder, Environment};
use adyen_legal_entity::{
    LegalEntityApi, LegalEntityInfo, BusinessLineInfo, TransferInstrumentInfo, OnboardingLinkInfo
};
use adyen_legal_entity::types::*;

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
    fn test_individual_legal_entity_builder() {
        let individual = Individual {
            name: Name {
                first_name: "John".into(),
                last_name: "Doe".into(),
                in_fix: Some("Michael".into()),
            },
            email: Some("john.doe@example.com".into()),
            phone: Some(PhoneNumber {
                number: "+1234567890".into(),
                r#type: PhoneType::Mobile,
            }),
            birth_data: Some(BirthData {
                date_of_birth: "1985-03-15".into(),
                city_of_birth: Some("New York".into()),
                country_of_birth: Some("US".into()),
                state_or_province_of_birth: Some("NY".into()),
            }),
            nationality: Some("US".into()),
            identification_data: Some(IdentificationData {
                r#type: IdentificationType::Passport,
                number: "P123456789".into(),
                issuer_country: Some("US".into()),
                issuer_state: None,
                expiry_date: Some("2030-03-15".into()),
            }),
            residential_address: Some(Address {
                country: "US".into(),
                city: Some("New York".into()),
                postal_code: Some("10001".into()),
                state_or_province: Some("NY".into()),
                street_address: Some("123 Main St".into()),
                street_address2: Some("Apt 4B".into()),
            }),
            tax_information: Some(vec![TaxInformation {
                country: "US".into(),
                number: Some("123-45-6789".into()),
                r#type: Some(TaxIdType::UsSsn),
                tax_reporting_classification: Some(TaxReportingClassification {
                    business_type: Some(TaxBusinessType::Individual),
                    commercial_type: None,
                    financial_institution_number: None,
                }),
            }]),
        };

        let request = LegalEntityInfo::builder()
            .entity_type(LegalEntityType::Individual)
            .individual(individual)
            .reference("individual_001")
            .build()
            .unwrap();

        assert!(matches!(request.r#type, LegalEntityType::Individual));
        assert!(request.individual.is_some());
        assert_eq!(request.reference.as_ref().unwrap().as_ref(), "individual_001");
    }

    #[test]
    fn test_organization_legal_entity() {
        let organization = Organization {
            legal_name: "Example Corp".into(),
            trading_name: Some("Example Store".into()),
            registration_number: Some("123456789".into()),
            tax_id: Some("98-7654321".into()),
            date_of_incorporation: Some("2020-01-15".into()),
            r#type: Some(OrganizationType::LimitedLiabilityCompany),
            vat_number: Some("GB123456789".into()),
            vat_exemption: None,
            registered_address: Some(Address {
                country: "US".into(),
                city: Some("San Francisco".into()),
                postal_code: Some("94105".into()),
                state_or_province: Some("CA".into()),
                street_address: Some("456 Business Ave".into()),
                street_address2: None,
            }),
            principal_business_address: Some(Address {
                country: "US".into(),
                city: Some("San Francisco".into()),
                postal_code: Some("94105".into()),
                state_or_province: Some("CA".into()),
                street_address: Some("456 Business Ave".into()),
                street_address2: None,
            }),
            email: Some("contact@example.com".into()),
            phone: Some(PhoneNumber {
                number: "+14155551234".into(),
                r#type: PhoneType::Landline,
            }),
            web_data: Some(WebData {
                website_url: Some("https://example.com".into()),
                web_data_exemption: None,
            }),
            stock_data: Some(StockData {
                market: Some("NASDAQ".into()),
                ticker_symbol: Some("EXPL".into()),
                isin: Some("US1234567890".into()),
                cusip: Some("123456789".into()),
            }),
            tax_information: Some(vec![TaxInformation {
                country: "US".into(),
                number: Some("98-7654321".into()),
                r#type: Some(TaxIdType::UsEin),
                tax_reporting_classification: Some(TaxReportingClassification {
                    business_type: Some(TaxBusinessType::Corporation),
                    commercial_type: Some(TaxCommercialType::NonFinancialEntity),
                    financial_institution_number: None,
                }),
            }]),
        };

        let request = LegalEntityInfo {
            r#type: LegalEntityType::Organization,
            individual: None,
            organization: Some(organization),
            sole_proprietorship: None,
            trust: None,
            unincorporated_partnership: None,
            reference: Some("org_001".into()),
        };

        assert!(matches!(request.r#type, LegalEntityType::Organization));
        assert!(request.organization.is_some());
        assert_eq!(request.organization.as_ref().unwrap().legal_name.as_ref(), "Example Corp");
    }

    #[test]
    fn test_business_line_info() {
        let request = BusinessLineInfo {
            legal_entity_id: "LE12345".into(),
            industry: "5411".into(), // Grocery stores
            industry_code: Some("GROCERY_RETAIL".into()),
            sales_channels: Some(vec![SalesChannel::Pos, SalesChannel::Ecommerce]),
            web_data: Some(vec![WebData {
                website_url: Some("https://grocery.example.com".into()),
                web_data_exemption: None,
            }]),
            service: Some("Grocery retail and online delivery".into()),
        };

        assert_eq!(request.legal_entity_id.as_ref(), "LE12345");
        assert_eq!(request.industry.as_ref(), "5411");
        assert!(request.sales_channels.is_some());
    }

    #[test]
    fn test_transfer_instrument_info() {
        let bank_account = BankAccountInfo {
            account_holder: "Example Corp".into(),
            account_identification: BankAccountIdentification::UsLocal(UsLocalAccountIdentification {
                account_number: "123456789".into(),
                account_type: Some(UsAccountType::Checking),
                routing_number: "021000021".into(),
                r#type: "usLocal".into(),
            }),
        };

        let request = TransferInstrumentInfo {
            legal_entity_id: "LE12345".into(),
            r#type: TransferInstrumentType::BankAccount,
            bank_account: Some(bank_account),
        };

        assert_eq!(request.legal_entity_id.as_ref(), "LE12345");
        assert!(matches!(request.r#type, TransferInstrumentType::BankAccount));
        assert!(request.bank_account.is_some());
    }

    #[test]
    fn test_onboarding_link_info() {
        let settings = OnboardingLinkSettings {
            collect_entity_types: Some(vec![LegalEntityType::Individual, LegalEntityType::Organization]),
            enable_manual_review: Some(true),
            required_verification_checks: Some(vec![
                VerificationCheckType::IdentityVerification,
                VerificationCheckType::CompanyVerification,
            ]),
        };

        let request = OnboardingLinkInfo {
            legal_entity_id: "LE12345".into(),
            settings: Some(settings),
            theme_id: Some("theme_123".into()),
        };

        assert_eq!(request.legal_entity_id.as_ref(), "LE12345");
        assert!(request.settings.is_some());
        assert_eq!(request.theme_id.as_ref().unwrap().as_ref(), "theme_123");
    }
}

#[cfg(test)]
mod serialization_tests {
    use super::*;

    #[test]
    fn test_legal_entity_info_serialization() {
        let individual = Individual {
            name: Name {
                first_name: "Jane".into(),
                last_name: "Smith".into(),
                in_fix: None,
            },
            email: Some("jane.smith@example.com".into()),
            phone: None,
            birth_data: None,
            nationality: Some("CA".into()),
            identification_data: None,
            residential_address: None,
            tax_information: None,
        };

        let request = LegalEntityInfo {
            r#type: LegalEntityType::Individual,
            individual: Some(individual),
            organization: None,
            sole_proprietorship: None,
            trust: None,
            unincorporated_partnership: None,
            reference: Some("test_ref".into()),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"type\":\"individual\""));
        assert!(json.contains("\"firstName\":\"Jane\""));
        assert!(json.contains("\"reference\":\"test_ref\""));

        let _deserialized: LegalEntityInfo = serde_json::from_str(&json).unwrap();
    }

    #[test]
    fn test_business_line_serialization() {
        let business_line = BusinessLine {
            id: "BL12345".into(),
            legal_entity_id: "LE12345".into(),
            industry: "5411".into(),
            industry_code: Some("GROCERY".into()),
            sales_channels: Some(vec![SalesChannel::Pos]),
            web_data: None,
            service: Some("Grocery retail".into()),
            problems: None,
        };

        let json = serde_json::to_string(&business_line).unwrap();
        let _deserialized: BusinessLine = serde_json::from_str(&json).unwrap();
    }

    #[test]
    fn test_bank_account_identification_serialization() {
        let iban_account = BankAccountIdentification::Iban(IbanAccountIdentification {
            iban: "GB29NWBK60161331926819".into(),
            r#type: "iban".into(),
        });

        let json = serde_json::to_string(&iban_account).unwrap();
        assert!(json.contains("GB29NWBK60161331926819"));

        let us_account = BankAccountIdentification::UsLocal(UsLocalAccountIdentification {
            account_number: "123456789".into(),
            account_type: Some(UsAccountType::Checking),
            routing_number: "021000021".into(),
            r#type: "usLocal".into(),
        });

        let json = serde_json::to_string(&us_account).unwrap();
        assert!(json.contains("\"accountType\":\"checking\""));
    }

    #[test]
    fn test_enum_serialization() {
        assert_eq!(serde_json::to_string(&LegalEntityType::Individual).unwrap(), "\"individual\"");
        assert_eq!(serde_json::to_string(&PhoneType::Mobile).unwrap(), "\"mobile\"");
        assert_eq!(serde_json::to_string(&IdentificationType::Passport).unwrap(), "\"passport\"");
        assert_eq!(serde_json::to_string(&OrganizationType::LimitedLiabilityCompany).unwrap(), "\"limitedLiabilityCompany\"");
        assert_eq!(serde_json::to_string(&SalesChannel::Ecommerce).unwrap(), "\"ecommerce\"");
        assert_eq!(serde_json::to_string(&VerificationStatus::Valid).unwrap(), "\"valid\"");
        assert_eq!(serde_json::to_string(&TransferInstrumentType::BankAccount).unwrap(), "\"bankAccount\"");
    }
}

#[cfg(test)]
mod validation_tests {
    use super::*;

    #[test]
    fn test_legal_entity_info_validation() {
        // Missing required entity type should fail
        let result = LegalEntityInfoBuilder::default().build();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().as_ref(), "entity type is required");
    }

    #[test]
    fn test_valid_legal_entity_info() {
        let request = LegalEntityInfo::builder()
            .entity_type(LegalEntityType::Individual)
            .build()
            .unwrap();

        assert!(matches!(request.r#type, LegalEntityType::Individual));
        assert!(request.individual.is_none());
        assert!(request.reference.is_none());
    }
}

#[cfg(test)]
mod workflow_tests {
    use super::*;

    #[test]
    fn test_individual_kyc_workflow_types() {
        // 1. Create Individual Legal Entity
        let individual = Individual {
            name: Name {
                first_name: "Alice".into(),
                last_name: "Johnson".into(),
                in_fix: None,
            },
            email: Some("alice.johnson@example.com".into()),
            phone: Some(PhoneNumber {
                number: "+1555123456".into(),
                r#type: PhoneType::Mobile,
            }),
            birth_data: Some(BirthData {
                date_of_birth: "1990-05-20".into(),
                city_of_birth: Some("Boston".into()),
                country_of_birth: Some("US".into()),
                state_or_province_of_birth: Some("MA".into()),
            }),
            nationality: Some("US".into()),
            identification_data: Some(IdentificationData {
                r#type: IdentificationType::DriversLicense,
                number: "D123456789".into(),
                issuer_country: Some("US".into()),
                issuer_state: Some("MA".into()),
                expiry_date: Some("2028-05-20".into()),
            }),
            residential_address: Some(Address {
                country: "US".into(),
                city: Some("Boston".into()),
                postal_code: Some("02101".into()),
                state_or_province: Some("MA".into()),
                street_address: Some("789 Elm St".into()),
                street_address2: None,
            }),
            tax_information: Some(vec![TaxInformation {
                country: "US".into(),
                number: Some("987-65-4321".into()),
                r#type: Some(TaxIdType::UsSsn),
                tax_reporting_classification: Some(TaxReportingClassification {
                    business_type: Some(TaxBusinessType::Individual),
                    commercial_type: None,
                    financial_institution_number: None,
                }),
            }]),
        };

        let legal_entity_request = LegalEntityInfo {
            r#type: LegalEntityType::Individual,
            individual: Some(individual),
            organization: None,
            sole_proprietorship: None,
            trust: None,
            unincorporated_partnership: None,
            reference: Some("individual_kyc_001".into()),
        };

        // 2. Document for Verification
        let verification_document = Document {
            id: "DOC12345".into(),
            r#type: DocumentType::DriversLicense,
            pages: Some(vec![DocumentPage {
                content: "base64_encoded_image_data".into(),
                content_type: Some("image/jpeg".into()),
                page_number: Some(1),
            }]),
            owner: Some(EntityReference {
                id: "LE12345".into(),
                r#type: Some(LegalEntityType::Individual),
            }),
            creation_date: Some("2024-01-15T10:30:00Z".into()),
            modification_date: None,
            expiry_date: Some("2028-05-20".into()),
            number: Some("D123456789".into()),
            description: Some("Driver's license for identity verification".into()),
            file_name: Some("drivers_license.jpg".into()),
        };

        // 3. Transfer Instrument for Payouts
        let transfer_instrument = TransferInstrumentInfo {
            legal_entity_id: "LE12345".into(),
            r#type: TransferInstrumentType::BankAccount,
            bank_account: Some(BankAccountInfo {
                account_holder: "Alice Johnson".into(),
                account_identification: BankAccountIdentification::UsLocal(UsLocalAccountIdentification {
                    account_number: "987654321".into(),
                    account_type: Some(UsAccountType::Checking),
                    routing_number: "011000015".into(),
                    r#type: "usLocal".into(),
                }),
            }),
        };

        // 4. Business Line for Individual Freelancer
        let business_line = BusinessLineInfo {
            legal_entity_id: "LE12345".into(),
            industry: "7311".into(), // Advertising agencies
            industry_code: Some("FREELANCE_MARKETING".into()),
            sales_channels: Some(vec![SalesChannel::Ecommerce]),
            web_data: Some(vec![WebData {
                website_url: Some("https://alicejohnson.com".into()),
                web_data_exemption: None,
            }]),
            service: Some("Digital marketing consulting services".into()),
        };

        // Verify the KYC workflow integrity
        assert!(matches!(legal_entity_request.r#type, LegalEntityType::Individual));
        assert!(legal_entity_request.individual.is_some());
        assert!(matches!(verification_document.r#type, DocumentType::DriversLicense));
        assert!(matches!(transfer_instrument.r#type, TransferInstrumentType::BankAccount));
        assert_eq!(business_line.industry.as_ref(), "7311");
    }

    #[test]
    fn test_organization_onboarding_workflow_types() {
        // 1. Create Organization Legal Entity
        let organization = Organization {
            legal_name: "TechCorp Solutions LLC".into(),
            trading_name: Some("TechCorp".into()),
            registration_number: Some("LLC-123456789".into()),
            tax_id: Some("12-3456789".into()),
            date_of_incorporation: Some("2022-03-01".into()),
            r#type: Some(OrganizationType::LimitedLiabilityCompany),
            vat_number: Some("US123456789".into()),
            vat_exemption: None,
            registered_address: Some(Address {
                country: "US".into(),
                city: Some("Austin".into()),
                postal_code: Some("78701".into()),
                state_or_province: Some("TX".into()),
                street_address: Some("100 Congress Ave".into()),
                street_address2: Some("Suite 2000".into()),
            }),
            principal_business_address: Some(Address {
                country: "US".into(),
                city: Some("Austin".into()),
                postal_code: Some("78701".into()),
                state_or_province: Some("TX".into()),
                street_address: Some("100 Congress Ave".into()),
                street_address2: Some("Suite 2000".into()),
            }),
            email: Some("contact@techcorp.com".into()),
            phone: Some(PhoneNumber {
                number: "+15125551234".into(),
                r#type: PhoneType::Landline,
            }),
            web_data: Some(WebData {
                website_url: Some("https://techcorp.com".into()),
                web_data_exemption: None,
            }),
            stock_data: None, // Private company
            tax_information: Some(vec![TaxInformation {
                country: "US".into(),
                number: Some("12-3456789".into()),
                r#type: Some(TaxIdType::UsEin),
                tax_reporting_classification: Some(TaxReportingClassification {
                    business_type: Some(TaxBusinessType::LimitedLiability),
                    commercial_type: Some(TaxCommercialType::NonFinancialEntity),
                    financial_institution_number: None,
                }),
            }]),
        };

        // 2. Multi-Currency Transfer Instruments
        let us_bank_account = TransferInstrumentInfo {
            legal_entity_id: "LE_ORG_001".into(),
            r#type: TransferInstrumentType::BankAccount,
            bank_account: Some(BankAccountInfo {
                account_holder: "TechCorp Solutions LLC".into(),
                account_identification: BankAccountIdentification::UsLocal(UsLocalAccountIdentification {
                    account_number: "1234567890".into(),
                    account_type: Some(UsAccountType::Checking),
                    routing_number: "021000021".into(),
                    r#type: "usLocal".into(),
                }),
            }),
        };

        let eu_bank_account = TransferInstrumentInfo {
            legal_entity_id: "LE_ORG_001".into(),
            r#type: TransferInstrumentType::BankAccount,
            bank_account: Some(BankAccountInfo {
                account_holder: "TechCorp Solutions LLC".into(),
                account_identification: BankAccountIdentification::Iban(IbanAccountIdentification {
                    iban: "DE89370400440532013000".into(),
                    r#type: "iban".into(),
                }),
            }),
        };

        // 3. Multiple Business Lines
        let software_business_line = BusinessLineInfo {
            legal_entity_id: "LE_ORG_001".into(),
            industry: "5734".into(), // Computer software stores
            industry_code: Some("SOFTWARE_DEVELOPMENT".into()),
            sales_channels: Some(vec![SalesChannel::Ecommerce, SalesChannel::ContAuth]),
            web_data: Some(vec![WebData {
                website_url: Some("https://techcorp.com/software".into()),
                web_data_exemption: None,
            }]),
            service: Some("Custom software development and SaaS solutions".into()),
        };

        let consulting_business_line = BusinessLineInfo {
            legal_entity_id: "LE_ORG_001".into(),
            industry: "7379".into(), // Computer related services
            industry_code: Some("IT_CONSULTING".into()),
            sales_channels: Some(vec![SalesChannel::ContAuth]),
            web_data: Some(vec![WebData {
                website_url: Some("https://techcorp.com/consulting".into()),
                web_data_exemption: None,
            }]),
            service: Some("IT consulting and digital transformation services".into()),
        };

        // 4. Hosted Onboarding for Sub-entities
        let onboarding_link = OnboardingLinkInfo {
            legal_entity_id: "LE_ORG_001".into(),
            settings: Some(OnboardingLinkSettings {
                collect_entity_types: Some(vec![LegalEntityType::Individual]),
                enable_manual_review: Some(true),
                required_verification_checks: Some(vec![
                    VerificationCheckType::IdentityVerification,
                    VerificationCheckType::CompanyVerification,
                ]),
            }),
            theme_id: Some("techcorp_theme".into()),
        };

        // Verify organization onboarding workflow
        assert_eq!(organization.legal_name.as_ref(), "TechCorp Solutions LLC");
        assert!(matches!(organization.r#type, Some(OrganizationType::LimitedLiabilityCompany)));
        assert!(us_bank_account.bank_account.is_some());
        assert!(eu_bank_account.bank_account.is_some());
        assert_eq!(software_business_line.industry.as_ref(), "5734");
        assert_eq!(consulting_business_line.industry.as_ref(), "7379");
        assert!(onboarding_link.settings.is_some());
    }

    #[test]
    fn test_trust_entity_workflow_types() {
        // Trust Entity for Complex Structures
        let trust = Trust {
            name: "Johnson Family Trust".into(),
            r#type: Some(TrustType::IrrevocableTrust),
            date_of_settlement: Some("2020-12-01".into()),
            country_of_governance: Some("US".into()),
            principal_place_of_business: Some(Address {
                country: "US".into(),
                city: Some("Miami".into()),
                postal_code: Some("33101".into()),
                state_or_province: Some("FL".into()),
                street_address: Some("200 Biscayne Blvd".into()),
                street_address2: None,
            }),
            tax_information: Some(vec![TaxInformation {
                country: "US".into(),
                number: Some("98-7654321".into()),
                r#type: Some(TaxIdType::UsEin),
                tax_reporting_classification: Some(TaxReportingClassification {
                    business_type: Some(TaxBusinessType::Trust),
                    commercial_type: Some(TaxCommercialType::NonFinancialEntity),
                    financial_institution_number: None,
                }),
            }]),
            source_of_funds: Some(SourceOfFunds {
                r#type: FundsSourceType::Inheritance,
                description: Some("Family inheritance and real estate investments".into()),
            }),
            undefined_beneficiary: Some(UndefinedBeneficiary {
                description: Some("Future grandchildren of the Johnson family".into()),
            }),
        };

        // Entity Associations for Trust Structure
        let trustee_association = LegalEntityAssociation {
            associator_id: "LE_TRUSTEE_001".into(),
            r#type: AssociationType::TrusteeBeneficiary,
            name: Some("Primary Trustee".into()),
            job_title: Some("Trust Administrator".into()),
            entity_ids: Some(vec!["LE_TRUST_001".into()]),
        };

        let beneficiary_association = LegalEntityAssociation {
            associator_id: "LE_BENEFICIARY_001".into(),
            r#type: AssociationType::BeneficialOwner,
            name: Some("Trust Beneficiary".into()),
            job_title: None,
            entity_ids: Some(vec!["LE_TRUST_001".into()]),
        };

        // Verify trust workflow
        assert_eq!(trust.name.as_ref(), "Johnson Family Trust");
        assert!(matches!(trust.r#type, Some(TrustType::IrrevocableTrust)));
        assert!(matches!(trust.source_of_funds.as_ref().unwrap().r#type, FundsSourceType::Inheritance));
        assert!(matches!(trustee_association.r#type, AssociationType::TrusteeBeneficiary));
        assert!(matches!(beneficiary_association.r#type, AssociationType::BeneficialOwner));
    }
}

#[cfg(test)]
mod api_tests {
    use super::*;

    #[test]
    fn test_legal_entity_api_creation() {
        let config = create_test_config();
        let _api = LegalEntityApi::new(config).unwrap();
        // API created successfully indicates proper configuration
    }
}