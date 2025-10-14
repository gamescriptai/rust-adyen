//! Types for the Adyen Balance Platform API v2.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// Balance Account Management
// ============================================================================

/// Balance account configuration for marketplace operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BalanceAccount {
    /// Unique identifier of the balance account.
    pub id: Box<str>,
    /// Description of the balance account.
    pub description: Option<Box<str>>,
    /// Default currency code.
    pub default_currency_code: Option<Box<str>>,
    /// Account holder details.
    pub account_holder: AccountHolder,
    /// Current balances by currency.
    pub balances: Option<Vec<Balance>>,
    /// Metadata about the balance account.
    pub metadata: Option<HashMap<String, String>>,
    /// Status of the balance account.
    pub status: BalanceAccountStatus,
    /// Time zone for the balance account.
    pub time_zone: Option<Box<str>>,
}

/// Request to create a new balance account.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateBalanceAccountRequest {
    /// Account holder identifier.
    pub account_holder_id: Box<str>,
    /// Description of the balance account.
    pub description: Option<Box<str>>,
    /// Reference for the balance account.
    pub reference: Option<Box<str>>,
    /// Default currency code.
    pub default_currency_code: Option<Box<str>>,
    /// Metadata about the balance account.
    pub metadata: Option<HashMap<String, String>>,
    /// Time zone for the balance account.
    pub time_zone: Option<Box<str>>,
}

/// Current balance information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Balance {
    /// Three-character ISO currency code.
    pub currency: Box<str>,
    /// Available balance amount.
    pub available: i64,
    /// Pending balance amount.
    pub pending: Option<i64>,
    /// Reserved balance amount.
    pub reserved: Option<i64>,
}

/// Status of a balance account.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BalanceAccountStatus {
    Active,
    Closed,
    Inactive,
    Suspended,
}

// ============================================================================
// Account Holder Management
// ============================================================================

/// Account holder details.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountHolder {
    /// Unique identifier of the account holder.
    pub id: Box<str>,
    /// Reference provided during account holder creation.
    pub reference: Option<Box<str>>,
    /// Legal entity identifier.
    pub legal_entity_id: Box<str>,
    /// Description of the account holder.
    pub description: Option<Box<str>>,
    /// Status of the account holder.
    pub status: AccountHolderStatus,
    /// Capabilities of the account holder.
    pub capabilities: Option<HashMap<String, AccountHolderCapability>>,
    /// Contact details for the account holder.
    pub contact_details: Option<ContactDetails>,
    /// Time zone for the account holder.
    pub time_zone: Option<Box<str>>,
    /// Metadata about the account holder.
    pub metadata: Option<HashMap<String, String>>,
}

/// Request to create a new account holder.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateAccountHolderRequest {
    /// Legal entity identifier.
    pub legal_entity_id: Box<str>,
    /// Reference for the account holder.
    pub reference: Option<Box<str>>,
    /// Description of the account holder.
    pub description: Option<Box<str>>,
    /// Contact details for the account holder.
    pub contact_details: Option<ContactDetails>,
    /// Time zone for the account holder.
    pub time_zone: Option<Box<str>>,
    /// Metadata about the account holder.
    pub metadata: Option<HashMap<String, String>>,
}

/// Status of an account holder.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AccountHolderStatus {
    Active,
    Inactive,
    Suspended,
    Closed,
}

/// Account holder capability configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountHolderCapability {
    /// Whether the capability is allowed.
    pub allowed: bool,
    /// Verification status of the capability.
    pub verification_status: Option<VerificationStatus>,
    /// Settings for the capability.
    pub settings: Option<CapabilitySettings>,
}

/// Verification status for capabilities.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum VerificationStatus {
    Pending,
    Valid,
    Invalid,
    Rejected,
}

/// Settings for account holder capabilities.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CapabilitySettings {
    /// Whether the capability requires additional documentation.
    pub requires_additional_documentation: Option<bool>,
    /// Document types required for verification.
    pub document_types: Option<Vec<Box<str>>>,
    /// Supported countries for the capability.
    pub enabled_card_types: Option<Vec<Box<str>>>,
}

// ============================================================================
// Payment Instrument Management
// ============================================================================

/// Payment instrument configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentInstrument {
    /// Unique identifier of the payment instrument.
    pub id: Box<str>,
    /// Type of payment instrument.
    pub r#type: PaymentInstrumentType,
    /// Status of the payment instrument.
    pub status: PaymentInstrumentStatus,
    /// Associated balance account ID.
    pub balance_account_id: Box<str>,
    /// Description of the payment instrument.
    pub description: Option<Box<str>>,
    /// Reference provided during creation.
    pub reference: Option<Box<str>>,
    /// Card details if applicable.
    pub card: Option<Card>,
    /// Bank account details if applicable.
    pub bank_account: Option<BankAccount>,
}

/// Request to create a new payment instrument.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePaymentInstrumentRequest {
    /// Balance account ID to associate with.
    pub balance_account_id: Box<str>,
    /// Type of payment instrument.
    pub r#type: PaymentInstrumentType,
    /// Description of the payment instrument.
    pub description: Option<Box<str>>,
    /// Reference for the payment instrument.
    pub reference: Option<Box<str>>,
    /// Card details if creating a card.
    pub card: Option<CreateCardRequest>,
    /// Bank account details if creating a bank account.
    pub bank_account: Option<CreateBankAccountRequest>,
}

/// Type of payment instrument.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PaymentInstrumentType {
    BankAccount,
    Card,
}

/// Status of a payment instrument.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PaymentInstrumentStatus {
    Active,
    Closed,
    Inactive,
    Suspended,
}

/// Card details for payment instruments.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Card {
    /// Card brand (e.g., visa, mastercard).
    pub brand: Box<str>,
    /// Brand variant of the card.
    pub brand_variant: Option<Box<str>>,
    /// Expiry month of the card.
    pub expiry_month: u8,
    /// Expiry year of the card.
    pub expiry_year: u16,
    /// Form factor of the card.
    pub form_factor: Option<CardFormFactor>,
    /// Last four digits of the card number.
    pub last_four: Box<str>,
    /// BIN (Bank Identification Number) of the card.
    pub bin: Option<Box<str>>,
}

/// Request to create a new card.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCardRequest {
    /// Card brand to create.
    pub brand: Box<str>,
    /// Brand variant for the card.
    pub brand_variant: Option<Box<str>>,
    /// Form factor of the card.
    pub form_factor: Option<CardFormFactor>,
    /// Currency for the card.
    pub currency: Option<Box<str>>,
}

/// Form factor options for cards.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CardFormFactor {
    Physical,
    Virtual,
}

/// Bank account details for payment instruments.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BankAccount {
    /// Account number.
    pub account_number: Box<str>,
    /// Routing number or sort code.
    pub routing_number: Option<Box<str>>,
    /// IBAN (International Bank Account Number).
    pub iban: Option<Box<str>>,
    /// BIC/SWIFT code.
    pub bic: Option<Box<str>>,
    /// Account holder name.
    pub account_holder_name: Box<str>,
    /// Bank name.
    pub bank_name: Option<Box<str>>,
    /// Country code where the bank is located.
    pub country_code: Box<str>,
    /// Currency of the bank account.
    pub currency: Box<str>,
}

/// Request to create a new bank account.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateBankAccountRequest {
    /// Account number.
    pub account_number: Box<str>,
    /// Routing number or sort code.
    pub routing_number: Option<Box<str>>,
    /// IBAN (International Bank Account Number).
    pub iban: Option<Box<str>>,
    /// BIC/SWIFT code.
    pub bic: Option<Box<str>>,
    /// Account holder name.
    pub account_holder_name: Box<str>,
    /// Bank name.
    pub bank_name: Option<Box<str>>,
    /// Country code where the bank is located.
    pub country_code: Box<str>,
    /// Currency of the bank account.
    pub currency: Box<str>,
}

// ============================================================================
// Transaction Rules Management
// ============================================================================

/// Transaction rule configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionRule {
    /// Unique identifier of the transaction rule.
    pub id: Box<str>,
    /// Description of the transaction rule.
    pub description: Option<Box<str>>,
    /// Reference provided during creation.
    pub reference: Option<Box<str>>,
    /// Status of the transaction rule.
    pub status: TransactionRuleStatus,
    /// Type of transaction rule.
    pub r#type: TransactionRuleType,
    /// Rule intervals (velocity limits).
    pub rule_restrictions: TransactionRuleRestrictions,
    /// Entity level where the rule applies.
    pub entity_key: EntityKey,
    /// Outcome when rule is triggered.
    pub outcome_type: OutcomeType,
}

/// Request to create a new transaction rule.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTransactionRuleRequest {
    /// Description of the transaction rule.
    pub description: Option<Box<str>>,
    /// Reference for the transaction rule.
    pub reference: Option<Box<str>>,
    /// Type of transaction rule.
    pub r#type: TransactionRuleType,
    /// Rule restrictions to apply.
    pub rule_restrictions: TransactionRuleRestrictions,
    /// Entity where the rule applies.
    pub entity_key: EntityKey,
    /// Outcome when rule is triggered.
    pub outcome_type: OutcomeType,
}

/// Status of a transaction rule.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TransactionRuleStatus {
    Active,
    Inactive,
}

/// Type of transaction rule.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TransactionRuleType {
    Velocity,
    BlockList,
    AllowList,
}

/// Transaction rule restrictions configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionRuleRestrictions {
    /// Maximum transaction amount restrictions.
    pub max_amount: Option<Amount>,
    /// Velocity restrictions (limits over time periods).
    pub velocity: Option<VelocityRestriction>,
    /// Geographic restrictions.
    pub processing_types: Option<ProcessingTypesRestriction>,
    /// Time-based restrictions.
    pub time_period: Option<TimePeriodRestriction>,
}

/// Entity key for transaction rule application.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntityKey {
    /// Type of entity.
    pub entity_type: EntityType,
    /// Entity reference (balance account ID, etc.).
    pub entity_reference: Box<str>,
}

/// Type of entity for transaction rules.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum EntityType {
    BalanceAccount,
    PaymentInstrument,
    AccountHolder,
}

/// Outcome type when transaction rule is triggered.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum OutcomeType {
    HardBlock,
    AdviseOnly,
    AskAcquirer,
}

/// Amount restriction configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Amount {
    /// Currency code.
    pub currency: Box<str>,
    /// Amount value in minor units.
    pub value: i64,
}

/// Velocity restriction configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VelocityRestriction {
    /// Maximum number of transactions allowed.
    pub max_amount: Option<Amount>,
    /// Time period for the velocity check.
    pub time_period: TimePeriod,
}

/// Processing types restriction.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessingTypesRestriction {
    /// Processing types to include or exclude.
    pub types: Vec<ProcessingType>,
    /// Whether to include or exclude the specified types.
    pub operation: RestrictionOperation,
}

/// Time period restriction configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimePeriodRestriction {
    /// Start time for restrictions.
    pub start_time: Option<Box<str>>,
    /// End time for restrictions.
    pub end_time: Option<Box<str>>,
    /// Time zone for the restriction.
    pub time_zone: Option<Box<str>>,
}

/// Time period options for velocity restrictions.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TimePeriod {
    Daily,
    Weekly,
    Monthly,
    Lifetime,
}

/// Processing type categories.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ProcessingType {
    Pos,
    Ecommerce,
    ContAuth,
    Moto,
}

/// Restriction operation type.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RestrictionOperation {
    Include,
    Exclude,
}

// ============================================================================
// Common Types
// ============================================================================

/// Contact details for account holders.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContactDetails {
    /// Email address.
    pub email: Option<Box<str>>,
    /// Phone number.
    pub phone: Option<Phone>,
    /// Address details.
    pub address: Option<Address>,
    /// Website URL.
    pub website_url: Option<Box<str>>,
}

/// Phone number details.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Phone {
    /// Phone number.
    pub number: Box<str>,
    /// Phone number type.
    pub r#type: PhoneType,
}

/// Type of phone number.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PhoneType {
    Mobile,
    Landline,
    Fax,
}

/// Address details.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    /// Country code (ISO 3166-1 alpha-2).
    pub country: Box<str>,
    /// City name.
    pub city: Option<Box<str>>,
    /// Postal code.
    pub postal_code: Option<Box<str>>,
    /// Province, state, or region.
    pub state_or_province: Option<Box<str>>,
    /// Street address line 1.
    pub street_address: Option<Box<str>>,
    /// Street address line 2.
    pub street_address2: Option<Box<str>>,
}

// ============================================================================
// Response Types
// ============================================================================

/// Response wrapper for paginated results.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaginatedResponse<T> {
    /// Array of result items.
    pub data: Vec<T>,
    /// Pagination information.
    pub has_next: Option<bool>,
    /// Pagination information.
    pub has_previous: Option<bool>,
}

// ============================================================================
// Builder Implementations
// ============================================================================

impl CreateBalanceAccountRequest {
    /// Create a new builder for CreateBalanceAccountRequest.
    #[must_use]
    pub fn builder() -> CreateBalanceAccountRequestBuilder {
        CreateBalanceAccountRequestBuilder::default()
    }
}

/// Builder for CreateBalanceAccountRequest.
#[derive(Debug, Default)]
pub struct CreateBalanceAccountRequestBuilder {
    account_holder_id: Option<Box<str>>,
    description: Option<Box<str>>,
    reference: Option<Box<str>>,
    default_currency_code: Option<Box<str>>,
    metadata: Option<HashMap<String, String>>,
    time_zone: Option<Box<str>>,
}

impl CreateBalanceAccountRequestBuilder {
    /// Set the account holder ID.
    #[must_use]
    pub fn account_holder_id(mut self, account_holder_id: &str) -> Self {
        self.account_holder_id = Some(account_holder_id.into());
        self
    }

    /// Set the description.
    #[must_use]
    pub fn description(mut self, description: &str) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set the reference.
    #[must_use]
    pub fn reference(mut self, reference: &str) -> Self {
        self.reference = Some(reference.into());
        self
    }

    /// Set the default currency code.
    #[must_use]
    pub fn default_currency_code(mut self, currency: &str) -> Self {
        self.default_currency_code = Some(currency.into());
        self
    }

    /// Set metadata.
    #[must_use]
    pub fn metadata(mut self, metadata: HashMap<String, String>) -> Self {
        self.metadata = Some(metadata);
        self
    }

    /// Set the time zone.
    #[must_use]
    pub fn time_zone(mut self, time_zone: &str) -> Self {
        self.time_zone = Some(time_zone.into());
        self
    }

    /// Build the CreateBalanceAccountRequest.
    ///
    /// # Errors
    ///
    /// Returns an error if required fields are missing.
    pub fn build(self) -> Result<CreateBalanceAccountRequest, Box<str>> {
        let account_holder_id = self.account_holder_id
            .ok_or("account_holder_id is required")?;

        Ok(CreateBalanceAccountRequest {
            account_holder_id,
            description: self.description,
            reference: self.reference,
            default_currency_code: self.default_currency_code,
            metadata: self.metadata,
            time_zone: self.time_zone,
        })
    }
}