//! # Adyen Recurring API v68
//!
//! This crate provides access to Adyen's Recurring API v68 for managing stored payment methods
//! and recurring payments. It includes support for listing, disabling, and managing stored
//! payment methods, as well as shopper notifications and account updater services.
//!
//! ## Features
//!
//! - **Stored Payment Methods**: List and manage stored payment methods
//! - **Payment Method Management**: Disable stored payment methods
//! - **Shopper Notifications**: Send notifications about stored payment methods
//! - **Account Updater**: Schedule automatic card updates
//! - **Type Safety**: Full Rust type safety with builder patterns
//! - **Modern Patterns**: Async/await support with comprehensive error handling
//!
//! ## Quick Start
//!
//! ```rust
//! use adyen_core::{ConfigBuilder, Environment};
//! use adyen_recurring::RecurringApi;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let config = ConfigBuilder::new()
//!     .environment(Environment::test())
//!     .api_key("your_api_key")?
//!     .build()?;
//!
//! let recurring = RecurringApi::new(config)?;
//! # Ok(())
//! # }
//! ```

pub mod api;
pub mod types;

// Re-export main types for convenience
pub use api::RecurringApi;
pub use types::{
    RecurringDetailsRequest, RecurringDetailsResult, RecurringDetail,
    DisableRequest, DisableResult, NotifyShopperRequest, NotifyShopperResult,
    ScheduleAccountUpdaterRequest, ScheduleAccountUpdaterResult,
    Recurring, RecurringContract,
};