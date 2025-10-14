//! Classic Payments API client implementation.

use adyen_core::{Client, Config, Result};
use crate::types::*;

/// Adyen Classic Payments API client.
///
/// Provides access to Adyen's Classic Payments API v68 for traditional payment processing,
/// including authorization, 3D Secure, and payment modifications.
///
/// # Example
///
/// ```rust
/// use adyen_core::{ConfigBuilder, Environment};
/// use adyen_payments::PaymentsApi;
///
/// # fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let config = ConfigBuilder::new()
///     .environment(Environment::test())
///     .api_key("your_api_key")?
///     .build()?;
///
/// let payments = PaymentsApi::new(config)?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct PaymentsApi {
    client: Client,
}

impl PaymentsApi {
    /// Create a new Classic Payments API client with the given configuration.
    ///
    /// # Errors
    ///
    /// Returns an error if the underlying HTTP client cannot be created.
    pub fn new(config: Config) -> Result<Self> {
        let client = Client::new(config)?;
        Ok(Self { client })
    }

    /// Create an authorization for a payment.
    ///
    /// Creates a payment with a unique reference (pspReference) and attempts to obtain
    /// an authorization hold. For cards, this amount can be captured or cancelled later.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    ///
    /// # Example
    ///
    /// ```rust
    /// use adyen_core::{Amount, Currency};
    /// use adyen_payments::{PaymentsApi, PaymentRequest, Card};
    ///
    /// # async fn example(payments: PaymentsApi) -> Result<(), Box<dyn std::error::Error>> {
    /// let request = PaymentRequest::builder()
    ///     .amount(Amount::from_major_units(100, Currency::EUR))
    ///     .merchant_account("YourMerchantAccount")
    ///     .reference("Order-12345")
    ///     .card(Card::new("4111111111111111", "12", "2025", "123"))
    ///     .build()?;
    ///
    /// let response = payments.authorise(&request).await?;
    /// println!("Authorization result: {:?}", response.result_code);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn authorise(&self, request: &PaymentRequest) -> Result<PaymentResult> {
        let url = format!("{}/pal/servlet/Payment/v68/authorise", self.client.config().environment().classic_api_url());
        let response = self.client.post(&url, request).await?;
        Ok(response.data)
    }

    /// Handle 3D Secure 1.0 authentication.
    ///
    /// Used to complete payments that require 3D Secure 1.0 authentication.
    /// This endpoint processes the authentication result from the issuer.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn authorise_3d(&self, request: &PaymentRequest3d) -> Result<PaymentResult> {
        let url = format!("{}/pal/servlet/Payment/v68/authorise3d", self.client.config().environment().classic_api_url());
        let response = self.client.post(&url, request).await?;
        Ok(response.data)
    }

    /// Handle 3D Secure 2.0 authentication.
    ///
    /// Used to complete payments that require 3D Secure 2.0 authentication.
    /// This endpoint processes the challenge result from the issuer.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn authorise_3ds2(&self, request: &PaymentRequest3ds2) -> Result<PaymentResult> {
        let url = format!("{}/pal/servlet/Payment/v68/authorise3ds2", self.client.config().environment().classic_api_url());
        let response = self.client.post(&url, request).await?;
        Ok(response.data)
    }
}

/// Adyen Payment Modifications API client.
///
/// Provides access to payment modification operations including capture,
/// cancel, refund, and authorization adjustments.
#[derive(Debug, Clone)]
pub struct ModificationsApi {
    client: Client,
}

impl ModificationsApi {
    /// Create a new Payment Modifications API client with the given configuration.
    ///
    /// # Errors
    ///
    /// Returns an error if the underlying HTTP client cannot be created.
    pub fn new(config: Config) -> Result<Self> {
        let client = Client::new(config)?;
        Ok(Self { client })
    }

    /// Capture an authorized payment.
    ///
    /// Captures the authorized amount (or a lower amount) for a payment.
    /// The captured amount will be transferred to your account.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    ///
    /// # Example
    ///
    /// ```rust
    /// use adyen_core::Amount;
    /// use adyen_payments::{ModificationsApi, CaptureRequest};
    ///
    /// # async fn example(modifications: ModificationsApi) -> Result<(), Box<dyn std::error::Error>> {
    /// let request = CaptureRequest::builder()
    ///     .merchant_account("YourMerchantAccount")
    ///     .original_reference("8515131751004933")
    ///     .modification_amount(Amount::from_major_units(75, adyen_core::Currency::EUR))
    ///     .build().unwrap();
    ///
    /// let response = modifications.capture(&request).await?;
    /// println!("Capture result: {:?}", response.response);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn capture(&self, request: &CaptureRequest) -> Result<ModificationResult> {
        let url = format!("{}/pal/servlet/Payment/v68/capture", self.client.config().environment().classic_api_url());
        let response = self.client.post(&url, request).await?;
        Ok(response.data)
    }

    /// Cancel an authorized payment.
    ///
    /// Cancels the authorization hold on a payment. The authorized amount
    /// will be released back to the shopper.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn cancel(&self, request: &CancelRequest) -> Result<ModificationResult> {
        let url = format!("{}/pal/servlet/Payment/v68/cancel", self.client.config().environment().classic_api_url());
        let response = self.client.post(&url, request).await?;
        Ok(response.data)
    }

    /// Refund a captured payment.
    ///
    /// Refunds the captured amount (or a lower amount) for a payment.
    /// The refunded amount will be returned to the shopper's account.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn refund(&self, request: &RefundRequest) -> Result<ModificationResult> {
        let url = format!("{}/pal/servlet/Payment/v68/refund", self.client.config().environment().classic_api_url());
        let response = self.client.post(&url, request).await?;
        Ok(response.data)
    }

    /// Cancel or refund a payment automatically.
    ///
    /// Automatically determines whether to cancel (if not captured) or refund (if captured)
    /// a payment. This is useful when you're not sure of the payment's current state.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn cancel_or_refund(&self, request: &CancelOrRefundRequest) -> Result<ModificationResult> {
        let url = format!("{}/pal/servlet/Payment/v68/cancelOrRefund", self.client.config().environment().classic_api_url());
        let response = self.client.post(&url, request).await?;
        Ok(response.data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use adyen_core::{ConfigBuilder, Environment};

    #[test]
    fn test_payments_api_creation() {
        let config = ConfigBuilder::new()
            .environment(Environment::test())
            .api_key("test_key_1234567890123456")
            .unwrap()
            .build()
            .unwrap();

        let api = PaymentsApi::new(config).unwrap();
        assert!(api.client.config().environment().is_test());
    }

    #[test]
    fn test_modifications_api_creation() {
        let config = ConfigBuilder::new()
            .environment(Environment::test())
            .api_key("test_key_1234567890123456")
            .unwrap()
            .build()
            .unwrap();

        let api = ModificationsApi::new(config).unwrap();
        assert!(api.client.config().environment().is_test());
    }
}