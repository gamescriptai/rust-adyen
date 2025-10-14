//! Common types used across the Adyen library.

use crate::{Currency, AdyenError, Result};
use rust_decimal::{Decimal, prelude::ToPrimitive};
use std::fmt;

/// Represents a monetary amount with currency.
///
/// This type stores amounts in minor units (e.g., cents for USD/EUR) to avoid
/// floating-point precision issues. All operations maintain precision and
/// currency safety.
///
/// # Examples
///
/// ```rust
/// use adyen_core::{Amount, Currency};
/// use rust_decimal::Decimal;
///
/// // Create from major units (e.g., dollars)
/// let amount = Amount::from_major_units(100, Currency::USD);
/// assert_eq!(amount.minor_units(), 10000); // 100 dollars = 10000 cents
///
/// // Create from minor units (e.g., cents)
/// let amount = Amount::from_minor_units(10000, Currency::USD);
/// assert_eq!(amount.major_units(), Decimal::from(100));
///
/// // Create from decimal
/// let amount = Amount::new(Decimal::new(10050, 2), Currency::USD).unwrap();
/// assert_eq!(amount.minor_units(), 10050); // $100.50
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize))]
pub struct Amount {
    /// Amount in minor units (e.g., cents)
    minor_units: u64,
    /// Currency of the amount
    currency: Currency,
}

impl Amount {
    /// Create a new amount from a decimal value and currency.
    ///
    /// The decimal should represent the amount in major units (e.g., dollars).
    ///
    /// # Errors
    ///
    /// Returns an error if the decimal cannot be converted to minor units
    /// (e.g., too many decimal places or negative value).
    pub fn new(amount: Decimal, currency: Currency) -> Result<Self> {
        let multiplier = Decimal::from(currency.minor_unit_multiplier());
        let minor_units = amount * multiplier;

        if minor_units.is_sign_negative() {
            return Err(AdyenError::config("Amount cannot be negative"));
        }

        let minor_units = minor_units.to_u64()
            .ok_or_else(|| AdyenError::config("Amount too large or has too many decimal places"))?;

        Ok(Self {
            minor_units,
            currency,
        })
    }

    /// Create an amount from major units (e.g., dollars).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use adyen_core::{Amount, Currency};
    ///
    /// let amount = Amount::from_major_units(100, Currency::USD);
    /// assert_eq!(amount.minor_units(), 10000); // 100 dollars = 10000 cents
    /// ```
    #[must_use]
    pub fn from_major_units(major_units: u64, currency: Currency) -> Self {
        Self {
            minor_units: major_units * currency.minor_unit_multiplier(),
            currency,
        }
    }

    /// Create an amount from minor units (e.g., cents).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use adyen_core::{Amount, Currency};
    ///
    /// let amount = Amount::from_minor_units(10050, Currency::USD);
    /// // This represents $100.50
    /// ```
    #[must_use]
    pub const fn from_minor_units(minor_units: u64, currency: Currency) -> Self {
        Self {
            minor_units,
            currency,
        }
    }

    /// Get the amount in minor units.
    #[must_use]
    pub const fn minor_units(&self) -> u64 {
        self.minor_units
    }

    /// Get the currency.
    #[must_use]
    pub const fn currency(&self) -> Currency {
        self.currency
    }

    /// Get the amount in major units as a decimal.
    #[must_use]
    pub fn major_units(&self) -> Decimal {
        let divisor = Decimal::from(self.currency.minor_unit_multiplier());
        Decimal::from(self.minor_units) / divisor
    }

    /// Check if this amount is zero.
    #[must_use]
    pub const fn is_zero(&self) -> bool {
        self.minor_units == 0
    }

    /// Add another amount to this one.
    ///
    /// # Errors
    ///
    /// Returns an error if the currencies don't match or if the result overflows.
    pub fn add(&self, other: &Self) -> Result<Self> {
        if self.currency != other.currency {
            return Err(AdyenError::config(format!(
                "Cannot add amounts with different currencies: {} and {}",
                self.currency, other.currency
            )));
        }

        let result = self.minor_units.checked_add(other.minor_units)
            .ok_or_else(|| AdyenError::config("Amount addition overflow"))?;

        Ok(Self {
            minor_units: result,
            currency: self.currency,
        })
    }

    /// Subtract another amount from this one.
    ///
    /// # Errors
    ///
    /// Returns an error if the currencies don't match or if the result would be negative.
    pub fn subtract(&self, other: &Self) -> Result<Self> {
        if self.currency != other.currency {
            return Err(AdyenError::config(format!(
                "Cannot subtract amounts with different currencies: {} and {}",
                self.currency, other.currency
            )));
        }

        let result = self.minor_units.checked_sub(other.minor_units)
            .ok_or_else(|| AdyenError::config("Amount subtraction underflow"))?;

        Ok(Self {
            minor_units: result,
            currency: self.currency,
        })
    }
}

impl fmt::Display for Amount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.major_units(), self.currency)
    }
}

/// A unique identifier for requests to ensure idempotency.
///
/// Request IDs are used to prevent duplicate processing of requests.
/// They should be unique for each request and can be used to retry
/// failed requests safely.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize))]
pub struct RequestId(compact_str::CompactString);

impl RequestId {
    /// Create a new random request ID.
    #[must_use]
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4().to_string().into())
    }

    /// Create a request ID from a string.
    ///
    /// # Errors
    ///
    /// Returns an error if the string is empty or too long.
    pub fn from_string(id: impl Into<String>) -> Result<Self> {
        let id = id.into();
        if id.is_empty() {
            return Err(AdyenError::config("Request ID cannot be empty"));
        }
        if id.len() > 64 {
            return Err(AdyenError::config("Request ID cannot be longer than 64 characters"));
        }
        Ok(Self(id.into()))
    }

    /// Get the request ID as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for RequestId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for RequestId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<uuid::Uuid> for RequestId {
    fn from(uuid: uuid::Uuid) -> Self {
        Self(uuid.to_string().into())
    }
}

impl std::str::FromStr for RequestId {
    type Err = AdyenError;

    fn from_str(s: &str) -> Result<Self> {
        Self::from_string(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::Decimal;

    #[test]
    fn test_amount_from_major_units() {
        let amount = Amount::from_major_units(100, Currency::USD);
        assert_eq!(amount.minor_units(), 10000);
        assert_eq!(amount.currency(), Currency::USD);
        assert_eq!(amount.major_units(), Decimal::from(100));
    }

    #[test]
    fn test_amount_from_minor_units() {
        let amount = Amount::from_minor_units(10050, Currency::USD);
        assert_eq!(amount.minor_units(), 10050);
        assert_eq!(amount.major_units(), Decimal::new(10050, 2));
    }

    #[test]
    fn test_amount_new() {
        let amount = Amount::new(Decimal::new(10050, 2), Currency::USD).unwrap();
        assert_eq!(amount.minor_units(), 10050);
    }

    #[test]
    fn test_amount_zero_decimal_currency() {
        let amount = Amount::from_major_units(100, Currency::JPY);
        assert_eq!(amount.minor_units(), 100);
        assert_eq!(amount.major_units(), Decimal::from(100));
    }

    #[test]
    fn test_amount_add() {
        let amount1 = Amount::from_minor_units(1000, Currency::USD);
        let amount2 = Amount::from_minor_units(500, Currency::USD);
        let result = amount1.add(&amount2).unwrap();
        assert_eq!(result.minor_units(), 1500);
    }

    #[test]
    fn test_amount_add_different_currencies() {
        let amount1 = Amount::from_minor_units(1000, Currency::USD);
        let amount2 = Amount::from_minor_units(500, Currency::EUR);
        assert!(amount1.add(&amount2).is_err());
    }

    #[test]
    fn test_amount_subtract() {
        let amount1 = Amount::from_minor_units(1000, Currency::USD);
        let amount2 = Amount::from_minor_units(500, Currency::USD);
        let result = amount1.subtract(&amount2).unwrap();
        assert_eq!(result.minor_units(), 500);
    }

    #[test]
    fn test_amount_subtract_underflow() {
        let amount1 = Amount::from_minor_units(500, Currency::USD);
        let amount2 = Amount::from_minor_units(1000, Currency::USD);
        assert!(amount1.subtract(&amount2).is_err());
    }

    #[test]
    fn test_amount_is_zero() {
        let amount = Amount::from_minor_units(0, Currency::USD);
        assert!(amount.is_zero());

        let amount = Amount::from_minor_units(1, Currency::USD);
        assert!(!amount.is_zero());
    }

    #[test]
    fn test_request_id_new() {
        let id1 = RequestId::new();
        let id2 = RequestId::new();
        assert_ne!(id1, id2);
        assert!(!id1.as_str().is_empty());
    }

    #[test]
    fn test_request_id_from_string() {
        let id = RequestId::from_string("test-id").unwrap();
        assert_eq!(id.as_str(), "test-id");
    }

    #[test]
    fn test_request_id_from_empty_string() {
        assert!(RequestId::from_string("").is_err());
    }

    #[test]
    fn test_request_id_from_long_string() {
        let long_string = "a".repeat(65);
        assert!(RequestId::from_string(long_string).is_err());
    }

    #[test]
    fn test_request_id_from_uuid() {
        let uuid = uuid::Uuid::new_v4();
        let id = RequestId::from(uuid);
        assert_eq!(id.as_str(), uuid.to_string());
    }
}