//! Recurring API client implementation.

use adyen_core::{Client, Config, Result};
use crate::types::*;

/// Adyen Recurring API client.
///
/// Provides access to Adyen's Recurring API v68 for managing stored payment methods,
/// recurring payments, and shopper notifications.
///
/// # Example
///
/// ```rust
/// use adyen_core::{ConfigBuilder, Environment};
/// use adyen_recurring::RecurringApi;
///
/// # fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let config = ConfigBuilder::new()
///     .environment(Environment::test())
///     .api_key("your_api_key")?
///     .build()?;
///
/// let recurring = RecurringApi::new(config)?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct RecurringApi {
    client: Client,
}

impl RecurringApi {
    /// Create a new Recurring API client with the given configuration.
    ///
    /// # Errors
    ///
    /// Returns an error if the underlying HTTP client cannot be created.
    pub fn new(config: Config) -> Result<Self> {
        let client = Client::new(config)?;
        Ok(Self { client })
    }

    /// Retrieve stored payment methods for a shopper.
    ///
    /// Returns a list of the shopper's stored payment methods available for recurring payments.
    /// You can use this list to present the available payment methods to the shopper.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    ///
    /// # Example
    ///
    /// ```rust
    /// use adyen_recurring::{RecurringApi, RecurringDetailsRequest};
    ///
    /// # async fn example(recurring: RecurringApi) -> Result<(), Box<dyn std::error::Error>> {
    /// let request = RecurringDetailsRequest::builder()
    ///     .merchant_account("YourMerchantAccount")
    ///     .shopper_reference("shopper_12345")
    ///     .build()?;
    ///
    /// let response = recurring.list_recurring_details(&request).await?;
    /// println!("Found {} stored payment methods", response.details.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list_recurring_details(&self, request: &RecurringDetailsRequest) -> Result<RecurringDetailsResult> {
        let url = format!("{}/pal/servlet/Recurring/v68/listRecurringDetails", self.client.config().environment().classic_api_url());
        let response = self.client.post(&url, request).await?;
        Ok(response.data)
    }

    /// Disable a stored payment method.
    ///
    /// Disables a shopper's stored payment method. This prevents the payment method
    /// from being used for future recurring payments.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    ///
    /// # Example
    ///
    /// ```rust
    /// use adyen_recurring::{RecurringApi, DisableRequest};
    ///
    /// # async fn example(recurring: RecurringApi) -> Result<(), Box<dyn std::error::Error>> {
    /// let request = DisableRequest::builder()
    ///     .merchant_account("YourMerchantAccount")
    ///     .shopper_reference("shopper_12345")
    ///     .recurring_detail_reference("8415736344864224")
    ///     .build()?;
    ///
    /// let response = recurring.disable(&request).await?;
    /// println!("Disable result: {:?}", response.response);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn disable(&self, request: &DisableRequest) -> Result<DisableResult> {
        let url = format!("{}/pal/servlet/Recurring/v68/disable", self.client.config().environment().classic_api_url());
        let response = self.client.post(&url, request).await?;
        Ok(response.data)
    }

    /// Send a notification to the shopper about their stored payment methods.
    ///
    /// Sends a notification to the shopper with information about their stored payment methods.
    /// This can be used to inform shoppers about expiring cards or other account updates.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn notify_shopper(&self, request: &NotifyShopperRequest) -> Result<NotifyShopperResult> {
        let url = format!("{}/pal/servlet/Recurring/v68/notifyShopper", self.client.config().environment().classic_api_url());
        let response = self.client.post(&url, request).await?;
        Ok(response.data)
    }

    /// Schedule an account updater service request.
    ///
    /// Schedules an account updater service to automatically update stored card details
    /// when cards are renewed or replaced by the issuing bank.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn schedule_account_updater(&self, request: &ScheduleAccountUpdaterRequest) -> Result<ScheduleAccountUpdaterResult> {
        let url = format!("{}/pal/servlet/Recurring/v68/scheduleAccountUpdater", self.client.config().environment().classic_api_url());
        let response = self.client.post(&url, request).await?;
        Ok(response.data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use adyen_core::{ConfigBuilder, Environment};

    #[test]
    fn test_recurring_api_creation() {
        let config = ConfigBuilder::new()
            .environment(Environment::test())
            .api_key("test_key_1234567890123456")
            .unwrap()
            .build()
            .unwrap();

        let api = RecurringApi::new(config).unwrap();
        assert!(api.client.config().environment().is_test());
    }
}