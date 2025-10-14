//! Types for the Adyen Management API v3.
//!
//! This module contains all request and response types for managing company and merchant
//! accounts, stores, payment methods, terminals, and webhooks.

use adyen_core::{AdyenError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Company account information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Company {
    /// The unique identifier of the company account.
    pub id: Box<str>,
    /// The name of the company.
    pub name: Box<str>,
    /// The company's registration details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration: Option<CompanyRegistration>,
    /// The company's billing address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<Address>,
    /// Data processing specifications.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_processing: Option<DataProcessing>,
    /// Additional properties.
    #[serde(flatten)]
    pub additional_data: HashMap<String, serde_json::Value>,
}

/// Company registration details.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompanyRegistration {
    /// The company's registration number.
    pub registration_number: Box<str>,
    /// The country where the company is registered.
    pub country_code: Box<str>,
    /// The type of company registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_type: Option<Box<str>>,
}

/// Data processing specifications for compliance.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataProcessing {
    /// The data processing region.
    pub region: Box<str>,
    /// Whether data processing is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

/// Merchant account information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MerchantAccount {
    /// The unique identifier of the merchant account.
    pub id: Box<str>,
    /// The name of the merchant account.
    pub name: Box<str>,
    /// The merchant account code.
    pub merchant_code: Box<str>,
    /// The company ID this merchant belongs to.
    pub company_id: Box<str>,
    /// The merchant's business details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_details: Option<BusinessDetails>,
    /// The merchant's billing address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<Address>,
    /// The merchant account status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<MerchantStatus>,
    /// Primary contact information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_contact: Option<Contact>,
    /// Links to related resources.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Links>,
}

/// Request to create a new merchant account.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateMerchantRequest {
    /// The company ID to create the merchant under.
    pub company_id: Box<str>,
    /// The name of the merchant account.
    pub merchant_account: Box<str>,
    /// The merchant's business details.
    pub business_details: BusinessDetails,
    /// Primary contact information.
    pub primary_contact: Contact,
    /// The merchant's billing address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<Address>,
}

/// Business details for a merchant.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BusinessDetails {
    /// The legal name of the business.
    pub legal_business_name: Box<str>,
    /// The trading name of the business.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trading_name: Option<Box<str>>,
    /// The business category code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcc: Option<Box<str>>,
    /// The business registration details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration: Option<CompanyRegistration>,
    /// The business website URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website_url: Option<Box<str>>,
}

/// Merchant account status.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum MerchantStatus {
    Active,
    Inactive,
    Suspended,
    Closed,
}

/// Store information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Store {
    /// The unique identifier of the store.
    pub id: Box<str>,
    /// The store reference code.
    pub store_reference: Box<str>,
    /// The merchant ID this store belongs to.
    pub merchant_id: Box<str>,
    /// The name or description of the store.
    pub description: Box<str>,
    /// The store's address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
    /// The store's phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<Box<str>>,
    /// The store's status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<StoreStatus>,
    /// Business line ID for this store.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_line_id: Option<Box<str>>,
    /// Links to related resources.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Links>,
}

/// Request to create a new store.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateStoreRequest {
    /// The store reference code.
    pub store_reference: Box<str>,
    /// The name or description of the store.
    pub description: Box<str>,
    /// The store's address.
    pub address: Address,
    /// The store's phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<Box<str>>,
    /// Business line ID for this store.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_line_id: Option<Box<str>>,
}

/// Request to create a store with merchant code.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StoreCreationWithMerchantCodeRequest {
    /// The merchant account code.
    pub merchant_account: Box<str>,
    /// The store creation details.
    #[serde(flatten)]
    pub store_details: CreateStoreRequest,
}

/// Store status.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum StoreStatus {
    Active,
    Inactive,
    Closed,
}

/// Payment method configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentMethod {
    /// The payment method type (e.g., "scheme", "klarna").
    pub r#type: Box<str>,
    /// Whether this payment method is enabled.
    pub enabled: bool,
    /// Payment method-specific configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<PaymentMethodConfiguration>,
    /// Supported countries for this payment method.
    #[serde(default)]
    pub countries: Vec<Box<str>>,
    /// Supported currencies for this payment method.
    #[serde(default)]
    pub currencies: Vec<Box<str>>,
}

/// Payment method configuration details.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentMethodConfiguration {
    /// Merchant identifier for this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_id: Option<Box<str>>,
    /// API credentials for this payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_credentials: Option<HashMap<String, Box<str>>>,
    /// Payment method-specific properties.
    #[serde(flatten)]
    pub properties: HashMap<String, serde_json::Value>,
}

/// Payment method settings for a store or merchant.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentMethodSettings {
    /// The store or merchant ID these settings apply to.
    pub id: Box<str>,
    /// List of configured payment methods.
    #[serde(default)]
    pub payment_methods: Vec<PaymentMethod>,
}

/// Request to update payment method settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePaymentMethodRequest {
    /// The payment method type to update.
    pub r#type: Box<str>,
    /// Whether to enable or disable this payment method.
    pub enabled: bool,
    /// Updated configuration for the payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<PaymentMethodConfiguration>,
    /// Countries where this payment method should be available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub countries: Option<Vec<Box<str>>>,
    /// Currencies this payment method should support.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currencies: Option<Vec<Box<str>>>,
}

/// Webhook configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Webhook {
    /// The unique identifier of the webhook.
    pub id: Box<str>,
    /// The webhook URL that will receive notifications.
    pub url: Box<str>,
    /// The webhook description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Box<str>>,
    /// Whether the webhook is active.
    pub active: bool,
    /// The communication format (e.g., "soap", "json").
    pub communication_format: Box<str>,
    /// List of event types this webhook subscribes to.
    #[serde(default)]
    pub filter_merchant_accounts: Vec<Box<str>>,
    /// Additional HTTP headers to include.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_settings: Option<WebhookAdditionalSettings>,
    /// Links to related resources.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Links>,
}

/// Request to create a new webhook.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateWebhookRequest {
    /// The webhook URL that will receive notifications.
    pub url: Box<str>,
    /// The webhook description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Box<str>>,
    /// Whether the webhook should be active.
    pub active: bool,
    /// The communication format (e.g., "soap", "json").
    pub communication_format: Box<str>,
    /// List of merchant accounts to filter events for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_merchant_accounts: Option<Vec<Box<str>>>,
    /// Additional webhook settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_settings: Option<WebhookAdditionalSettings>,
}

/// Request to update an existing webhook.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateWebhookRequest {
    /// The webhook URL that will receive notifications.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Box<str>>,
    /// The webhook description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Box<str>>,
    /// Whether the webhook should be active.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// The communication format (e.g., "soap", "json").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub communication_format: Option<Box<str>>,
    /// List of merchant accounts to filter events for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_merchant_accounts: Option<Vec<Box<str>>>,
    /// Additional webhook settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_settings: Option<WebhookAdditionalSettings>,
}

/// Additional settings for webhooks.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebhookAdditionalSettings {
    /// Additional HTTP headers to include in webhook requests.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, Box<str>>>,
    /// Include event codes in webhook notifications.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_event_codes: Option<Vec<Box<str>>>,
    /// Exclude event codes from webhook notifications.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_event_codes: Option<Vec<Box<str>>>,
}

/// Terminal model information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TerminalModel {
    /// The terminal model identifier.
    pub id: Box<str>,
    /// The name of the terminal model.
    pub name: Box<str>,
    /// The manufacturer of the terminal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<Box<str>>,
    /// Whether this model supports contactless payments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contactless: Option<bool>,
    /// Maximum transaction amount for contactless.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contactless_limit: Option<i64>,
}

/// Terminal settings and configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TerminalSettings {
    /// Card acquisition settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_acquisition: Option<CardAcquisitionSettings>,
    /// Connectivity settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connectivity: Option<ConnectivitySettings>,
    /// Receipt settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_options: Option<ReceiptOptions>,
    /// Gratuity settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gratuity: Option<GratuitySettings>,
}

/// Card acquisition settings for terminals.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardAcquisitionSettings {
    /// Operation mode for card acquisition.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<Box<str>>,
    /// Timeout for card acquisition in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,
}

/// Connectivity settings for terminals.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectivitySettings {
    /// Ethernet settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ethernet: Option<EthernetSettings>,
    /// WiFi settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wifi: Option<WiFiSettings>,
}

/// Ethernet connectivity settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EthernetSettings {
    /// Whether DHCP is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dhcp: Option<bool>,
    /// Static IP address if DHCP is disabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<Box<str>>,
}

/// WiFi connectivity settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WiFiSettings {
    /// WiFi network SSID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssid: Option<Box<str>>,
    /// WiFi security type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_type: Option<Box<str>>,
}

/// Receipt printing options.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReceiptOptions {
    /// Whether to print merchant receipts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_receipt: Option<bool>,
    /// Whether to print shopper receipts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shopper_receipt: Option<bool>,
}

/// Gratuity/tip settings for terminals.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GratuitySettings {
    /// Whether gratuity is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Predefined gratuity amounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predefined_tip_entries: Option<Vec<i64>>,
}

/// Terminal information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Terminal {
    /// The terminal ID.
    pub id: Box<str>,
    /// The terminal serial number.
    pub serial_number: Box<str>,
    /// The terminal model.
    pub model: Box<str>,
    /// The store this terminal is assigned to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store_id: Option<Box<str>>,
    /// The terminal status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TerminalStatus>,
    /// Terminal assignment details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignment: Option<TerminalAssignment>,
}

/// Terminal assignment information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TerminalAssignment {
    /// The company ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_id: Option<Box<str>>,
    /// The merchant ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_id: Option<Box<str>>,
    /// The store ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store_id: Option<Box<str>>,
    /// Assignment status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Box<str>>,
}

/// Terminal status.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum TerminalStatus {
    Active,
    Inactive,
    Boarded,
    DeliveryPending,
    Delivered,
    Assigned,
}

/// Contact information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contact {
    /// The contact's email address.
    pub email: Box<str>,
    /// The contact's first name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<Box<str>>,
    /// The contact's last name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<Box<str>>,
    /// The contact's phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<Box<str>>,
}

/// Address information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    /// Street address line 1.
    pub street_address: Box<str>,
    /// Street address line 2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street_address2: Option<Box<str>>,
    /// City name.
    pub city: Box<str>,
    /// State or province.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_or_province: Option<Box<str>>,
    /// Postal code.
    pub postal_code: Box<str>,
    /// Two-letter country code.
    pub country: Box<str>,
}

/// Links to related resources.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Links {
    /// Link to the resource itself.
    #[serde(skip_serializing_if = "Option::is_none", rename = "self")]
    pub self_link: Option<Box<str>>,
    /// Link to the parent resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<Box<str>>,
    /// Links to related resources.
    #[serde(flatten)]
    pub additional_links: HashMap<String, Box<str>>,
}

// Builder implementations for easier API usage
impl CreateMerchantRequest {
    /// Create a new builder for CreateMerchantRequest.
    pub fn builder() -> CreateMerchantRequestBuilder {
        CreateMerchantRequestBuilder::default()
    }
}

/// Builder for CreateMerchantRequest.
#[derive(Default)]
pub struct CreateMerchantRequestBuilder {
    company_id: Option<Box<str>>,
    merchant_account: Option<Box<str>>,
    business_details: Option<BusinessDetails>,
    primary_contact: Option<Contact>,
    billing_address: Option<Address>,
}

impl CreateMerchantRequestBuilder {
    /// Set the company ID.
    pub fn company_id(mut self, company_id: impl Into<Box<str>>) -> Self {
        self.company_id = Some(company_id.into());
        self
    }

    /// Set the merchant account name.
    pub fn merchant_account(mut self, merchant_account: impl Into<Box<str>>) -> Self {
        self.merchant_account = Some(merchant_account.into());
        self
    }

    /// Set the business details.
    pub fn business_details(mut self, business_details: BusinessDetails) -> Self {
        self.business_details = Some(business_details);
        self
    }

    /// Set the primary contact.
    pub fn primary_contact(mut self, primary_contact: Contact) -> Self {
        self.primary_contact = Some(primary_contact);
        self
    }

    /// Set the billing address.
    pub fn billing_address(mut self, billing_address: Address) -> Self {
        self.billing_address = Some(billing_address);
        self
    }

    /// Build the CreateMerchantRequest.
    pub fn build(self) -> Result<CreateMerchantRequest> {
        let company_id = self.company_id
            .ok_or_else(|| AdyenError::generic("company_id is required"))?;
        let merchant_account = self.merchant_account
            .ok_or_else(|| AdyenError::generic("merchant_account is required"))?;
        let business_details = self.business_details
            .ok_or_else(|| AdyenError::generic("business_details is required"))?;
        let primary_contact = self.primary_contact
            .ok_or_else(|| AdyenError::generic("primary_contact is required"))?;

        Ok(CreateMerchantRequest {
            company_id,
            merchant_account,
            business_details,
            primary_contact,
            billing_address: self.billing_address,
        })
    }
}

impl CreateStoreRequest {
    /// Create a new builder for CreateStoreRequest.
    pub fn builder() -> CreateStoreRequestBuilder {
        CreateStoreRequestBuilder::default()
    }
}

/// Builder for CreateStoreRequest.
#[derive(Default)]
pub struct CreateStoreRequestBuilder {
    store_reference: Option<Box<str>>,
    description: Option<Box<str>>,
    address: Option<Address>,
    phone_number: Option<Box<str>>,
    business_line_id: Option<Box<str>>,
}

impl CreateStoreRequestBuilder {
    /// Set the store reference.
    pub fn store_reference(mut self, store_reference: impl Into<Box<str>>) -> Self {
        self.store_reference = Some(store_reference.into());
        self
    }

    /// Set the store description.
    pub fn description(mut self, description: impl Into<Box<str>>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set the store address.
    pub fn address(mut self, address: Address) -> Self {
        self.address = Some(address);
        self
    }

    /// Set the store phone number.
    pub fn phone_number(mut self, phone_number: impl Into<Box<str>>) -> Self {
        self.phone_number = Some(phone_number.into());
        self
    }

    /// Set the business line ID.
    pub fn business_line_id(mut self, business_line_id: impl Into<Box<str>>) -> Self {
        self.business_line_id = Some(business_line_id.into());
        self
    }

    /// Build the CreateStoreRequest.
    pub fn build(self) -> Result<CreateStoreRequest> {
        let store_reference = self.store_reference
            .ok_or_else(|| AdyenError::generic("store_reference is required"))?;
        let description = self.description
            .ok_or_else(|| AdyenError::generic("description is required"))?;
        let address = self.address
            .ok_or_else(|| AdyenError::generic("address is required"))?;

        Ok(CreateStoreRequest {
            store_reference,
            description,
            address,
            phone_number: self.phone_number,
            business_line_id: self.business_line_id,
        })
    }
}