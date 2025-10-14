//! # Adyen Classic Payments API
//!
//! This crate provides access to Adyen's Classic Payments API (v68) for traditional
//! payment processing, including authorization, capture, refund, and cancellation.
//!
//! ## Features
//!
//! - **Payment Authorization**: Create authorizations for card and alternative payments
//! - **3D Secure Support**: Handle 3D Secure 1.0 and 2.0 authentication flows
//! - **Payment Modifications**: Capture, cancel, refund, and adjust authorizations
//! - **Fraud Detection**: Comprehensive fraud scoring and risk management
//! - **Recurring Payments**: Support for stored payment methods and subscriptions
//!
//! ## Example
//!
//! ```rust
//! use adyen_core::{Amount, Currency, Environment, ConfigBuilder};
//! use adyen_payments::{PaymentsApi, PaymentRequest, Card};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let config = ConfigBuilder::new()
//!     .environment(Environment::test())
//!     .api_key("your_api_key")
//!     .unwrap()
//!     .build()?;
//!
//! let payments = PaymentsApi::new(config)?;
//!
//! let request = PaymentRequest::builder()
//!     .amount(Amount::from_major_units(100, Currency::EUR))
//!     .merchant_account("YourMerchantAccount")
//!     .reference("Order-12345")
//!     .card(Card::new("4111111111111111", "12", "2025", "123"))
//!     .build()?;
//!
//! let response = payments.authorise(&request).await?;
//! println!("Payment result: {:?}", response.result_code);
//! # Ok(())
//! # }
//! ```

#![deny(missing_docs)]
#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

pub mod api;
pub mod types;

// Re-export main API and commonly used types
pub use api::{PaymentsApi, ModificationsApi};
pub use types::{
    PaymentRequest, PaymentResult,
    PaymentRequest3d, PaymentRequest3ds2,
    CaptureRequest, CancelRequest, RefundRequest, CancelOrRefundRequest,
    ModificationResult, Card, PaymentResultCode,
};
