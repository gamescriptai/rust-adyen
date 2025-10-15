//! Checkout session types for creating and managing sessions.

use adyen_core::{Amount, AdyenError, Result};
use crate::types::payments::Address;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Request to create a checkout session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCheckoutSessionRequest {
    /// The payment amount and currency.
    pub amount: Amount,

    /// The merchant account identifier.
    pub merchant_account: String,

    /// Your reference for the session.
    pub reference: String,

    /// The URL to return to after the payment.
    pub return_url: String,

    /// The sales channel for the transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,

    /// The shopper's country code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,

    /// The shopper's locale.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shopper_locale: Option<String>,

    /// The shopper reference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shopper_reference: Option<String>,

    /// The shopper's email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shopper_email: Option<String>,

    /// Whether to store payment methods for future use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store_payment_method: Option<bool>,

    /// Billing address for the session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<Address>,

    /// Delivery address for the session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_address: Option<Address>,

    /// Line items for the payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_items: Option<Vec<LineItem>>,

    /// Additional data for the session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<HashMap<String, String>>,

    /// The session expiry time in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
}

/// Response from creating a checkout session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCheckoutSessionResponse {
    /// The unique session identifier.
    pub id: String,

    /// The session data for the frontend.
    pub session_data: String,

    /// The URL for the session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    /// The session expiry time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,

    /// The amount for the session.
    pub amount: Amount,

    /// The merchant account.
    pub merchant_account: String,

    /// The reference for the session.
    pub reference: String,

    /// The return URL.
    pub return_url: String,

    /// The country code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,

    /// The shopper locale.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shopper_locale: Option<String>,
}

/// A line item for the payment.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LineItem {
    /// The item ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The item description.
    pub description: String,

    /// The quantity of the item.
    pub quantity: u32,

    /// The amount per item.
    pub amount_including_tax: Amount,

    /// The amount excluding tax.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_excluding_tax: Option<Amount>,

    /// The tax amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_amount: Option<Amount>,

    /// The tax percentage (in basis points).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_percentage: Option<u32>,

    /// The item category.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_category: Option<String>,

    /// Additional item data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<HashMap<String, String>>,
}

/// Builder for creating checkout session requests.
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionRequestBuilder {
    amount: Option<Amount>,
    merchant_account: Option<String>,
    reference: Option<String>,
    return_url: Option<String>,
    channel: Option<String>,
    country_code: Option<String>,
    shopper_locale: Option<String>,
    shopper_reference: Option<String>,
    shopper_email: Option<String>,
    store_payment_method: Option<bool>,
    billing_address: Option<Address>,
    delivery_address: Option<Address>,
    line_items: Option<Vec<LineItem>>,
    additional_data: Option<HashMap<String, String>>,
    expires_at: Option<String>,
}

impl CreateCheckoutSessionRequestBuilder {
    /// Create a new builder.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the payment amount.
    #[must_use]
    pub fn amount(mut self, amount: Amount) -> Self {
        self.amount = Some(amount);
        self
    }

    /// Set the merchant account.
    #[must_use]
    pub fn merchant_account(mut self, merchant_account: impl Into<String>) -> Self {
        self.merchant_account = Some(merchant_account.into());
        self
    }

    /// Set the session reference.
    #[must_use]
    pub fn reference(mut self, reference: impl Into<String>) -> Self {
        self.reference = Some(reference.into());
        self
    }

    /// Set the return URL.
    #[must_use]
    pub fn return_url(mut self, return_url: impl Into<String>) -> Self {
        self.return_url = Some(return_url.into());
        self
    }

    /// Set the sales channel.
    #[must_use]
    pub fn channel(mut self, channel: impl Into<String>) -> Self {
        self.channel = Some(channel.into());
        self
    }

    /// Set the shopper's country code.
    #[must_use]
    pub fn country_code(mut self, country_code: impl Into<String>) -> Self {
        self.country_code = Some(country_code.into());
        self
    }

    /// Set the shopper's locale.
    #[must_use]
    pub fn shopper_locale(mut self, locale: impl Into<String>) -> Self {
        self.shopper_locale = Some(locale.into());
        self
    }

    /// Set the shopper reference.
    #[must_use]
    pub fn shopper_reference(mut self, reference: impl Into<String>) -> Self {
        self.shopper_reference = Some(reference.into());
        self
    }

    /// Set the shopper's email.
    #[must_use]
    pub fn shopper_email(mut self, email: impl Into<String>) -> Self {
        self.shopper_email = Some(email.into());
        self
    }

    /// Set whether to store payment methods.
    #[must_use]
    pub fn store_payment_method(mut self, store: bool) -> Self {
        self.store_payment_method = Some(store);
        self
    }

    /// Set billing address.
    #[must_use]
    pub fn billing_address(mut self, address: Address) -> Self {
        self.billing_address = Some(address);
        self
    }

    /// Set delivery address.
    #[must_use]
    pub fn delivery_address(mut self, address: Address) -> Self {
        self.delivery_address = Some(address);
        self
    }

    /// Add a line item.
    #[must_use]
    pub fn line_item(mut self, item: LineItem) -> Self {
        self.line_items.get_or_insert_with(Vec::new).push(item);
        self
    }

    /// Set line items.
    #[must_use]
    pub fn line_items(mut self, items: Vec<LineItem>) -> Self {
        self.line_items = Some(items);
        self
    }

    /// Set session expiry time.
    #[must_use]
    pub fn expires_at(mut self, expires_at: impl Into<String>) -> Self {
        self.expires_at = Some(expires_at.into());
        self
    }

    /// Add additional data.
    #[must_use]
    pub fn additional_data(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.additional_data
            .get_or_insert_with(HashMap::new)
            .insert(key.into(), value.into());
        self
    }

    /// Build the session request.
    ///
    /// # Errors
    ///
    /// Returns an error if required fields are not set.
    pub fn build(self) -> Result<CreateCheckoutSessionRequest> {
        let amount = self.amount
            .ok_or_else(|| AdyenError::config("amount is required"))?;
        let merchant_account = self.merchant_account
            .ok_or_else(|| AdyenError::config("merchant_account is required"))?;
        let reference = self.reference
            .ok_or_else(|| AdyenError::config("reference is required"))?;
        let return_url = self.return_url
            .ok_or_else(|| AdyenError::config("return_url is required"))?;

        Ok(CreateCheckoutSessionRequest {
            amount,
            merchant_account,
            reference,
            return_url,
            channel: self.channel,
            country_code: self.country_code,
            shopper_locale: self.shopper_locale,
            shopper_reference: self.shopper_reference,
            shopper_email: self.shopper_email,
            store_payment_method: self.store_payment_method,
            billing_address: self.billing_address,
            delivery_address: self.delivery_address,
            line_items: self.line_items,
            additional_data: self.additional_data,
            expires_at: self.expires_at,
        })
    }
}

impl CreateCheckoutSessionRequest {
    /// Create a new builder for session requests.
    #[must_use]
    pub fn builder() -> CreateCheckoutSessionRequestBuilder {
        CreateCheckoutSessionRequestBuilder::new()
    }
}

impl LineItem {
    /// Create a new line item.
    #[must_use]
    pub fn new(description: impl Into<String>, quantity: u32, amount_including_tax: Amount) -> Self {
        Self {
            id: None,
            description: description.into(),
            quantity,
            amount_including_tax,
            amount_excluding_tax: None,
            tax_amount: None,
            tax_percentage: None,
            item_category: None,
            additional_data: None,
        }
    }

    /// Set the item ID.
    #[must_use]
    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Set the tax information.
    #[must_use]
    pub fn with_tax(mut self, amount_excluding_tax: Amount, tax_amount: Amount, tax_percentage: u32) -> Self {
        self.amount_excluding_tax = Some(amount_excluding_tax);
        self.tax_amount = Some(tax_amount);
        self.tax_percentage = Some(tax_percentage);
        self
    }

    /// Set the item category.
    #[must_use]
    pub fn with_category(mut self, category: impl Into<String>) -> Self {
        self.item_category = Some(category.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use adyen_core::{Amount, Currency};

    #[test]
    fn test_create_checkout_session_request_builder() {
        let amount = Amount::from_major_units(100, Currency::EUR);

        let request = CreateCheckoutSessionRequest::builder()
            .amount(amount.clone())
            .merchant_account("TestMerchant")
            .reference("Session-12345")
            .return_url("https://example.com/return")
            .country_code("NL")
            .shopper_locale("nl-NL")
            .build()
            .unwrap();

        assert_eq!(request.amount, amount);
        assert_eq!(request.merchant_account, "TestMerchant");
        assert_eq!(request.reference, "Session-12345");
        assert_eq!(request.return_url, "https://example.com/return");
        assert_eq!(request.country_code, Some("NL".to_string()));
        assert_eq!(request.shopper_locale, Some("nl-NL".to_string()));
    }

    #[test]
    fn test_line_item_creation() {
        let amount = Amount::from_major_units(10, Currency::EUR);
        let line_item = LineItem::new("Test Product", 2, amount.clone())
            .with_id("item-123")
            .with_category("electronics");

        assert_eq!(line_item.description, "Test Product");
        assert_eq!(line_item.quantity, 2);
        assert_eq!(line_item.amount_including_tax, amount);
        assert_eq!(line_item.id, Some("item-123".to_string()));
        assert_eq!(line_item.item_category, Some("electronics".to_string()));
    }

    #[test]
    fn test_session_request_with_line_items() {
        let amount = Amount::from_major_units(100, Currency::EUR);
        let item_amount = Amount::from_major_units(50, Currency::EUR);

        let line_item1 = LineItem::new("Product 1", 1, item_amount.clone());
        let line_item2 = LineItem::new("Product 2", 1, item_amount);

        let request = CreateCheckoutSessionRequest::builder()
            .amount(amount)
            .merchant_account("TestMerchant")
            .reference("Session-12345")
            .return_url("https://example.com/return")
            .line_item(line_item1)
            .line_item(line_item2)
            .build()
            .unwrap();

        assert_eq!(request.line_items.as_ref().unwrap().len(), 2);
    }
}