//! # Adyen Management API v3
//!
//! This crate provides access to Adyen's Management API v3 for configuring and managing
//! your Adyen company and merchant accounts, stores, and payment terminals.

#![allow(clippy::type_complexity)]
//!
//! ## Features
//!
//! - **Company Management**: Manage your Adyen company account and settings
//! - **Merchant Accounts**: Create and manage merchant accounts
//! - **Store Management**: Add and configure stores under merchant accounts
//! - **Payment Methods**: Configure payment methods for stores and accounts
//! - **Terminal Management**: Manage payment terminals and their assignments
//! - **Webhook Configuration**: Set up and manage webhook endpoints
//! - **User Management**: Manage users and their permissions
//! - **Type Safety**: Full Rust type safety with builder patterns
//! - **Modern Patterns**: Async/await support with comprehensive error handling
//!
//! ## Quick Start
//!
//! ```rust
//! use adyen_core::{ConfigBuilder, Environment};
//! use adyen_management::ManagementApi;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let config = ConfigBuilder::new()
//!     .environment(Environment::test())
//!     .api_key("your_api_key")?
//!     .build()?;
//!
//! let management = ManagementApi::new(config)?;
//! # Ok(())
//! # }
//! ```

pub mod api;
pub mod types;

// Re-export main types for convenience
pub use api::ManagementApi;
pub use types::{
    // Common types
    Address,
    // Company and Merchant types
    Company,
    Contact,
    CreateMerchantRequest,
    CreateStoreRequest,
    CreateWebhookRequest,
    Links,
    MerchantAccount,
    // Payment methods
    PaymentMethod,
    PaymentMethodSettings,
    // Store management
    Store,
    StoreCreationWithMerchantCodeRequest,
    Terminal,
    // Terminal management
    TerminalModel,
    TerminalSettings,
    UpdatePaymentMethodRequest,
    UpdateWebhookRequest,
    // Webhooks
    Webhook,
};
