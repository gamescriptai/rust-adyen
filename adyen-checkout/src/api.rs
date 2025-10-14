//! Checkout API client implementation.

use adyen_core::{Client, Config, Result};
use crate::types::*;

/// Adyen Checkout API client.
///
/// Provides access to Adyen's Checkout API v71 for payment processing,
/// including payment methods, sessions, and payment transactions.
///
/// # Example
///
/// ```rust
/// use adyen_core::{ConfigBuilder, Environment};
/// use adyen_checkout::CheckoutApi;
///
/// # fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let config = ConfigBuilder::new()
///     .environment(Environment::test())
///     .api_key("your_api_key")?
///     .build()?;
///
/// let checkout = CheckoutApi::new(config);
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct CheckoutApi {
    client: Client,
}

impl CheckoutApi {
    /// Create a new Checkout API client with the given configuration.
    ///
    /// # Errors
    ///
    /// Returns an error if the underlying HTTP client cannot be created.
    pub fn new(config: Config) -> Result<Self> {
        let client = Client::new(config)?;
        Ok(Self { client })
    }

    /// Get a list of available payment methods.
    ///
    /// Retrieves the payment methods available for the given merchant account,
    /// country, currency, and amount.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    ///
    /// # Example
    ///
    /// ```rust
    /// use adyen_core::{Amount, Currency};
    /// use adyen_checkout::{CheckoutApi, PaymentMethodsRequest};
    ///
    /// # async fn example(checkout: CheckoutApi) -> Result<(), Box<dyn std::error::Error>> {
    /// let request = PaymentMethodsRequest::builder()
    ///     .merchant_account("YourMerchantAccount")
    ///     .amount(Amount::from_major_units(100, Currency::EUR))
    ///     .country_code("NL")
    ///     .build()?;
    ///
    /// let response = checkout.payment_methods(&request).await?;
    /// println!("Found {} payment methods", response.payment_methods.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn payment_methods(&self, request: &PaymentMethodsRequest) -> Result<PaymentMethodsResponse> {
        let url = format!("{}/v71/paymentMethods", self.client.config().environment().checkout_api_url());
        let response = self.client.post(&url, request).await?;
        Ok(response.data)
    }

    /// Start a payment transaction.
    ///
    /// Initiates a payment with the specified payment method and amount.
    /// This may return an action that requires additional steps from the shopper.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    ///
    /// # Example
    ///
    /// ```rust
    /// use adyen_core::{Amount, Currency};
    /// use adyen_checkout::{CheckoutApi, PaymentRequest};
    ///
    /// # async fn example(checkout: CheckoutApi) -> Result<(), Box<dyn std::error::Error>> {
    /// let request = PaymentRequest::builder()
    ///     .amount(Amount::from_major_units(100, Currency::EUR))
    ///     .merchant_account("YourMerchantAccount")
    ///     .reference("Order-12345")
    ///     .return_url("https://your-company.com/return")
    ///     .build()?;
    ///
    /// let response = checkout.payments(&request).await?;
    /// println!("Payment result: {:?}", response.result_code);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn payments(&self, request: &PaymentRequest) -> Result<PaymentResponse> {
        let url = format!("{}/v71/payments", self.client.config().environment().checkout_api_url());
        let response = self.client.post(&url, request).await?;
        Ok(response.data)
    }

    /// Submit additional details for a payment.
    ///
    /// Used to provide additional authentication data (like 3D Secure results)
    /// or other required details to complete a payment.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn payment_details(&self, request: &PaymentDetailsRequest) -> Result<PaymentDetailsResponse> {
        let url = format!("{}/v71/payments/details", self.client.config().environment().checkout_api_url());
        let response = self.client.post(&url, request).await?;
        Ok(response.data)
    }

    /// Create a checkout session.
    ///
    /// Creates a session that can be used with Adyen's Drop-in or Components
    /// to handle the complete payment flow.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn sessions(&self, request: &CreateCheckoutSessionRequest) -> Result<CreateCheckoutSessionResponse> {
        let url = format!("{}/v71/sessions", self.client.config().environment().checkout_api_url());
        let response = self.client.post(&url, request).await?;
        Ok(response.data)
    }

    /// Get card details including brand and validation information.
    ///
    /// Provides information about a card based on its number, including
    /// the brand, supported features, and validation status.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn card_details(&self, request: &CardDetailsRequest) -> Result<CardDetailsResponse> {
        let url = format!("{}/v71/cardDetails", self.client.config().environment().checkout_api_url());
        let response = self.client.post(&url, request).await?;
        Ok(response.data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use adyen_core::{ConfigBuilder, Environment};

    #[test]
    fn test_checkout_api_creation() {
        let config = ConfigBuilder::new()
            .environment(Environment::test())
            .api_key("test_key_1234567890123456")
            .unwrap()
            .build()
            .unwrap();

        let api = CheckoutApi::new(config).unwrap();
        assert!(api.client.config().environment().is_test());
    }
}