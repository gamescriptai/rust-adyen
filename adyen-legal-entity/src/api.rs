//! Legal Entity API client implementation.

use crate::types::*;
use adyen_core::{Client, Config, Result};

/// Adyen Legal Entity API client.
///
/// Provides access to Adyen's Legal Entity API v3 for KYC and onboarding workflows,
/// including legal entity management, document verification, business lines, and compliance.
///
/// # Example
///
/// ```rust
/// use adyen_core::{ConfigBuilder, Environment};
/// use adyen_legal_entity::LegalEntityApi;
///
/// # fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let config = ConfigBuilder::new()
///     .environment(Environment::test())
///     .api_key("your_api_key")?
///     .build()?;
///
/// let legal_entity_api = LegalEntityApi::new(config)?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct LegalEntityApi {
    client: Client,
}

impl LegalEntityApi {
    /// Create a new Legal Entity API client with the given configuration.
    ///
    /// # Errors
    ///
    /// Returns an error if the underlying HTTP client cannot be created.
    pub fn new(config: Config) -> Result<Self> {
        let client = Client::new(config)?;
        Ok(Self { client })
    }

    // ============================================================================
    // Legal Entities Management
    // ============================================================================

    /// Create a new legal entity.
    ///
    /// Creates a legal entity for KYC and onboarding workflows.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    ///
    /// # Example
    ///
    /// ```rust
    /// use adyen_legal_entity::{LegalEntityApi, LegalEntityInfo, LegalEntityType, Individual, Name};
    ///
    /// # async fn example(api: LegalEntityApi) -> Result<(), Box<dyn std::error::Error>> {
    /// let individual = Individual {
    ///     name: Name {
    ///         first_name: "John".into(),
    ///         last_name: "Doe".into(),
    ///         in_fix: None,
    ///     },
    ///     email: Some("john.doe@example.com".into()),
    ///     phone: None,
    ///     birth_data: None,
    ///     nationality: Some("US".into()),
    ///     identification_data: None,
    ///     residential_address: None,
    ///     tax_information: None,
    /// };
    ///
    /// let request = LegalEntityInfo::builder()
    ///     .entity_type(LegalEntityType::Individual)
    ///     .individual(individual)
    ///     .reference("individual_001")
    ///     .build()
    ///     .map_err(|e| format!("Builder error: {}", e))?;
    ///
    /// let legal_entity = api.create_legal_entity(&request).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_legal_entity(&self, request: &LegalEntityInfo) -> Result<LegalEntity> {
        let url = format!(
            "{}/v3/legalEntities",
            self.client.config().environment().legal_entity_api_url()
        );
        let response = self.client.post(&url, request).await?;
        Ok(response.data)
    }

    /// Get a legal entity by ID.
    ///
    /// Returns the legal entity details for the specified ID.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn get_legal_entity(&self, legal_entity_id: &str) -> Result<LegalEntity> {
        let url = format!(
            "{}/v3/legalEntities/{}",
            self.client.config().environment().legal_entity_api_url(),
            legal_entity_id
        );
        let response = self.client.get(&url).await?;
        Ok(response.data)
    }

    /// Update a legal entity.
    ///
    /// Updates the legal entity with the provided information.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn update_legal_entity(
        &self,
        legal_entity_id: &str,
        request: &LegalEntityInfo,
    ) -> Result<LegalEntity> {
        let url = format!(
            "{}/v3/legalEntities/{}",
            self.client.config().environment().legal_entity_api_url(),
            legal_entity_id
        );
        let response = self.client.patch(&url, request).await?;
        Ok(response.data)
    }

    // ============================================================================
    // Business Lines Management
    // ============================================================================

    /// Create a new business line.
    ///
    /// Creates a business line for a legal entity to specify operational details.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn create_business_line(&self, request: &BusinessLineInfo) -> Result<BusinessLine> {
        let url = format!(
            "{}/v3/businessLines",
            self.client.config().environment().legal_entity_api_url()
        );
        let response = self.client.post(&url, request).await?;
        Ok(response.data)
    }

    /// Get a business line by ID.
    ///
    /// Returns the business line details for the specified ID.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn get_business_line(&self, business_line_id: &str) -> Result<BusinessLine> {
        let url = format!(
            "{}/v3/businessLines/{}",
            self.client.config().environment().legal_entity_api_url(),
            business_line_id
        );
        let response = self.client.get(&url).await?;
        Ok(response.data)
    }

    /// Update a business line.
    ///
    /// Updates the business line with the provided information.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn update_business_line(
        &self,
        business_line_id: &str,
        request: &BusinessLineInfo,
    ) -> Result<BusinessLine> {
        let url = format!(
            "{}/v3/businessLines/{}",
            self.client.config().environment().legal_entity_api_url(),
            business_line_id
        );
        let response = self.client.patch(&url, request).await?;
        Ok(response.data)
    }

    /// Delete a business line.
    ///
    /// Deletes the business line for the specified ID.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails.
    pub async fn delete_business_line(&self, business_line_id: &str) -> Result<()> {
        let url = format!(
            "{}/v3/businessLines/{}",
            self.client.config().environment().legal_entity_api_url(),
            business_line_id
        );
        self.client.delete(&url).await?;
        Ok(())
    }

    // ============================================================================
    // Documents Management
    // ============================================================================

    /// Upload a document for verification.
    ///
    /// Uploads a document to support KYC verification for a legal entity.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn upload_document(&self, request: &Document) -> Result<Document> {
        let url = format!(
            "{}/v3/documents",
            self.client.config().environment().legal_entity_api_url()
        );
        let response = self.client.post(&url, request).await?;
        Ok(response.data)
    }

    /// Get a document by ID.
    ///
    /// Returns the document details for the specified ID.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn get_document(&self, document_id: &str) -> Result<Document> {
        let url = format!(
            "{}/v3/documents/{}",
            self.client.config().environment().legal_entity_api_url(),
            document_id
        );
        let response = self.client.get(&url).await?;
        Ok(response.data)
    }

    /// Update a document.
    ///
    /// Updates the document with the provided information.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn update_document(&self, document_id: &str, request: &Document) -> Result<Document> {
        let url = format!(
            "{}/v3/documents/{}",
            self.client.config().environment().legal_entity_api_url(),
            document_id
        );
        let response = self.client.patch(&url, request).await?;
        Ok(response.data)
    }

    /// Delete a document.
    ///
    /// Deletes the document for the specified ID.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails.
    pub async fn delete_document(&self, document_id: &str) -> Result<()> {
        let url = format!(
            "{}/v3/documents/{}",
            self.client.config().environment().legal_entity_api_url(),
            document_id
        );
        self.client.delete(&url).await?;
        Ok(())
    }

    // ============================================================================
    // Transfer Instruments Management
    // ============================================================================

    /// Create a new transfer instrument.
    ///
    /// Creates a transfer instrument for moving funds to/from a legal entity.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn create_transfer_instrument(
        &self,
        request: &TransferInstrumentInfo,
    ) -> Result<TransferInstrument> {
        let url = format!(
            "{}/v3/transferInstruments",
            self.client.config().environment().legal_entity_api_url()
        );
        let response = self.client.post(&url, request).await?;
        Ok(response.data)
    }

    /// Get a transfer instrument by ID.
    ///
    /// Returns the transfer instrument details for the specified ID.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn get_transfer_instrument(
        &self,
        transfer_instrument_id: &str,
    ) -> Result<TransferInstrument> {
        let url = format!(
            "{}/v3/transferInstruments/{}",
            self.client.config().environment().legal_entity_api_url(),
            transfer_instrument_id
        );
        let response = self.client.get(&url).await?;
        Ok(response.data)
    }

    /// Update a transfer instrument.
    ///
    /// Updates the transfer instrument with the provided information.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn update_transfer_instrument(
        &self,
        transfer_instrument_id: &str,
        request: &TransferInstrumentInfo,
    ) -> Result<TransferInstrument> {
        let url = format!(
            "{}/v3/transferInstruments/{}",
            self.client.config().environment().legal_entity_api_url(),
            transfer_instrument_id
        );
        let response = self.client.patch(&url, request).await?;
        Ok(response.data)
    }

    /// Delete a transfer instrument.
    ///
    /// Deletes the transfer instrument for the specified ID.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails.
    pub async fn delete_transfer_instrument(&self, transfer_instrument_id: &str) -> Result<()> {
        let url = format!(
            "{}/v3/transferInstruments/{}",
            self.client.config().environment().legal_entity_api_url(),
            transfer_instrument_id
        );
        self.client.delete(&url).await?;
        Ok(())
    }

    // ============================================================================
    // Hosted Onboarding
    // ============================================================================

    /// Create a hosted onboarding link.
    ///
    /// Creates a link for user self-service onboarding.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn create_onboarding_link(
        &self,
        request: &OnboardingLinkInfo,
    ) -> Result<OnboardingLink> {
        let url = format!(
            "{}/v3/hostedOnboarding/links",
            self.client.config().environment().legal_entity_api_url()
        );
        let response = self.client.post(&url, request).await?;
        Ok(response.data)
    }

    /// Get hosted onboarding themes.
    ///
    /// Returns available themes for hosted onboarding customization.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn get_onboarding_themes(&self) -> Result<Vec<OnboardingTheme>> {
        let url = format!(
            "{}/v3/hostedOnboarding/themes",
            self.client.config().environment().legal_entity_api_url()
        );
        let response: adyen_core::ApiResponse<PaginatedResponse<OnboardingTheme>> =
            self.client.get(&url).await?;
        Ok(response.data.data)
    }

    // ============================================================================
    // PCI Questionnaires (Compliance)
    // ============================================================================

    /// Get PCI questionnaire information.
    ///
    /// Returns PCI DSS questionnaire information for compliance.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn get_pci_questionnaire_infos(
        &self,
        legal_entity_id: &str,
    ) -> Result<serde_json::Value> {
        let url = format!(
            "{}/v3/legalEntities/{}/pciQuestionnaires",
            self.client.config().environment().legal_entity_api_url(),
            legal_entity_id
        );
        let response = self.client.get(&url).await?;
        Ok(response.data)
    }

    /// Get PCI questionnaire details.
    ///
    /// Returns detailed PCI questionnaire for a specific questionnaire ID.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn get_pci_questionnaire(
        &self,
        legal_entity_id: &str,
        pci_id: &str,
    ) -> Result<serde_json::Value> {
        let url = format!(
            "{}/v3/legalEntities/{}/pciQuestionnaires/{}",
            self.client.config().environment().legal_entity_api_url(),
            legal_entity_id,
            pci_id
        );
        let response = self.client.get(&url).await?;
        Ok(response.data)
    }

    /// Generate PCI description.
    ///
    /// Generates PCI DSS questionnaire description.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn generate_pci_description(
        &self,
        legal_entity_id: &str,
        pci_id: &str,
        request: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        let url = format!(
            "{}/v3/legalEntities/{}/pciQuestionnaires/{}/generatePciDescription",
            self.client.config().environment().legal_entity_api_url(),
            legal_entity_id,
            pci_id
        );
        let response = self.client.post(&url, request).await?;
        Ok(response.data)
    }

    /// Sign PCI questionnaire.
    ///
    /// Signs the PCI DSS questionnaire for compliance.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn sign_pci_questionnaire(
        &self,
        legal_entity_id: &str,
        pci_id: &str,
        request: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        let url = format!(
            "{}/v3/legalEntities/{}/pciQuestionnaires/{}/signPciQuestionnaire",
            self.client.config().environment().legal_entity_api_url(),
            legal_entity_id,
            pci_id
        );
        let response = self.client.post(&url, request).await?;
        Ok(response.data)
    }

    // ============================================================================
    // Terms of Service
    // ============================================================================

    /// Get terms of service acceptance information.
    ///
    /// Returns information about terms of service acceptance status.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn get_terms_of_service_acceptance_infos(
        &self,
        legal_entity_id: &str,
    ) -> Result<serde_json::Value> {
        let url = format!(
            "{}/v3/legalEntities/{}/termsOfService",
            self.client.config().environment().legal_entity_api_url(),
            legal_entity_id
        );
        let response = self.client.get(&url).await?;
        Ok(response.data)
    }

    /// Get terms of service document.
    ///
    /// Returns the terms of service document for the legal entity.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn get_terms_of_service_document(
        &self,
        legal_entity_id: &str,
        tos_id: &str,
    ) -> Result<serde_json::Value> {
        let url = format!(
            "{}/v3/legalEntities/{}/termsOfService/{}",
            self.client.config().environment().legal_entity_api_url(),
            legal_entity_id,
            tos_id
        );
        let response = self.client.get(&url).await?;
        Ok(response.data)
    }

    /// Accept terms of service.
    ///
    /// Accepts the terms of service for the legal entity.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn accept_terms_of_service(
        &self,
        legal_entity_id: &str,
        tos_id: &str,
        request: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        let url = format!(
            "{}/v3/legalEntities/{}/termsOfService/{}/acceptTermsOfService",
            self.client.config().environment().legal_entity_api_url(),
            legal_entity_id,
            tos_id
        );
        let response = self.client.patch(&url, request).await?;
        Ok(response.data)
    }

    // ============================================================================
    // Tax E-Delivery Consent
    // ============================================================================

    /// Check tax electronic delivery consent.
    ///
    /// Checks the status of tax document electronic delivery consent.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn check_tax_e_delivery_consent(
        &self,
        legal_entity_id: &str,
    ) -> Result<serde_json::Value> {
        let url = format!(
            "{}/v3/legalEntities/{}/taxElectronicDeliveryConsent",
            self.client.config().environment().legal_entity_api_url(),
            legal_entity_id
        );
        let response = self.client.get(&url).await?;
        Ok(response.data)
    }

    /// Set tax electronic delivery consent.
    ///
    /// Sets the consent for electronic delivery of tax documents.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    pub async fn set_tax_e_delivery_consent(
        &self,
        legal_entity_id: &str,
        request: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        let url = format!(
            "{}/v3/legalEntities/{}/taxElectronicDeliveryConsent",
            self.client.config().environment().legal_entity_api_url(),
            legal_entity_id
        );
        let response = self.client.patch(&url, request).await?;
        Ok(response.data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use adyen_core::{ConfigBuilder, Environment};

    #[test]
    fn test_legal_entity_api_creation() {
        let config = ConfigBuilder::new()
            .environment(Environment::test())
            .api_key("test_key_1234567890123456")
            .unwrap()
            .build()
            .unwrap();

        let _api = LegalEntityApi::new(config).unwrap();
        // API created successfully indicates proper configuration
    }
}
