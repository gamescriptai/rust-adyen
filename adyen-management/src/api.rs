//! Management API client implementation.

use crate::types::*;
use adyen_core::{Client, Config, Result};

/// Adyen Management API client.
///
/// Provides access to Adyen's Management API v3 for configuring and managing
/// your Adyen company and merchant accounts, stores, and payment terminals.
///
/// # Example
///
/// ```rust
/// use adyen_core::{ConfigBuilder, Environment};
/// use adyen_management::ManagementApi;
///
/// # fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let config = ConfigBuilder::new()
///     .environment(Environment::test())
///     .api_key("your_api_key")?
///     .build()?;
///
/// let management = ManagementApi::new(config)?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct ManagementApi {
    client: Client,
}

impl ManagementApi {
    /// Create a new Management API client with the given configuration.
    ///
    /// # Errors
    ///
    /// Returns an error if the underlying HTTP client cannot be created.
    pub fn new(config: Config) -> Result<Self> {
        let client = Client::new(config)?;
        Ok(Self { client })
    }

    // Company Management

    /// Get a company account.
    ///
    /// Returns the company account details for the specified company ID.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn get_company(&self, company_id: &str) -> Result<Company> {
        let url = format!(
            "{}/v3/companies/{}",
            self.client.config().environment().management_api_url(),
            company_id
        );
        let response = self.client.get(&url).await?;
        Ok(response.data)
    }

    /// List all merchant accounts for a company.
    ///
    /// Returns a list of merchant accounts under the specified company.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn list_merchants(&self, company_id: &str) -> Result<Vec<MerchantAccount>> {
        let url = format!(
            "{}/v3/companies/{}/merchants",
            self.client.config().environment().management_api_url(),
            company_id
        );
        let response: adyen_core::ApiResponse<ListMerchantsResponse> =
            self.client.get(&url).await?;
        Ok(response.data.data)
    }

    /// Create a new merchant account.
    ///
    /// Creates a new merchant account under the specified company.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    ///
    /// # Example
    ///
    /// ```rust
    /// use adyen_management::{ManagementApi, CreateMerchantRequest};
    /// use adyen_management::types::{BusinessDetails, Contact, Address};
    ///
    /// # async fn example(management: ManagementApi) -> Result<(), Box<dyn std::error::Error>> {
    /// let business_details = BusinessDetails {
    ///     legal_business_name: "Example Corp".into(),
    ///     trading_name: Some("Example Store".into()),
    ///     mcc: Some("5411".into()),
    ///     registration: None,
    ///     website_url: Some("https://example.com".into()),
    /// };
    ///
    /// let contact = Contact {
    ///     email: "contact@example.com".into(),
    ///     first_name: Some("John".into()),
    ///     last_name: Some("Doe".into()),
    ///     phone_number: Some("+1234567890".into()),
    /// };
    ///
    /// let request = CreateMerchantRequest::builder()
    ///     .company_id("company_123")
    ///     .merchant_account("ExampleMerchant")
    ///     .business_details(business_details)
    ///     .primary_contact(contact)
    ///     .build()?;
    ///
    /// let merchant = management.create_merchant(&request).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_merchant(
        &self,
        request: &CreateMerchantRequest,
    ) -> Result<MerchantAccount> {
        let url = format!(
            "{}/v3/companies/{}/merchants",
            self.client.config().environment().management_api_url(),
            request.company_id
        );
        let response = self.client.post(&url, request).await?;
        Ok(response.data)
    }

    /// Get a merchant account.
    ///
    /// Returns the merchant account details for the specified merchant ID.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn get_merchant(&self, merchant_id: &str) -> Result<MerchantAccount> {
        let url = format!(
            "{}/v3/merchants/{}",
            self.client.config().environment().management_api_url(),
            merchant_id
        );
        let response = self.client.get(&url).await?;
        Ok(response.data)
    }

    // Store Management

    /// List all stores for a merchant.
    ///
    /// Returns a list of stores under the specified merchant account.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn list_stores(&self, merchant_id: &str) -> Result<Vec<Store>> {
        let url = format!(
            "{}/v3/merchants/{}/stores",
            self.client.config().environment().management_api_url(),
            merchant_id
        );
        let response: adyen_core::ApiResponse<ListStoresResponse> = self.client.get(&url).await?;
        Ok(response.data.data)
    }

    /// Create a new store.
    ///
    /// Creates a new store under the specified merchant account.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    ///
    /// # Example
    ///
    /// ```rust
    /// use adyen_management::{ManagementApi, CreateStoreRequest, Address};
    ///
    /// # async fn example(management: ManagementApi) -> Result<(), Box<dyn std::error::Error>> {
    /// let address = Address {
    ///     street_address: "123 Main St".into(),
    ///     street_address2: None,
    ///     city: "New York".into(),
    ///     state_or_province: Some("NY".into()),
    ///     postal_code: "10001".into(),
    ///     country: "US".into(),
    /// };
    ///
    /// let request = CreateStoreRequest::builder()
    ///     .store_reference("store_001")
    ///     .description("Main Store")
    ///     .address(address)
    ///     .phone_number("+1234567890")
    ///     .build()?;
    ///
    /// let store = management.create_store("merchant_123", &request).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_store(
        &self,
        merchant_id: &str,
        request: &CreateStoreRequest,
    ) -> Result<Store> {
        let url = format!(
            "{}/v3/merchants/{}/stores",
            self.client.config().environment().management_api_url(),
            merchant_id
        );
        let response = self.client.post(&url, request).await?;
        Ok(response.data)
    }

    /// Get a store.
    ///
    /// Returns the store details for the specified store ID.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn get_store(&self, merchant_id: &str, store_id: &str) -> Result<Store> {
        let url = format!(
            "{}/v3/merchants/{}/stores/{}",
            self.client.config().environment().management_api_url(),
            merchant_id,
            store_id
        );
        let response = self.client.get(&url).await?;
        Ok(response.data)
    }

    /// Update a store.
    ///
    /// Updates the store with the provided information.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn update_store(
        &self,
        merchant_id: &str,
        store_id: &str,
        request: &CreateStoreRequest,
    ) -> Result<Store> {
        let url = format!(
            "{}/v3/merchants/{}/stores/{}",
            self.client.config().environment().management_api_url(),
            merchant_id,
            store_id
        );
        let response = self.client.patch(&url, request).await?;
        Ok(response.data)
    }

    // Payment Method Management

    /// Get payment method settings for a merchant.
    ///
    /// Returns the payment method configuration for the specified merchant.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn get_payment_method_settings(
        &self,
        merchant_id: &str,
    ) -> Result<PaymentMethodSettings> {
        let url = format!(
            "{}/v3/merchants/{}/paymentMethodSettings",
            self.client.config().environment().management_api_url(),
            merchant_id
        );
        let response = self.client.get(&url).await?;
        Ok(response.data)
    }

    /// Update payment method settings for a merchant.
    ///
    /// Updates the payment method configuration for the specified merchant.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn update_payment_method(
        &self,
        merchant_id: &str,
        payment_method_id: &str,
        request: &UpdatePaymentMethodRequest,
    ) -> Result<PaymentMethod> {
        let url = format!(
            "{}/v3/merchants/{}/paymentMethodSettings/{}",
            self.client.config().environment().management_api_url(),
            merchant_id,
            payment_method_id
        );
        let response = self.client.patch(&url, request).await?;
        Ok(response.data)
    }

    /// Request a new payment method for a merchant.
    ///
    /// Requests a new payment method to be enabled for the merchant.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn request_payment_method(
        &self,
        merchant_id: &str,
        request: &UpdatePaymentMethodRequest,
    ) -> Result<PaymentMethod> {
        let url = format!(
            "{}/v3/merchants/{}/paymentMethodSettings",
            self.client.config().environment().management_api_url(),
            merchant_id
        );
        let response = self.client.post(&url, request).await?;
        Ok(response.data)
    }

    // Webhook Management

    /// List all webhooks for a merchant.
    ///
    /// Returns a list of webhook configurations for the specified merchant.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn list_webhooks(&self, merchant_id: &str) -> Result<Vec<Webhook>> {
        let url = format!(
            "{}/v3/merchants/{}/webhooks",
            self.client.config().environment().management_api_url(),
            merchant_id
        );
        let response: adyen_core::ApiResponse<ListWebhooksResponse> = self.client.get(&url).await?;
        Ok(response.data.data)
    }

    /// Create a new webhook.
    ///
    /// Creates a new webhook configuration for the specified merchant.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    ///
    /// # Example
    ///
    /// ```rust
    /// use adyen_management::{ManagementApi, CreateWebhookRequest};
    ///
    /// # async fn example(management: ManagementApi) -> Result<(), Box<dyn std::error::Error>> {
    /// let request = CreateWebhookRequest {
    ///     url: "https://example.com/webhook".into(),
    ///     description: Some("Payment notifications".into()),
    ///     active: true,
    ///     communication_format: "json".into(),
    ///     filter_merchant_accounts: Some(vec!["merchant_123".into()]),
    ///     additional_settings: None,
    /// };
    ///
    /// let webhook = management.create_webhook("merchant_123", &request).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_webhook(
        &self,
        merchant_id: &str,
        request: &CreateWebhookRequest,
    ) -> Result<Webhook> {
        let url = format!(
            "{}/v3/merchants/{}/webhooks",
            self.client.config().environment().management_api_url(),
            merchant_id
        );
        let response = self.client.post(&url, request).await?;
        Ok(response.data)
    }

    /// Get a webhook.
    ///
    /// Returns the webhook configuration for the specified webhook ID.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn get_webhook(&self, merchant_id: &str, webhook_id: &str) -> Result<Webhook> {
        let url = format!(
            "{}/v3/merchants/{}/webhooks/{}",
            self.client.config().environment().management_api_url(),
            merchant_id,
            webhook_id
        );
        let response = self.client.get(&url).await?;
        Ok(response.data)
    }

    /// Update a webhook.
    ///
    /// Updates the webhook configuration for the specified webhook ID.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn update_webhook(
        &self,
        merchant_id: &str,
        webhook_id: &str,
        request: &UpdateWebhookRequest,
    ) -> Result<Webhook> {
        let url = format!(
            "{}/v3/merchants/{}/webhooks/{}",
            self.client.config().environment().management_api_url(),
            merchant_id,
            webhook_id
        );
        let response = self.client.patch(&url, request).await?;
        Ok(response.data)
    }

    /// Delete a webhook.
    ///
    /// Deletes the webhook configuration for the specified webhook ID.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails.
    pub async fn delete_webhook(&self, merchant_id: &str, webhook_id: &str) -> Result<()> {
        let url = format!(
            "{}/v3/merchants/{}/webhooks/{}",
            self.client.config().environment().management_api_url(),
            merchant_id,
            webhook_id
        );
        self.client.delete(&url).await?;
        Ok(())
    }

    // Terminal Management

    /// List terminal models available to a merchant.
    ///
    /// Returns a list of terminal models that the merchant can order.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn list_terminal_models(&self, merchant_id: &str) -> Result<Vec<TerminalModel>> {
        let url = format!(
            "{}/v3/merchants/{}/terminalModels",
            self.client.config().environment().management_api_url(),
            merchant_id
        );
        let response: adyen_core::ApiResponse<ListTerminalModelsResponse> =
            self.client.get(&url).await?;
        Ok(response.data.data)
    }

    /// Get terminal settings for a merchant.
    ///
    /// Returns the terminal settings configuration for the specified merchant.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn get_terminal_settings(&self, merchant_id: &str) -> Result<TerminalSettings> {
        let url = format!(
            "{}/v3/merchants/{}/terminalSettings",
            self.client.config().environment().management_api_url(),
            merchant_id
        );
        let response = self.client.get(&url).await?;
        Ok(response.data)
    }

    /// Update terminal settings for a merchant.
    ///
    /// Updates the terminal settings configuration for the specified merchant.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn update_terminal_settings(
        &self,
        merchant_id: &str,
        request: &TerminalSettings,
    ) -> Result<TerminalSettings> {
        let url = format!(
            "{}/v3/merchants/{}/terminalSettings",
            self.client.config().environment().management_api_url(),
            merchant_id
        );
        let response = self.client.patch(&url, request).await?;
        Ok(response.data)
    }

    /// List terminals for a store.
    ///
    /// Returns a list of terminals assigned to the specified store.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn list_store_terminals(
        &self,
        merchant_id: &str,
        store_id: &str,
    ) -> Result<Vec<Terminal>> {
        let url = format!(
            "{}/v3/merchants/{}/stores/{}/terminals",
            self.client.config().environment().management_api_url(),
            merchant_id,
            store_id
        );
        let response: adyen_core::ApiResponse<ListTerminalsResponse> =
            self.client.get(&url).await?;
        Ok(response.data.data)
    }
}

// Response wrapper types for list endpoints
#[derive(Debug, Clone, serde::Deserialize)]
struct ListMerchantsResponse {
    data: Vec<MerchantAccount>,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct ListStoresResponse {
    data: Vec<Store>,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct ListWebhooksResponse {
    data: Vec<Webhook>,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct ListTerminalModelsResponse {
    data: Vec<TerminalModel>,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct ListTerminalsResponse {
    data: Vec<Terminal>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use adyen_core::{ConfigBuilder, Environment};

    #[test]
    fn test_management_api_creation() {
        let config = ConfigBuilder::new()
            .environment(Environment::test())
            .api_key("test_key_1234567890123456")
            .unwrap()
            .build()
            .unwrap();

        let _api = ManagementApi::new(config).unwrap();
        // API created successfully indicates proper configuration
    }
}
