//! # Adyen Checkout API
//!
//! This crate provides access to Adyen's Checkout API (v71) for payment processing,
//! including payment methods, sessions, and payment transactions.
//!
//! ## Features
//!
//! - **Payment Methods**: Retrieve available payment methods for a merchant
//! - **Payment Sessions**: Create and manage checkout sessions
//! - **Payments**: Process payment transactions
//! - **Payment Details**: Submit additional details for payments (3DS, etc.)
//! - **Card Details**: Get card brand and validation information
//!
//! ## Example
//!
//! ```rust
//! use adyen_core::{Amount, Currency, Environment, ConfigBuilder};
//! use adyen_checkout::{CheckoutApi, PaymentMethodsRequest};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let config = ConfigBuilder::new()
//!     .environment(Environment::test())
//!     .api_key("your_api_key")
//!     .unwrap()
//!     .build()?;
//!
//! let checkout = CheckoutApi::new(config)?;
//!
//! let request = PaymentMethodsRequest::builder()
//!     .merchant_account("YourMerchantAccount")
//!     .amount(Amount::from_major_units(100, Currency::EUR))
//!     .build()?;
//!
//! let response = checkout.payment_methods(&request).await?;
//! println!("Available payment methods: {}", response.payment_methods.len());
//! # Ok(())
//! # }
//! ```

#![deny(missing_docs)]
#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

pub mod api;
pub mod types;

// Re-export main API and commonly used types
pub use api::CheckoutApi;
pub use types::{
    PaymentMethodsRequest, PaymentMethodsResponse,
    PaymentRequest, PaymentResponse,
    PaymentDetailsRequest, PaymentDetailsResponse,
    CreateCheckoutSessionRequest, CreateCheckoutSessionResponse,
    CardDetailsRequest, CardDetailsResponse,
};
