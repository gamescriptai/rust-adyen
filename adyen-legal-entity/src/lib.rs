//! Adyen Legal Entity API v3 for KYC and onboarding workflows.
//!
//! This crate provides comprehensive access to Adyen's Legal Entity API v3,
//! enabling platforms to manage legal entities, conduct KYC verification,
//! and handle compliance workflows.
//!
//! # Features
//!
//! - **Legal Entity Management**: Create and manage individuals, organizations, trusts, etc.
//! - **KYC Verification**: Document upload and verification workflows
//! - **Business Lines**: Define operational details and industry classifications
//! - **Transfer Instruments**: Configure bank accounts for fund transfers
//! - **Hosted Onboarding**: Self-service onboarding links with customization
//! - **PCI Compliance**: PCI DSS questionnaire management
//! - **Terms of Service**: Legal agreement acceptance workflows
//! - **Tax Compliance**: Electronic delivery consent for tax documents
//! - **Comprehensive Type Safety**: Full type definitions for all 91 models
//! - **Builder Patterns**: Ergonomic request builders with validation
//!
//! # Quick Start
//!
//! ```rust
//! use adyen_core::{ConfigBuilder, Environment};
//! use adyen_legal_entity::{LegalEntityApi, LegalEntityInfo, LegalEntityType, Individual, Name};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let config = ConfigBuilder::new()
//!     .environment(Environment::test())
//!     .api_key("your_api_key")?
//!     .build()?;
//!
//! let api = LegalEntityApi::new(config)?;
//!
//! let individual = Individual {
//!     name: Name {
//!         first_name: "John".into(),
//!         last_name: "Doe".into(),
//!         in_fix: None,
//!     },
//!     email: Some("john.doe@example.com".into()),
//!     phone: None,
//!     birth_data: None,
//!     nationality: Some("US".into()),
//!     identification_data: None,
//!     residential_address: None,
//!     tax_information: None,
//! };
//!
//! let request = LegalEntityInfo::builder()
//!     .entity_type(LegalEntityType::Individual)
//!     .individual(individual)
//!     .reference("individual_001")
//!     .build()
//!     .map_err(|e| format!("Builder error: {}", e))?;
//!
//! let legal_entity = api.create_legal_entity(&request).await?;
//! # Ok(())
//! # }
//! ```

pub mod api;
pub mod types;

pub use api::LegalEntityApi;
pub use types::*;
