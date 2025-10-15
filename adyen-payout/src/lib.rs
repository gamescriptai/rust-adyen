//! # Adyen Payout API v68
//!
//! This crate provides access to Adyen's Payout API v68 for fund disbursement and payout management.
//! It includes support for instant payouts, batch payouts, payout reviews, and comprehensive
//! payout status tracking.

#![allow(clippy::type_complexity)]
//!
//! ## Features
//!
//! - **Instant Payouts**: Submit and confirm immediate fund transfers
//! - **Batch Processing**: Handle multiple payouts efficiently
//! - **Review System**: Approve or decline payouts requiring manual review
//! - **Status Tracking**: Monitor payout status and handle notifications
//! - **Type Safety**: Full Rust type safety with builder patterns
//! - **Modern Patterns**: Async/await support with comprehensive error handling
//!
//! ## Quick Start
//!
//! ```rust
//! use adyen_core::{ConfigBuilder, Environment};
//! use adyen_payout::PayoutApi;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let config = ConfigBuilder::new()
//!     .environment(Environment::test())
//!     .api_key("your_api_key")?
//!     .build()?;
//!
//! let payout = PayoutApi::new(config)?;
//! # Ok(())
//! # }
//! ```

pub mod api;
pub mod types;

// Re-export main types for convenience
pub use api::PayoutApi;
pub use types::{
    Address, BankAccount, BankAccountType, Card, ConfirmRequest, ConfirmResponse,
    DeclinePayoutRequest, EntityType, Name, PayoutMethodDetails, PayoutResponse,
    ReviewPayoutRequest, SubmitRequest, SubmitResponse,
};
