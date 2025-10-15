//! Types for the Adyen Legal Entity API v3.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// Legal Entity Management
// ============================================================================

/// Legal entity representing individuals or organizations for KYC purposes.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LegalEntity {
    /// Unique identifier of the legal entity.
    pub id: Box<str>,
    /// Type of legal entity.
    pub r#type: LegalEntityType,
    /// Individual details if applicable.
    pub individual: Option<Individual>,
    /// Organization details if applicable.
    pub organization: Option<Organization>,
    /// Sole proprietorship details if applicable.
    pub sole_proprietorship: Option<SoleProprietorship>,
    /// Trust details if applicable.
    pub trust: Option<Trust>,
    /// Unincorporated partnership details if applicable.
    pub unincorporated_partnership: Option<UnincorporatedPartnership>,
    /// Verification status and deadlines.
    pub verification: Option<VerificationErrors>,
    /// Entity capabilities and their status.
    pub capabilities: Option<HashMap<String, LegalEntityCapability>>,
    /// Entity associations (beneficial owners, representatives, etc.).
    pub entity_associations: Option<Vec<LegalEntityAssociation>>,
    /// Reference provided by the client.
    pub reference: Option<Box<str>>,
    /// Problems that need to be resolved.
    pub problems: Option<Vec<CapabilityProblem>>,
}

/// Request to create or update a legal entity.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LegalEntityInfo {
    /// Type of legal entity.
    pub r#type: LegalEntityType,
    /// Individual details if applicable.
    pub individual: Option<Individual>,
    /// Organization details if applicable.
    pub organization: Option<Organization>,
    /// Sole proprietorship details if applicable.
    pub sole_proprietorship: Option<SoleProprietorship>,
    /// Trust details if applicable.
    pub trust: Option<Trust>,
    /// Unincorporated partnership details if applicable.
    pub unincorporated_partnership: Option<UnincorporatedPartnership>,
    /// Reference provided by the client.
    pub reference: Option<Box<str>>,
}

/// Type of legal entity.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum LegalEntityType {
    Individual,
    Organization,
    SoleProprietorship,
    Trust,
    UnincorporatedPartnership,
}

// ============================================================================
// Individual Details
// ============================================================================

/// Individual person details for KYC.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Individual {
    /// Full name of the individual.
    pub name: Name,
    /// Email address.
    pub email: Option<Box<str>>,
    /// Phone number.
    pub phone: Option<PhoneNumber>,
    /// Date and place of birth.
    pub birth_data: Option<BirthData>,
    /// Nationality and identification.
    pub nationality: Option<Box<str>>,
    /// Identification documents.
    pub identification_data: Option<IdentificationData>,
    /// Residential address.
    pub residential_address: Option<Address>,
    /// Tax information.
    pub tax_information: Option<Vec<TaxInformation>>,
}

/// Full name details.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Name {
    /// First name.
    pub first_name: Box<str>,
    /// Last name.
    pub last_name: Box<str>,
    /// In-fix (middle names, particles).
    pub in_fix: Option<Box<str>>,
}

/// Phone number information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PhoneNumber {
    /// Phone number including country code.
    pub number: Box<str>,
    /// Type of phone number.
    pub r#type: PhoneType,
}

/// Type of phone number.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PhoneType {
    Landline,
    Mobile,
    Fax,
}

/// Birth data including date and place.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BirthData {
    /// Date of birth (YYYY-MM-DD format).
    pub date_of_birth: Box<str>,
    /// City or town of birth.
    pub city_of_birth: Option<Box<str>>,
    /// Country of birth (ISO 3166-1 alpha-2).
    pub country_of_birth: Option<Box<str>>,
    /// State or province of birth.
    pub state_or_province_of_birth: Option<Box<str>>,
}

/// Identification document data.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IdentificationData {
    /// Type of identification document.
    pub r#type: IdentificationType,
    /// Document number.
    pub number: Box<str>,
    /// Issuer country (ISO 3166-1 alpha-2).
    pub issuer_country: Option<Box<str>>,
    /// Issuer state or province.
    pub issuer_state: Option<Box<str>>,
    /// Expiry date (YYYY-MM-DD format).
    pub expiry_date: Option<Box<str>>,
}

/// Type of identification document.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum IdentificationType {
    DriversLicense,
    IdentityCard,
    NationalIdNumber,
    Passport,
    SocialSecurityNumber,
    TaxId,
}

// ============================================================================
// Organization Details
// ============================================================================

/// Organization details for business entities.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Organization {
    /// Legal business name.
    pub legal_name: Box<str>,
    /// Trading name or DBA.
    pub trading_name: Option<Box<str>>,
    /// Business registration information.
    pub registration_number: Option<Box<str>>,
    /// Tax ID or VAT number.
    pub tax_id: Option<Box<str>>,
    /// Date of incorporation.
    pub date_of_incorporation: Option<Box<str>>,
    /// Type of organization.
    pub r#type: Option<OrganizationType>,
    /// VAT number.
    pub vat_number: Option<Box<str>>,
    /// VAT exemption details.
    pub vat_exemption: Option<VatExemption>,
    /// Registered address.
    pub registered_address: Option<Address>,
    /// Principal business address.
    pub principal_business_address: Option<Address>,
    /// Email address.
    pub email: Option<Box<str>>,
    /// Phone number.
    pub phone: Option<PhoneNumber>,
    /// Website URL.
    pub web_data: Option<WebData>,
    /// Stock exchange information if public.
    pub stock_data: Option<StockData>,
    /// Tax information.
    pub tax_information: Option<Vec<TaxInformation>>,
}

/// Type of organization.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum OrganizationType {
    AssociationIncorporated,
    GovernmentalOrganization,
    LimitedLiabilityCompany,
    LimitedLiabilityPartnership,
    NonProfitCorporation,
    ProfessionalCorporation,
    PublicLimitedCompany,
    RegisteredCharity,
}

/// VAT exemption information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VatExemption {
    /// Reason for VAT exemption.
    pub reason: Box<str>,
    /// Supporting documentation.
    pub domestic_exemption: Option<bool>,
}

/// Website and online presence information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebData {
    /// Website URL.
    pub website_url: Option<Box<str>>,
    /// Web data exemption if applicable.
    pub web_data_exemption: Option<WebDataExemption>,
}

/// Exemption from providing web data.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebDataExemption {
    /// Reason for exemption.
    pub reason: Box<str>,
}

/// Stock exchange information for public companies.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StockData {
    /// Market where the stock is traded.
    pub market: Option<Box<str>>,
    /// Stock ticker symbol.
    pub ticker_symbol: Option<Box<str>>,
    /// International Securities Identification Number.
    pub isin: Option<Box<str>>,
    /// Committee on Uniform Securities Identification Procedures number.
    pub cusip: Option<Box<str>>,
}

// ============================================================================
// Sole Proprietorship Details
// ============================================================================

/// Sole proprietorship business details.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SoleProprietorship {
    /// Principal business address.
    pub principal_business_address: Option<Address>,
    /// Business registration information.
    pub registration_number: Option<Box<str>>,
    /// Tax ID.
    pub tax_id: Option<Box<str>>,
    /// Date business was registered.
    pub date_of_birth: Option<Box<str>>,
    /// Trading name.
    pub trading_name: Option<Box<str>>,
    /// VAT number.
    pub vat_number: Option<Box<str>>,
    /// VAT exemption details.
    pub vat_exemption: Option<VatExemption>,
}

// ============================================================================
// Trust Details
// ============================================================================

/// Trust entity details.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Trust {
    /// Name of the trust.
    pub name: Box<str>,
    /// Type of trust.
    pub r#type: Option<TrustType>,
    /// Date the trust was established.
    pub date_of_settlement: Option<Box<str>>,
    /// Country where the trust is governed.
    pub country_of_governance: Option<Box<str>>,
    /// Principal place of business.
    pub principal_place_of_business: Option<Address>,
    /// Tax information.
    pub tax_information: Option<Vec<TaxInformation>>,
    /// Source of funds for the trust.
    pub source_of_funds: Option<SourceOfFunds>,
    /// Undefined beneficiary information.
    pub undefined_beneficiary: Option<UndefinedBeneficiary>,
}

/// Type of trust structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TrustType {
    RevocableTrust,
    IrrevocableTrust,
    TrustCorporation,
}

/// Source of funds information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceOfFunds {
    /// Type of funds source.
    pub r#type: FundsSourceType,
    /// Description of the funds source.
    pub description: Option<Box<str>>,
}

/// Type of funds source.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum FundsSourceType {
    Business,
    Donation,
    Inheritance,
    Investment,
    Loan,
    Other,
    Pension,
    Salary,
}

/// Undefined beneficiary information for trusts.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UndefinedBeneficiary {
    /// Description of undefined beneficiaries.
    pub description: Option<Box<str>>,
}

// ============================================================================
// Unincorporated Partnership Details
// ============================================================================

/// Unincorporated partnership entity details.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnincorporatedPartnership {
    /// Partnership name.
    pub name: Box<str>,
    /// Country of partnership.
    pub country_of_governance: Option<Box<str>>,
    /// Date partnership was established.
    pub date_of_incorporation: Option<Box<str>>,
    /// Principal place of business.
    pub principal_place_of_business: Option<Address>,
    /// Registered address.
    pub registered_address: Option<Address>,
    /// Registration number.
    pub registration_number: Option<Box<str>>,
    /// Tax ID.
    pub tax_id: Option<Box<str>>,
    /// VAT number.
    pub vat_number: Option<Box<str>>,
    /// VAT exemption details.
    pub vat_exemption: Option<VatExemption>,
    /// Tax information.
    pub tax_information: Option<Vec<TaxInformation>>,
    /// Source of funds.
    pub source_of_funds: Option<SourceOfFunds>,
}

// ============================================================================
// Address and Location
// ============================================================================

/// Address information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    /// Country code (ISO 3166-1 alpha-2).
    pub country: Box<str>,
    /// City name.
    pub city: Option<Box<str>>,
    /// Postal code.
    pub postal_code: Option<Box<str>>,
    /// State or province.
    pub state_or_province: Option<Box<str>>,
    /// Street address line 1.
    pub street_address: Option<Box<str>>,
    /// Street address line 2.
    pub street_address2: Option<Box<str>>,
}

// ============================================================================
// Tax Information
// ============================================================================

/// Tax classification and reporting information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaxInformation {
    /// Country for tax purposes.
    pub country: Box<str>,
    /// Tax identification number.
    pub number: Option<Box<str>>,
    /// Type of tax number.
    pub r#type: Option<TaxIdType>,
    /// Tax reporting classification.
    pub tax_reporting_classification: Option<TaxReportingClassification>,
}

/// Type of tax identification.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TaxIdType {
    AusTfn,
    BrCpf,
    BrCnpj,
    CaGst,
    CaSin,
    EuVat,
    GbVat,
    HkBr,
    InGstin,
    InPan,
    SgGst,
    SgUen,
    UsSsn,
    UsEin,
    UsTin,
}

/// Tax reporting classification.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaxReportingClassification {
    /// Business type for tax purposes.
    pub business_type: Option<TaxBusinessType>,
    /// Commercial type classification.
    pub commercial_type: Option<TaxCommercialType>,
    /// FATCA/CRS classification.
    pub financial_institution_number: Option<Box<str>>,
}

/// Business type for tax classification.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TaxBusinessType {
    Corporation,
    DisregardedEntity,
    GovernmentEntity,
    Individual,
    LimitedLiability,
    Partnership,
    Trust,
}

/// Commercial type for tax classification.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TaxCommercialType {
    ApplicantExempt,
    CommodityPool,
    DerivativesDealer,
    FinancialInstitution,
    ForexDealer,
    FuturesCommissionMerchant,
    HedgeFund,
    IntroducingBroker,
    NonFinancialEntity,
    PrivateEquityFund,
    RegisteredSecuritiesDealer,
    RetailForexDealer,
    SwapDealer,
}

// ============================================================================
// Verification and Capabilities
// ============================================================================

/// Legal entity capability configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LegalEntityCapability {
    /// Whether the capability is allowed.
    pub allowed: Option<bool>,
    /// Capability status.
    pub allowed_level: Option<CapabilityAllowedLevel>,
    /// Required capability level.
    pub allowed_settings: Option<CapabilitySettings>,
    /// Whether the capability is enabled.
    pub enabled: Option<bool>,
    /// Problems that need resolution.
    pub problems: Option<Vec<CapabilityProblem>>,
    /// Whether the capability is requested.
    pub requested: Option<bool>,
    /// Requested capability level.
    pub requested_level: Option<CapabilityRequestedLevel>,
    /// Requested settings for the capability.
    pub requested_settings: Option<CapabilitySettings>,
    /// Transfer instruments for this capability.
    pub transfer_instruments: Option<Vec<SupportingEntityCapability>>,
    /// Verification status.
    pub verification_status: Option<VerificationStatus>,
}

/// Allowed level for a capability.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CapabilityAllowedLevel {
    High,
    Low,
    Medium,
    NotApplicable,
}

/// Requested level for a capability.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CapabilityRequestedLevel {
    High,
    Low,
    Medium,
    NotApplicable,
}

/// Settings for a capability.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CapabilitySettings {
    /// Amount limits.
    pub amount_per_industry: Option<HashMap<String, Amount>>,
    /// Authorized card users.
    pub authorized_card_users: Option<bool>,
    /// Fund sources.
    pub funding_source: Option<Vec<FundingSource>>,
    /// Interval for limits.
    pub interval: Option<CapabilitySettingsInterval>,
    /// Maximum amount.
    pub max_amount: Option<Amount>,
}

/// Funding source for capabilities.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum FundingSource {
    Credit,
    Debit,
}

/// Interval for capability settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CapabilitySettingsInterval {
    Daily,
    Monthly,
    Weekly,
}

/// Amount with currency.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Amount {
    /// Three-character ISO currency code.
    pub currency: Box<str>,
    /// Amount value in minor units.
    pub value: i64,
}

/// Problem that needs to be resolved for a capability.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CapabilityProblem {
    /// Problem entity details.
    pub entity: Option<CapabilityProblemEntity>,
    /// Problem type.
    pub r#type: Option<CapabilityProblemType>,
    /// Verification errors.
    pub verification_errors: Option<Vec<VerificationError>>,
}

/// Entity associated with a capability problem.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CapabilityProblemEntity {
    /// Associated documents.
    pub documents: Option<Vec<Box<str>>>,
    /// Entity ID.
    pub id: Option<Box<str>>,
    /// Entity type.
    pub r#type: Option<LegalEntityType>,
    /// Owner entity if applicable.
    pub owner: Option<OwnerEntity>,
}

/// Owner entity reference.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OwnerEntity {
    /// Owner entity ID.
    pub id: Option<Box<str>>,
    /// Owner entity type.
    pub r#type: Option<LegalEntityType>,
}

/// Type of capability problem.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CapabilityProblemType {
    DataMissing,
    InvalidData,
    VerificationFailed,
}

/// Supporting entity capability.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SupportingEntityCapability {
    /// Allowed level.
    pub allowed_level: Option<CapabilityAllowedLevel>,
    /// Whether enabled.
    pub enabled: Option<bool>,
    /// Entity ID.
    pub entity_id: Option<Box<str>>,
    /// Requested level.
    pub requested_level: Option<CapabilityRequestedLevel>,
    /// Verification status.
    pub verification_status: Option<VerificationStatus>,
}

/// Verification status for entities and capabilities.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum VerificationStatus {
    Pending,
    Valid,
    Invalid,
    Rejected,
}

/// Verification errors collection.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerificationErrors {
    /// List of verification errors.
    pub problems: Option<Vec<VerificationError>>,
}

/// Individual verification error.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerificationError {
    /// Error code.
    pub code: Option<Box<str>>,
    /// Error message.
    pub message: Option<Box<str>>,
    /// Error type.
    pub r#type: Option<VerificationErrorType>,
    /// Remediating actions to resolve the error.
    pub remediating_actions: Option<Vec<RemediatingAction>>,
}

/// Type of verification error.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum VerificationErrorType {
    DataMissing,
    InvalidData,
    PendingReview,
}

/// Action to remediate a verification error.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemediatingAction {
    /// Action code.
    pub code: Option<Box<str>>,
    /// Action message.
    pub message: Option<Box<str>>,
}

// ============================================================================
// Entity Associations
// ============================================================================

/// Association between legal entities (ownership, representation, etc.).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LegalEntityAssociation {
    /// Associated entity ID.
    pub associator_id: Box<str>,
    /// Type of association.
    pub r#type: AssociationType,
    /// Name of the association.
    pub name: Option<Box<str>>,
    /// Job title if applicable.
    pub job_title: Option<Box<str>>,
    /// Entity reference.
    pub entity_ids: Option<Vec<Box<str>>>,
}

/// Type of association between entities.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AssociationType {
    BeneficialOwner,
    Director,
    LegalRepresentative,
    Representative,
    SignatoryAndBeneficialOwner,
    Signatory,
    TrusteeBeneficiary,
    UndefinedBeneficiary,
}

// ============================================================================
// Documents Management
// ============================================================================

/// Document for verification purposes.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Document {
    /// Document ID.
    pub id: Box<str>,
    /// Type of document.
    pub r#type: DocumentType,
    /// Document pages.
    pub pages: Option<Vec<DocumentPage>>,
    /// Document owner.
    pub owner: Option<EntityReference>,
    /// Creation time.
    pub creation_date: Option<Box<str>>,
    /// Modification time.
    pub modification_date: Option<Box<str>>,
    /// Expiry date of the document.
    pub expiry_date: Option<Box<str>>,
    /// Document number or reference.
    pub number: Option<Box<str>>,
    /// Document description.
    pub description: Option<Box<str>>,
    /// File name.
    pub file_name: Option<Box<str>>,
}

/// Type of document for verification.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DocumentType {
    AdditionalRequirement,
    BankStatement,
    BusinessLicense,
    CertificateOfIncorporation,
    CompanyStructure,
    ContractualDocument,
    DirectorsStatement,
    DriversLicense,
    IdentityCard,
    PassportPage,
    ProofOfAddress,
    ProofOfFunds,
    ProofOfIndustry,
    RegistrationDocument,
    ShareholderStructure,
    TaxReturn,
    UboDeclaration,
    UtilityBill,
    VoterIdCard,
}

/// Individual page of a document.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentPage {
    /// Page content (base64 encoded).
    pub content: Box<str>,
    /// Content type (MIME type).
    pub content_type: Option<Box<str>>,
    /// Page number.
    pub page_number: Option<i32>,
}

/// Reference to an entity.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntityReference {
    /// Entity ID.
    pub id: Box<str>,
    /// Entity type.
    pub r#type: Option<LegalEntityType>,
}

// ============================================================================
// Transfer Instruments
// ============================================================================

/// Transfer instrument for moving funds.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferInstrument {
    /// Transfer instrument ID.
    pub id: Box<str>,
    /// Legal entity ID that owns this instrument.
    pub legal_entity_id: Box<str>,
    /// Type of transfer instrument.
    pub r#type: TransferInstrumentType,
    /// Bank account information.
    pub bank_account: Option<BankAccountInfo>,
}

/// Request to create transfer instrument.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferInstrumentInfo {
    /// Legal entity ID.
    pub legal_entity_id: Box<str>,
    /// Type of transfer instrument.
    pub r#type: TransferInstrumentType,
    /// Bank account information.
    pub bank_account: Option<BankAccountInfo>,
}

/// Type of transfer instrument.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TransferInstrumentType {
    BankAccount,
}

/// Bank account information for transfers.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BankAccountInfo {
    /// Account holder name.
    pub account_holder: Box<str>,
    /// Account identification details.
    pub account_identification: BankAccountIdentification,
}

/// Bank account identification methods.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum BankAccountIdentification {
    /// IBAN account identification.
    Iban(IbanAccountIdentification),
    /// US account identification.
    UsLocal(UsLocalAccountIdentification),
    /// UK account identification.
    UkLocal(UkLocalAccountIdentification),
    /// Canadian account identification.
    CaLocal(CaLocalAccountIdentification),
    /// Australian account identification.
    AuLocal(AuLocalAccountIdentification),
    /// Singapore account identification.
    SgLocal(SgLocalAccountIdentification),
    /// Number and BIC identification.
    NumberAndBic(NumberAndBicAccountIdentification),
}

/// IBAN account identification.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IbanAccountIdentification {
    /// International Bank Account Number.
    pub iban: Box<str>,
    /// Type identifier.
    pub r#type: String,
}

/// US local account identification.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsLocalAccountIdentification {
    /// Account number.
    pub account_number: Box<str>,
    /// Account type.
    pub account_type: Option<UsAccountType>,
    /// Routing number.
    pub routing_number: Box<str>,
    /// Type identifier.
    pub r#type: String,
}

/// US bank account type.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum UsAccountType {
    Checking,
    Savings,
}

/// UK local account identification.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UkLocalAccountIdentification {
    /// Account number.
    pub account_number: Box<str>,
    /// Sort code.
    pub sort_code: Box<str>,
    /// Type identifier.
    pub r#type: String,
}

/// Canadian local account identification.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CaLocalAccountIdentification {
    /// Account number.
    pub account_number: Box<str>,
    /// Account type.
    pub account_type: Option<CaAccountType>,
    /// Institution number.
    pub institution_number: Box<str>,
    /// Transit number.
    pub transit_number: Box<str>,
    /// Type identifier.
    pub r#type: String,
}

/// Canadian bank account type.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CaAccountType {
    Checking,
    Savings,
}

/// Australian local account identification.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuLocalAccountIdentification {
    /// Account number.
    pub account_number: Box<str>,
    /// Bank State Branch code.
    pub bsb_code: Box<str>,
    /// Type identifier.
    pub r#type: String,
}

/// Singapore local account identification.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SgLocalAccountIdentification {
    /// Account number.
    pub account_number: Box<str>,
    /// Bank code.
    pub bank_code: Box<str>,
    /// Type identifier.
    pub r#type: String,
}

/// Number and BIC account identification.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NumberAndBicAccountIdentification {
    /// Account number.
    pub account_number: Box<str>,
    /// Bank Identifier Code.
    pub bic: Box<str>,
    /// Type identifier.
    pub r#type: String,
}

// ============================================================================
// Business Lines
// ============================================================================

/// Business line for legal entity operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BusinessLine {
    /// Business line ID.
    pub id: Box<str>,
    /// Legal entity ID.
    pub legal_entity_id: Box<str>,
    /// Industry category.
    pub industry: Box<str>,
    /// Industry code.
    pub industry_code: Option<Box<str>>,
    /// Sales channels.
    pub sales_channels: Option<Vec<SalesChannel>>,
    /// Website information.
    pub web_data: Option<Vec<WebData>>,
    /// Service description.
    pub service: Option<Box<str>>,
    /// Capability problems.
    pub problems: Option<Vec<CapabilityProblem>>,
}

/// Request to create or update business line.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BusinessLineInfo {
    /// Legal entity ID.
    pub legal_entity_id: Box<str>,
    /// Industry category.
    pub industry: Box<str>,
    /// Industry code.
    pub industry_code: Option<Box<str>>,
    /// Sales channels.
    pub sales_channels: Option<Vec<SalesChannel>>,
    /// Website information.
    pub web_data: Option<Vec<WebData>>,
    /// Service description.
    pub service: Option<Box<str>>,
}

/// Sales channel for business operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SalesChannel {
    Ecommerce,
    Pos,
    ContAuth,
    Moto,
}

// ============================================================================
// Hosted Onboarding
// ============================================================================

/// Hosted onboarding link for user self-service.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OnboardingLink {
    /// Unique link ID.
    pub id: Box<str>,
    /// Onboarding URL for the user.
    pub url: Box<str>,
    /// Link expiration time.
    pub expires_at: Option<Box<str>>,
}

/// Request to create onboarding link.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OnboardingLinkInfo {
    /// Legal entity ID.
    pub legal_entity_id: Box<str>,
    /// Onboarding settings.
    pub settings: Option<OnboardingLinkSettings>,
    /// Theme customization.
    pub theme_id: Option<Box<str>>,
}

/// Settings for onboarding links.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OnboardingLinkSettings {
    /// Target entity types to collect.
    pub collect_entity_types: Option<Vec<LegalEntityType>>,
    /// Enable manual review.
    pub enable_manual_review: Option<bool>,
    /// Required verification checks.
    pub required_verification_checks: Option<Vec<VerificationCheckType>>,
}

/// Type of verification check.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum VerificationCheckType {
    CompanyVerification,
    IdentityVerification,
    PassportVerification,
    VisaVerification,
}

/// Onboarding theme customization.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OnboardingTheme {
    /// Theme ID.
    pub id: Box<str>,
    /// Primary color.
    pub primary_color: Option<Box<str>>,
    /// Secondary color.
    pub secondary_color: Option<Box<str>>,
    /// Logo URL.
    pub logo_url: Option<Box<str>>,
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

impl LegalEntityInfo {
    /// Create a new builder for LegalEntityInfo.
    #[must_use]
    pub fn builder() -> LegalEntityInfoBuilder {
        LegalEntityInfoBuilder::default()
    }
}

/// Builder for LegalEntityInfo.
#[derive(Debug, Default)]
pub struct LegalEntityInfoBuilder {
    r#type: Option<LegalEntityType>,
    individual: Option<Individual>,
    organization: Option<Organization>,
    sole_proprietorship: Option<SoleProprietorship>,
    trust: Option<Trust>,
    unincorporated_partnership: Option<UnincorporatedPartnership>,
    reference: Option<Box<str>>,
}

impl LegalEntityInfoBuilder {
    /// Set the legal entity type.
    #[must_use]
    pub fn entity_type(mut self, entity_type: LegalEntityType) -> Self {
        self.r#type = Some(entity_type);
        self
    }

    /// Set individual details.
    #[must_use]
    pub fn individual(mut self, individual: Individual) -> Self {
        self.individual = Some(individual);
        self
    }

    /// Set organization details.
    #[must_use]
    pub fn organization(mut self, organization: Organization) -> Self {
        self.organization = Some(organization);
        self
    }

    /// Set reference.
    #[must_use]
    pub fn reference(mut self, reference: &str) -> Self {
        self.reference = Some(reference.into());
        self
    }

    /// Build the LegalEntityInfo.
    ///
    /// # Errors
    ///
    /// Returns an error if required fields are missing.
    pub fn build(self) -> Result<LegalEntityInfo, Box<str>> {
        let entity_type = self.r#type.ok_or("entity type is required")?;

        Ok(LegalEntityInfo {
            r#type: entity_type,
            individual: self.individual,
            organization: self.organization,
            sole_proprietorship: self.sole_proprietorship,
            trust: self.trust,
            unincorporated_partnership: self.unincorporated_partnership,
            reference: self.reference,
        })
    }
}
