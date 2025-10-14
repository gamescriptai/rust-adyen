//! Currency handling for monetary amounts.

use std::fmt;

/// ISO 4217 currency codes supported by Adyen.
///
/// This enum represents the most commonly used currencies in the Adyen platform.
/// For a complete list of supported currencies, refer to the Adyen documentation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
)]
#[repr(u16)]
pub enum Currency {
    /// Euro
    EUR = 978,
    /// US Dollar
    USD = 840,
    /// British Pound Sterling
    GBP = 826,
    /// Japanese Yen
    JPY = 392,
    /// Swiss Franc
    CHF = 756,
    /// Canadian Dollar
    CAD = 124,
    /// Australian Dollar
    AUD = 36,
    /// Norwegian Krone
    NOK = 578,
    /// Swedish Krona
    SEK = 752,
    /// Danish Krone
    DKK = 208,
    /// Polish Zloty
    PLN = 985,
    /// Czech Koruna
    CZK = 203,
    /// Hungarian Forint
    HUF = 348,
    /// Brazilian Real
    BRL = 986,
    /// Mexican Peso
    MXN = 484,
    /// Singapore Dollar
    SGD = 702,
    /// Hong Kong Dollar
    HKD = 344,
    /// New Zealand Dollar
    NZD = 554,
    /// South African Rand
    ZAR = 710,
    /// Chinese Yuan Renminbi
    CNY = 156,
    /// Indian Rupee
    INR = 356,
    /// South Korean Won
    KRW = 410,
    /// Russian Ruble
    RUB = 643,
    /// Turkish Lira
    TRY = 949,
    /// Thai Baht
    THB = 764,
    /// Malaysian Ringgit
    MYR = 458,
    /// Indonesian Rupiah
    IDR = 360,
    /// Philippine Peso
    PHP = 608,
    /// Vietnamese Dong
    VND = 704,
    /// Icelandic Krona
    ISK = 352,
}

impl Currency {
    /// Get the number of decimal places for this currency.
    ///
    /// Most currencies use 2 decimal places, but some exceptions exist:
    /// - JPY, KRW, VND, ISK use 0 decimal places
    /// - Some Middle Eastern currencies use 3 decimal places (not included in this enum)
    #[must_use]
    pub const fn decimal_places(self) -> u8 {
        match self {
            Self::JPY | Self::KRW | Self::VND | Self::ISK => 0,
            _ => 2,
        }
    }

    /// Get the currency code as a string.
    #[must_use]
    pub const fn code(self) -> &'static str {
        match self {
            Self::EUR => "EUR",
            Self::USD => "USD",
            Self::GBP => "GBP",
            Self::JPY => "JPY",
            Self::CHF => "CHF",
            Self::CAD => "CAD",
            Self::AUD => "AUD",
            Self::NOK => "NOK",
            Self::SEK => "SEK",
            Self::DKK => "DKK",
            Self::PLN => "PLN",
            Self::CZK => "CZK",
            Self::HUF => "HUF",
            Self::BRL => "BRL",
            Self::MXN => "MXN",
            Self::SGD => "SGD",
            Self::HKD => "HKD",
            Self::NZD => "NZD",
            Self::ZAR => "ZAR",
            Self::CNY => "CNY",
            Self::INR => "INR",
            Self::KRW => "KRW",
            Self::RUB => "RUB",
            Self::TRY => "TRY",
            Self::THB => "THB",
            Self::MYR => "MYR",
            Self::IDR => "IDR",
            Self::PHP => "PHP",
            Self::VND => "VND",
            Self::ISK => "ISK",
        }
    }

    /// Get the numeric code for this currency (ISO 4217).
    #[must_use]
    pub const fn numeric_code(self) -> u16 {
        self as u16
    }

    /// Parse a currency from its string code.
    ///
    /// # Errors
    ///
    /// Returns an error if the currency code is not recognized.
    pub fn from_code(code: &str) -> Result<Self, crate::error::AdyenError> {
        match code {
            "EUR" => Ok(Self::EUR),
            "USD" => Ok(Self::USD),
            "GBP" => Ok(Self::GBP),
            "JPY" => Ok(Self::JPY),
            "CHF" => Ok(Self::CHF),
            "CAD" => Ok(Self::CAD),
            "AUD" => Ok(Self::AUD),
            "NOK" => Ok(Self::NOK),
            "SEK" => Ok(Self::SEK),
            "DKK" => Ok(Self::DKK),
            "PLN" => Ok(Self::PLN),
            "CZK" => Ok(Self::CZK),
            "HUF" => Ok(Self::HUF),
            "BRL" => Ok(Self::BRL),
            "MXN" => Ok(Self::MXN),
            "SGD" => Ok(Self::SGD),
            "HKD" => Ok(Self::HKD),
            "NZD" => Ok(Self::NZD),
            "ZAR" => Ok(Self::ZAR),
            "CNY" => Ok(Self::CNY),
            "INR" => Ok(Self::INR),
            "KRW" => Ok(Self::KRW),
            "RUB" => Ok(Self::RUB),
            "TRY" => Ok(Self::TRY),
            "THB" => Ok(Self::THB),
            "MYR" => Ok(Self::MYR),
            "IDR" => Ok(Self::IDR),
            "PHP" => Ok(Self::PHP),
            "VND" => Ok(Self::VND),
            "ISK" => Ok(Self::ISK),
            _ => Err(crate::error::AdyenError::config(format!(
                "Unsupported currency code: {code}"
            ))),
        }
    }

    /// Get the minor unit multiplier for this currency.
    ///
    /// This is 10^decimal_places and is used to convert between
    /// major units (e.g., dollars) and minor units (e.g., cents).
    #[must_use]
    pub const fn minor_unit_multiplier(self) -> u64 {
        match self.decimal_places() {
            0 => 1,
            1 => 10,
            2 => 100,
            3 => 1000,
            _ => 1000, // Fallback for any unexpected values
        }
    }
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.code())
    }
}

impl std::str::FromStr for Currency {
    type Err = crate::error::AdyenError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_code(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_currency_decimal_places() {
        assert_eq!(Currency::EUR.decimal_places(), 2);
        assert_eq!(Currency::USD.decimal_places(), 2);
        assert_eq!(Currency::JPY.decimal_places(), 0);
        assert_eq!(Currency::KRW.decimal_places(), 0);
        assert_eq!(Currency::ISK.decimal_places(), 0);
    }

    #[test]
    fn test_currency_code() {
        assert_eq!(Currency::EUR.code(), "EUR");
        assert_eq!(Currency::USD.code(), "USD");
        assert_eq!(Currency::JPY.code(), "JPY");
    }

    #[test]
    fn test_currency_from_code() {
        assert_eq!(Currency::from_code("EUR").unwrap(), Currency::EUR);
        assert_eq!(Currency::from_code("USD").unwrap(), Currency::USD);
        assert!(Currency::from_code("INVALID").is_err());
    }

    #[test]
    fn test_minor_unit_multiplier() {
        assert_eq!(Currency::EUR.minor_unit_multiplier(), 100);
        assert_eq!(Currency::USD.minor_unit_multiplier(), 100);
        assert_eq!(Currency::JPY.minor_unit_multiplier(), 1);
        assert_eq!(Currency::KRW.minor_unit_multiplier(), 1);
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", Currency::EUR), "EUR");
        assert_eq!(format!("{}", Currency::USD), "USD");
    }

    #[test]
    fn test_from_str() {
        assert_eq!("EUR".parse::<Currency>().unwrap(), Currency::EUR);
        assert_eq!("USD".parse::<Currency>().unwrap(), Currency::USD);
        assert!("INVALID".parse::<Currency>().is_err());
    }
}
