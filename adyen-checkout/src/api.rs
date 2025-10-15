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

    /// Get the result of a payment session.
    ///
    /// Retrieves the result of a payment session that was created earlier.
    /// Use this to get the final payment result for sessions.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn get_session_result(&self, session_id: &str, session_result: Option<&str>) -> Result<SessionResultResponse> {
        let mut url = format!("{}/v71/sessions/{}", self.client.config().environment().checkout_api_url(), session_id);
        if let Some(result) = session_result {
            url.push_str(&format!("?sessionResult={}", urlencoding::encode(result)));
        }
        let response = self.client.get(&url).await?;
        Ok(response.data)
    }

    /// Get stored payment methods for a shopper.
    ///
    /// Retrieves the stored payment methods for a given shopper reference.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn get_stored_payment_methods(&self, merchant_account: &str, shopper_reference: &str) -> Result<ListStoredPaymentMethodsResponse> {
        let url = format!("{}/v71/storedPaymentMethods?merchantAccount={}&shopperReference={}",
            self.client.config().environment().checkout_api_url(),
            urlencoding::encode(merchant_account),
            urlencoding::encode(shopper_reference)
        );
        let response = self.client.get(&url).await?;
        Ok(response.data)
    }

    /// Delete a stored payment method.
    ///
    /// Removes a stored payment method for a shopper.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn delete_stored_payment_method(&self, stored_payment_method_id: &str, merchant_account: &str, shopper_reference: &str) -> Result<()> {
        let url = format!("{}/v71/storedPaymentMethods/{}?merchantAccount={}&shopperReference={}",
            self.client.config().environment().checkout_api_url(),
            urlencoding::encode(stored_payment_method_id),
            urlencoding::encode(merchant_account),
            urlencoding::encode(shopper_reference)
        );
        let _response = self.client.delete(&url).await?;
        Ok(())
    }

    /// Check the balance of a payment method.
    ///
    /// Retrieves the balance available on a gift card or other prepaid payment method.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn payment_methods_balance(&self, request: &BalanceCheckRequest) -> Result<BalanceCheckResponse> {
        let url = format!("{}/v71/paymentMethods/balance", self.client.config().environment().checkout_api_url());
        let response = self.client.post(&url, request).await?;
        Ok(response.data)
    }

    /// Create a payment link.
    ///
    /// Creates a payment link that can be sent to shoppers for payment.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn payment_links(&self, request: &PaymentLinkRequest) -> Result<PaymentLinkResponse> {
        let url = format!("{}/v71/paymentLinks", self.client.config().environment().checkout_api_url());
        let response = self.client.post(&url, request).await?;
        Ok(response.data)
    }

    /// Get a payment link by ID.
    ///
    /// Retrieves the details of a payment link.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn get_payment_link(&self, link_id: &str) -> Result<PaymentLinkResponse> {
        let url = format!("{}/v71/paymentLinks/{}",
            self.client.config().environment().checkout_api_url(),
            urlencoding::encode(link_id)
        );
        let response = self.client.get(&url).await?;
        Ok(response.data)
    }

    /// Get origin keys for client-side encryption.
    ///
    /// Generates origin keys for securing payment data on the client side.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn origin_keys(&self, request: &OriginKeysRequest) -> Result<OriginKeysResponse> {
        let url = format!("{}/v71/originKeys", self.client.config().environment().checkout_api_url());
        let response = self.client.post(&url, request).await?;
        Ok(response.data)
    }

    /// Get Apple Pay session.
    ///
    /// Initiates an Apple Pay session for payment processing.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn apple_pay_sessions(&self, request: &ApplePaySessionRequest) -> Result<ApplePaySessionResponse> {
        let url = format!("{}/v71/applePay/sessions", self.client.config().environment().checkout_api_url());
        let response = self.client.post(&url, request).await?;
        Ok(response.data)
    }

    /// Capture a payment.
    ///
    /// Captures an authorized payment for the specified amount.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn capture_payment(&self, payment_psp_reference: &str, request: &CaptureRequest) -> Result<CaptureResponse> {
        let url = format!("{}/v71/payments/{}/captures",
            self.client.config().environment().checkout_api_url(),
            urlencoding::encode(payment_psp_reference)
        );
        let response = self.client.post(&url, request).await?;
        Ok(response.data)
    }

    /// Refund a payment.
    ///
    /// Refunds a captured payment for the specified amount.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn refund_payment(&self, payment_psp_reference: &str, request: &RefundRequest) -> Result<RefundResponse> {
        let url = format!("{}/v71/payments/{}/refunds",
            self.client.config().environment().checkout_api_url(),
            urlencoding::encode(payment_psp_reference)
        );
        let response = self.client.post(&url, request).await?;
        Ok(response.data)
    }

    /// Cancel a payment.
    ///
    /// Cancels an authorized payment.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn cancel_payment(&self, payment_psp_reference: &str, request: &CancelRequest) -> Result<CancelResponse> {
        let url = format!("{}/v71/payments/{}/cancels",
            self.client.config().environment().checkout_api_url(),
            urlencoding::encode(payment_psp_reference)
        );
        let response = self.client.post(&url, request).await?;
        Ok(response.data)
    }

    /// Reverse a payment.
    ///
    /// Reverses an authorized payment.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn reverse_payment(&self, payment_psp_reference: &str, request: &ReversalRequest) -> Result<ReversalResponse> {
        let url = format!("{}/v71/payments/{}/reversals",
            self.client.config().environment().checkout_api_url(),
            urlencoding::encode(payment_psp_reference)
        );
        let response = self.client.post(&url, request).await?;
        Ok(response.data)
    }

    /// Update payment amount.
    ///
    /// Updates the amount of an authorized payment.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn update_payment_amount(&self, payment_psp_reference: &str, request: &AmountUpdateRequest) -> Result<AmountUpdateResponse> {
        let url = format!("{}/v71/payments/{}/amountUpdates",
            self.client.config().environment().checkout_api_url(),
            urlencoding::encode(payment_psp_reference)
        );
        let response = self.client.post(&url, request).await?;
        Ok(response.data)
    }

    /// Cancel payment (legacy endpoint).
    ///
    /// Cancels a payment using the legacy cancel endpoint.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn cancel(&self, request: &CancelRequest) -> Result<CancelResponse> {
        let url = format!("{}/v71/cancels", self.client.config().environment().checkout_api_url());
        let response = self.client.post(&url, request).await?;
        Ok(response.data)
    }

    /// Create an order.
    ///
    /// Creates an order for payment processing.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn create_order(&self, request: &CreateOrderRequest) -> Result<CreateOrderResponse> {
        let url = format!("{}/v71/orders", self.client.config().environment().checkout_api_url());
        let response = self.client.post(&url, request).await?;
        Ok(response.data)
    }

    /// Cancel an order.
    ///
    /// Cancels an existing order.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn cancel_order(&self, request: &CancelOrderRequest) -> Result<CancelOrderResponse> {
        let url = format!("{}/v71/orders/cancel", self.client.config().environment().checkout_api_url());
        let response = self.client.post(&url, request).await?;
        Ok(response.data)
    }

    /// Make a donation.
    ///
    /// Processes a donation request.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn donations(&self, request: &DonationRequest) -> Result<DonationResponse> {
        let url = format!("{}/v71/donations", self.client.config().environment().checkout_api_url());
        let response = self.client.post(&url, request).await?;
        Ok(response.data)
    }

    /// Get donation campaigns.
    ///
    /// Retrieves available donation campaigns.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn donation_campaigns(&self, request: &DonationCampaignsRequest) -> Result<DonationCampaignsResponse> {
        let url = format!("{}/v71/donationCampaigns", self.client.config().environment().checkout_api_url());
        let response = self.client.post(&url, request).await?;
        Ok(response.data)
    }

    /// Update PayPal order.
    ///
    /// Updates a PayPal order with new information.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn paypal_update_order(&self, request: &PayPalUpdateOrderRequest) -> Result<PayPalUpdateOrderResponse> {
        let url = format!("{}/v71/paypal/updateOrder", self.client.config().environment().checkout_api_url());
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