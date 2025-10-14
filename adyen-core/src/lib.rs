//! # Adyen Core Library
//!
//! Core types, utilities, and abstractions for the Adyen Rust library.
//!
//! This crate provides the foundational components used across all Adyen API modules:
//! - Common data types (Amount, Currency, etc.)
//! - HTTP client abstractions
//! - Authentication mechanisms
//! - Error handling types
//! - Configuration management
//!
//! ## Features
//!
//! - `serde` (default): Enable serde serialization support
//! - `rkyv`: Enable zero-copy rkyv serialization support
//! - `observability`: Enable tracing and metrics collection
//!
//! ## Example
//!
//! ```rust
//! use adyen_core::{Amount, Currency, Environment, ConfigBuilder};
//!
//! // Create an amount
//! let amount = Amount::from_major_units(100, Currency::EUR);
//!
//! // Configure the client
//! let config = ConfigBuilder::new()
//!     .environment(Environment::test())
//!     .api_key("your_api_key")
//!     .unwrap()
//!     .build()
//!     .expect("valid configuration");
//! ```

#![deny(missing_docs)]
#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

pub mod auth;
pub mod client;
pub mod config;
pub mod currency;
pub mod environment;
pub mod error;
pub mod http;
pub mod types;

// Re-export commonly used types
pub use auth::{ApiKey, BasicAuth, Credentials};
pub use client::{ApiResponse, Client, Request};
pub use config::{Config, ConfigBuilder};
pub use currency::Currency;
pub use environment::Environment;
pub use error::{AdyenError, Result};
pub use types::{Amount, RequestId};

/// Current version of the Adyen Core library
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// User agent string for HTTP requests
pub const USER_AGENT: &str = concat!("adyen-rust/", env!("CARGO_PKG_VERSION"));