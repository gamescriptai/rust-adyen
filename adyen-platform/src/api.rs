//! Balance Platform API client implementation.

use adyen_core::{Client, Config, Result};
use crate::types::*;

/// Adyen Balance Platform API client.
///
/// Provides access to Adyen's Balance Platform API v2 for marketplace operations,
/// including balance account management, payment instruments, and transaction rules.
///
/// # Example
///
/// ```rust
/// use adyen_core::{ConfigBuilder, Environment};
/// use adyen_platform::BalancePlatformApi;
///
/// # fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let config = ConfigBuilder::new()
///     .environment(Environment::test())
///     .api_key("your_api_key")?
///     .build()?;
///
/// let platform = BalancePlatformApi::new(config)?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct BalancePlatformApi {
    client: Client,
}

impl BalancePlatformApi {
    /// Create a new Balance Platform API client with the given configuration.
    ///
    /// # Errors
    ///
    /// Returns an error if the underlying HTTP client cannot be created.
    pub fn new(config: Config) -> Result<Self> {
        let client = Client::new(config)?;
        Ok(Self { client })
    }

    // ============================================================================
    // Balance Account Management
    // ============================================================================

    /// Create a new balance account.
    ///
    /// Creates a balance account that holds the funds of the associated account holder.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    ///
    /// # Example
    ///
    /// ```rust
    /// use adyen_platform::{BalancePlatformApi, CreateBalanceAccountRequest};
    ///
    /// # async fn example(platform: BalancePlatformApi) -> Result<(), Box<dyn std::error::Error>> {
    /// let request = CreateBalanceAccountRequest::builder()
    ///     .account_holder_id("AH12345")
    ///     .description("Main balance account")
    ///     .default_currency_code("EUR")
    ///     .build()
    ///     .map_err(|e| format!("Builder error: {}", e))?;
    ///
    /// let balance_account = platform.create_balance_account(&request).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_balance_account(&self, request: &CreateBalanceAccountRequest) -> Result<BalanceAccount> {
        let url = format!("{}/v2/balanceAccounts", self.client.config().environment().balance_platform_api_url());
        let response = self.client.post(&url, request).await?;
        Ok(response.data)
    }

    /// Get a balance account by ID.
    ///
    /// Returns the balance account details for the specified ID.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn get_balance_account(&self, balance_account_id: &str) -> Result<BalanceAccount> {
        let url = format!("{}/v2/balanceAccounts/{}", self.client.config().environment().balance_platform_api_url(), balance_account_id);
        let response = self.client.get(&url).await?;
        Ok(response.data)
    }

    /// Update a balance account.
    ///
    /// Updates the balance account with the provided information.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn update_balance_account(&self, balance_account_id: &str, request: &CreateBalanceAccountRequest) -> Result<BalanceAccount> {
        let url = format!("{}/v2/balanceAccounts/{}", self.client.config().environment().balance_platform_api_url(), balance_account_id);
        let response = self.client.patch(&url, request).await?;
        Ok(response.data)
    }

    /// Close a balance account.
    ///
    /// Closes the balance account by setting its status to closed.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn close_balance_account(&self, balance_account_id: &str) -> Result<BalanceAccount> {
        let url = format!("{}/v2/balanceAccounts/{}/close", self.client.config().environment().balance_platform_api_url(), balance_account_id);
        let response = self.client.post(&url, &serde_json::json!({})).await?;
        Ok(response.data)
    }

    /// Get all balance accounts for an account holder.
    ///
    /// Returns a list of balance accounts associated with the specified account holder.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn list_balance_accounts(&self, account_holder_id: &str) -> Result<Vec<BalanceAccount>> {
        let url = format!("{}/v2/accountHolders/{}/balanceAccounts", self.client.config().environment().balance_platform_api_url(), account_holder_id);
        let response: adyen_core::ApiResponse<PaginatedResponse<BalanceAccount>> = self.client.get(&url).await?;
        Ok(response.data.data)
    }

    // ============================================================================
    // Account Holder Management
    // ============================================================================

    /// Create a new account holder.
    ///
    /// Creates an account holder linked to a legal entity that can hold balance accounts.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn create_account_holder(&self, request: &CreateAccountHolderRequest) -> Result<AccountHolder> {
        let url = format!("{}/v2/accountHolders", self.client.config().environment().balance_platform_api_url());
        let response = self.client.post(&url, request).await?;
        Ok(response.data)
    }

    /// Get an account holder by ID.
    ///
    /// Returns the account holder details for the specified ID.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn get_account_holder(&self, account_holder_id: &str) -> Result<AccountHolder> {
        let url = format!("{}/v2/accountHolders/{}", self.client.config().environment().balance_platform_api_url(), account_holder_id);
        let response = self.client.get(&url).await?;
        Ok(response.data)
    }

    /// Update an account holder.
    ///
    /// Updates the account holder with the provided information.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn update_account_holder(&self, account_holder_id: &str, request: &CreateAccountHolderRequest) -> Result<AccountHolder> {
        let url = format!("{}/v2/accountHolders/{}", self.client.config().environment().balance_platform_api_url(), account_holder_id);
        let response = self.client.patch(&url, request).await?;
        Ok(response.data)
    }

    /// Get all account holders.
    ///
    /// Returns a list of account holders associated with your balance platform.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn list_account_holders(&self) -> Result<Vec<AccountHolder>> {
        let url = format!("{}/v2/accountHolders", self.client.config().environment().balance_platform_api_url());
        let response: adyen_core::ApiResponse<PaginatedResponse<AccountHolder>> = self.client.get(&url).await?;
        Ok(response.data.data)
    }

    // ============================================================================
    // Payment Instrument Management
    // ============================================================================

    /// Create a new payment instrument.
    ///
    /// Creates a payment instrument to process payments through a balance account.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn create_payment_instrument(&self, request: &CreatePaymentInstrumentRequest) -> Result<PaymentInstrument> {
        let url = format!("{}/v2/paymentInstruments", self.client.config().environment().balance_platform_api_url());
        let response = self.client.post(&url, request).await?;
        Ok(response.data)
    }

    /// Get a payment instrument by ID.
    ///
    /// Returns the payment instrument details for the specified ID.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn get_payment_instrument(&self, payment_instrument_id: &str) -> Result<PaymentInstrument> {
        let url = format!("{}/v2/paymentInstruments/{}", self.client.config().environment().balance_platform_api_url(), payment_instrument_id);
        let response = self.client.get(&url).await?;
        Ok(response.data)
    }

    /// Update a payment instrument.
    ///
    /// Updates the payment instrument with the provided information.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn update_payment_instrument(&self, payment_instrument_id: &str, request: &CreatePaymentInstrumentRequest) -> Result<PaymentInstrument> {
        let url = format!("{}/v2/paymentInstruments/{}", self.client.config().environment().balance_platform_api_url(), payment_instrument_id);
        let response = self.client.patch(&url, request).await?;
        Ok(response.data)
    }

    /// Get all payment instruments for a balance account.
    ///
    /// Returns a list of payment instruments associated with the specified balance account.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn list_payment_instruments(&self, balance_account_id: &str) -> Result<Vec<PaymentInstrument>> {
        let url = format!("{}/v2/balanceAccounts/{}/paymentInstruments", self.client.config().environment().balance_platform_api_url(), balance_account_id);
        let response: adyen_core::ApiResponse<PaginatedResponse<PaymentInstrument>> = self.client.get(&url).await?;
        Ok(response.data.data)
    }

    // ============================================================================
    // Transaction Rules Management
    // ============================================================================

    /// Create a new transaction rule.
    ///
    /// Creates a rule to control transaction processing based on criteria.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn create_transaction_rule(&self, request: &CreateTransactionRuleRequest) -> Result<TransactionRule> {
        let url = format!("{}/v2/transactionRules", self.client.config().environment().balance_platform_api_url());
        let response = self.client.post(&url, request).await?;
        Ok(response.data)
    }

    /// Get a transaction rule by ID.
    ///
    /// Returns the transaction rule details for the specified ID.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn get_transaction_rule(&self, transaction_rule_id: &str) -> Result<TransactionRule> {
        let url = format!("{}/v2/transactionRules/{}", self.client.config().environment().balance_platform_api_url(), transaction_rule_id);
        let response = self.client.get(&url).await?;
        Ok(response.data)
    }

    /// Update a transaction rule.
    ///
    /// Updates the transaction rule with the provided information.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn update_transaction_rule(&self, transaction_rule_id: &str, request: &CreateTransactionRuleRequest) -> Result<TransactionRule> {
        let url = format!("{}/v2/transactionRules/{}", self.client.config().environment().balance_platform_api_url(), transaction_rule_id);
        let response = self.client.patch(&url, request).await?;
        Ok(response.data)
    }

    /// Delete a transaction rule.
    ///
    /// Deletes the transaction rule for the specified ID.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails.
    pub async fn delete_transaction_rule(&self, transaction_rule_id: &str) -> Result<()> {
        let url = format!("{}/v2/transactionRules/{}", self.client.config().environment().balance_platform_api_url(), transaction_rule_id);
        self.client.delete(&url).await?;
        Ok(())
    }

    /// Get all transaction rules for an entity.
    ///
    /// Returns a list of transaction rules associated with the specified entity.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn list_transaction_rules(&self, entity_type: &str, entity_id: &str) -> Result<Vec<TransactionRule>> {
        let url = format!("{}/v2/transactionRules?entityType={}&entityId={}",
            self.client.config().environment().balance_platform_api_url(), entity_type, entity_id);
        let response: adyen_core::ApiResponse<PaginatedResponse<TransactionRule>> = self.client.get(&url).await?;
        Ok(response.data.data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use adyen_core::{ConfigBuilder, Environment};

    #[test]
    fn test_balance_platform_api_creation() {
        let config = ConfigBuilder::new()
            .environment(Environment::test())
            .api_key("test_key_1234567890123456")
            .unwrap()
            .build()
            .unwrap();

        let _api = BalancePlatformApi::new(config).unwrap();
        // API created successfully indicates proper configuration
    }
}