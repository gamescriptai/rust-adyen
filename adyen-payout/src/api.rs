//! Payout API client implementation.

use crate::types::*;
use adyen_core::{Client, Config, Result};

/// Adyen Payout API client.
///
/// Provides access to Adyen's Payout API v68 for fund disbursement and payout management.
/// This includes instant payouts, batch processing, review workflows, and comprehensive
/// status tracking.
///
/// # Example
///
/// ```rust
/// use adyen_core::{ConfigBuilder, Environment};
/// use adyen_payout::PayoutApi;
///
/// # fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let config = ConfigBuilder::new()
///     .environment(Environment::test())
///     .api_key("your_api_key")?
///     .build()?;
///
/// let payout = PayoutApi::new(config)?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct PayoutApi {
    client: Client,
}

impl PayoutApi {
    /// Create a new Payout API client with the given configuration.
    ///
    /// # Errors
    ///
    /// Returns an error if the underlying HTTP client cannot be created.
    pub fn new(config: Config) -> Result<Self> {
        let client = Client::new(config)?;
        Ok(Self { client })
    }

    /// Submit a payout request.
    ///
    /// Submits a payout to a shopper's bank account or card. For instant payouts,
    /// this request needs to be followed by a `/confirmThirdParty` request to finalize the payout.
    /// For regular payouts, the payout is processed automatically.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    ///
    /// # Example
    ///
    /// ```rust
    /// use adyen_payout::{PayoutApi, SubmitRequest, PayoutMethodDetails, BankAccount};
    /// use adyen_core::{Amount, Currency};
    ///
    /// # async fn example(payout: PayoutApi) -> Result<(), Box<dyn std::error::Error>> {
    /// let bank_account = BankAccount {
    ///     account_number: "1234567890".into(),
    ///     bic: Some("ABNANL2A".into()),
    ///     country_code: "NL".into(),
    ///     owner_name: "John Doe".into(),
    ///     iban: Some("NL91ABNA0417164300".into()),
    ///     bank_account_type: Some(adyen_payout::BankAccountType::Checking),
    /// };
    ///
    /// let request = SubmitRequest::builder()
    ///     .amount(Amount::from_minor_units(1000, Currency::EUR))
    ///     .merchant_account("YourMerchantAccount")
    ///     .reference("payout-001")
    ///     .shopper_email("shopper@example.com")
    ///     .shopper_reference("shopper_123")
    ///     .payout_method_details(PayoutMethodDetails::BankAccount(bank_account))
    ///     .build()?;
    ///
    /// let response = payout.submit(&request).await?;
    /// println!("Payout submitted with PSP reference: {}", response.psp_reference);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn submit(&self, request: &SubmitRequest) -> Result<SubmitResponse> {
        let url = format!(
            "{}/pal/servlet/Payout/v68/submitThirdParty",
            self.client.config().environment().classic_api_url()
        );
        let response = self.client.post(&url, request).await?;
        Ok(response.data)
    }

    /// Confirm a payout that was submitted earlier.
    ///
    /// Confirms (and finalizes) a previously submitted payout. This is required for instant payouts
    /// after the initial `/submitThirdParty` request. Once confirmed, the payout cannot be cancelled.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    ///
    /// # Example
    ///
    /// ```rust
    /// use adyen_payout::{PayoutApi, ConfirmRequest};
    ///
    /// # async fn example(payout: PayoutApi) -> Result<(), Box<dyn std::error::Error>> {
    /// let request = ConfirmRequest::builder()
    ///     .merchant_account("YourMerchantAccount")
    ///     .original_reference("8515131751004933")
    ///     .build()?;
    ///
    /// let response = payout.confirm(&request).await?;
    /// println!("Payout confirmed: {}", response.response);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn confirm(&self, request: &ConfirmRequest) -> Result<ConfirmResponse> {
        let url = format!(
            "{}/pal/servlet/Payout/v68/confirmThirdParty",
            self.client.config().environment().classic_api_url()
        );
        let response = self.client.post(&url, request).await?;
        Ok(response.data)
    }

    /// Decline a payout that requires manual review.
    ///
    /// Declines a payout that was flagged for manual review. This permanently rejects the payout
    /// and prevents it from being processed.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    ///
    /// # Example
    ///
    /// ```rust
    /// use adyen_payout::{PayoutApi, DeclinePayoutRequest};
    ///
    /// # async fn example(payout: PayoutApi) -> Result<(), Box<dyn std::error::Error>> {
    /// let request = DeclinePayoutRequest::builder()
    ///     .merchant_account("YourMerchantAccount")
    ///     .psp_reference("8515131751004933")
    ///     .build()?;
    ///
    /// let response = payout.decline_payout(&request).await?;
    /// println!("Payout declined: {}", response.response);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn decline_payout(&self, request: &DeclinePayoutRequest) -> Result<PayoutResponse> {
        let url = format!(
            "{}/pal/servlet/Payout/v68/declineThirdParty",
            self.client.config().environment().classic_api_url()
        );
        let response = self.client.post(&url, request).await?;
        Ok(response.data)
    }

    /// Make an instant card payout.
    ///
    /// Makes an instant payout to a card. Funds will be made available within 30 minutes
    /// on the cardholder's bank account.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    ///
    /// # Example
    ///
    /// ```rust
    /// use adyen_payout::{PayoutApi, SubmitRequest, PayoutMethodDetails, Card};
    /// use adyen_core::{Amount, Currency};
    ///
    /// # async fn example(payout: PayoutApi) -> Result<(), Box<dyn std::error::Error>> {
    /// let card = Card {
    ///     number: "4111111111111111".into(),
    ///     expiry_month: "12".into(),
    ///     expiry_year: "2030".into(),
    ///     holder_name: "John Doe".into(),
    /// };
    ///
    /// let request = SubmitRequest::builder()
    ///     .amount(Amount::from_minor_units(1000, Currency::EUR))
    ///     .merchant_account("YourMerchantAccount")
    ///     .reference("instant-payout-001")
    ///     .shopper_email("shopper@example.com")
    ///     .shopper_reference("shopper_123")
    ///     .payout_method_details(PayoutMethodDetails::Card(card))
    ///     .build()?;
    ///
    /// let response = payout.instant_payout(&request).await?;
    /// println!("Instant payout processed: {}", response.psp_reference);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn instant_payout(&self, request: &SubmitRequest) -> Result<SubmitResponse> {
        let url = format!(
            "{}/pal/servlet/Payout/v68/payout",
            self.client.config().environment().classic_api_url()
        );
        let response = self.client.post(&url, request).await?;
        Ok(response.data)
    }

    /// Store payout details and submit for future use.
    ///
    /// Stores the payout details and submits the payout. This endpoint combines storing
    /// shopper details for future payouts and immediate payout processing.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    ///
    /// # Example
    ///
    /// ```rust
    /// use adyen_payout::{PayoutApi, SubmitRequest, PayoutMethodDetails, BankAccount};
    /// use adyen_core::{Amount, Currency};
    ///
    /// # async fn example(payout: PayoutApi) -> Result<(), Box<dyn std::error::Error>> {
    /// let bank_account = BankAccount {
    ///     account_number: "1234567890".into(),
    ///     bic: Some("ABNANL2A".into()),
    ///     country_code: "NL".into(),
    ///     owner_name: "John Doe".into(),
    ///     iban: Some("NL91ABNA0417164300".into()),
    ///     bank_account_type: Some(adyen_payout::BankAccountType::Checking),
    /// };
    ///
    /// let request = SubmitRequest::builder()
    ///     .amount(Amount::from_minor_units(1000, Currency::EUR))
    ///     .merchant_account("YourMerchantAccount")
    ///     .reference("payout-001")
    ///     .shopper_email("shopper@example.com")
    ///     .shopper_reference("shopper_123")
    ///     .payout_method_details(PayoutMethodDetails::BankAccount(bank_account))
    ///     .build()?;
    ///
    /// let response = payout.store_detail_and_submit(&request).await?;
    /// println!("Payout stored and submitted: {}", response.psp_reference);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn store_detail_and_submit(&self, request: &SubmitRequest) -> Result<SubmitResponse> {
        let url = format!(
            "{}/pal/servlet/Payout/v68/storeDetailAndSubmitThirdParty",
            self.client.config().environment().classic_api_url()
        );
        let response = self.client.post(&url, request).await?;
        Ok(response.data)
    }

    /// Store payout details for future use.
    ///
    /// Stores shopper details for future payout requests without immediately submitting a payout.
    /// This is useful for setting up payout methods that will be used later.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    ///
    /// # Example
    ///
    /// ```rust
    /// use adyen_payout::{PayoutApi, SubmitRequest, PayoutMethodDetails, BankAccount};
    /// use adyen_core::{Amount, Currency};
    ///
    /// # async fn example(payout: PayoutApi) -> Result<(), Box<dyn std::error::Error>> {
    /// let bank_account = BankAccount {
    ///     account_number: "1234567890".into(),
    ///     bic: Some("ABNANL2A".into()),
    ///     country_code: "NL".into(),
    ///     owner_name: "John Doe".into(),
    ///     iban: Some("NL91ABNA0417164300".into()),
    ///     bank_account_type: Some(adyen_payout::BankAccountType::Checking),
    /// };
    ///
    /// let request = SubmitRequest::builder()
    ///     .amount(Amount::from_minor_units(0, Currency::EUR))  // Amount 0 for storing only
    ///     .merchant_account("YourMerchantAccount")
    ///     .reference("store-only-001")
    ///     .shopper_email("shopper@example.com")
    ///     .shopper_reference("shopper_123")
    ///     .payout_method_details(PayoutMethodDetails::BankAccount(bank_account))
    ///     .build()?;
    ///
    /// let response = payout.store_detail(&request).await?;
    /// println!("Payout details stored: {}", response.psp_reference);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn store_detail(&self, request: &SubmitRequest) -> Result<SubmitResponse> {
        let url = format!(
            "{}/pal/servlet/Payout/v68/storeDetail",
            self.client.config().environment().classic_api_url()
        );
        let response = self.client.post(&url, request).await?;
        Ok(response.data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use adyen_core::{ConfigBuilder, Environment};

    #[test]
    fn test_payout_api_creation() {
        let config = ConfigBuilder::new()
            .environment(Environment::test())
            .api_key("test_key_1234567890123456")
            .unwrap()
            .build()
            .unwrap();

        let _api = PayoutApi::new(config).unwrap();
        // API created successfully indicates proper configuration
    }

    #[test]
    fn test_payout_api_endpoints() {
        let config = ConfigBuilder::new()
            .environment(Environment::test())
            .api_key("test_key_1234567890123456")
            .unwrap()
            .build()
            .unwrap();

        let _api = PayoutApi::new(config).unwrap();

        // Test that the API instance was created successfully
        // API created successfully indicates proper configuration

        // Verify that the API instance was created successfully with test configuration
    }
}
