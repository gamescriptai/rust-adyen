//! Adyen Balance Platform API v2 for marketplace operations.
//!
//! This crate provides comprehensive access to Adyen's Balance Platform API v2,
//! enabling marketplace platforms to manage balance accounts, payment instruments,
//! and transaction rules for their sub-merchants.
//!
//! # Features
//!
//! - **Balance Account Management**: Create and manage balance accounts that hold funds
//! - **Account Holder Management**: Manage account holders linked to legal entities
//! - **Payment Instruments**: Create cards and bank accounts for payment processing
//! - **Transaction Rules**: Configure rules to control transaction processing
//! - **Comprehensive Type Safety**: Full type definitions for all API endpoints
//! - **Builder Patterns**: Ergonomic request builders with validation
//!
//! # Quick Start
//!
//! ```rust
//! use adyen_core::{ConfigBuilder, Environment};
//! use adyen_platform::{BalancePlatformApi, CreateBalanceAccountRequest};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let config = ConfigBuilder::new()
//!     .environment(Environment::test())
//!     .api_key("your_api_key")?
//!     .build()?;
//!
//! let platform = BalancePlatformApi::new(config)?;
//!
//! let request = CreateBalanceAccountRequest::builder()
//!     .account_holder_id("AH12345")
//!     .description("Main balance account")
//!     .default_currency_code("EUR")
//!     .build()
//!     .map_err(|e| format!("Builder error: {}", e))?;
//!
//! let balance_account = platform.create_balance_account(&request).await?;
//! # Ok(())
//! # }
//! ```

pub mod api;
pub mod types;

pub use api::BalancePlatformApi;
pub use types::*;
