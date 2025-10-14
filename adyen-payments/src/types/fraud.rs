//! Fraud detection types and result handling for payment risk assessment.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Risk score threshold levels for fraud detection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RiskLevel {
    /// Low risk transaction
    Low,
    /// Medium risk transaction
    Medium,
    /// High risk transaction
    High,
    /// Critical risk transaction
    Critical,
}

/// Fraud check recommendation actions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum FraudAction {
    /// Allow the transaction to proceed
    Allow,
    /// Block the transaction
    Block,
    /// Flag the transaction for manual review
    Challenge,
    /// Require additional authentication
    Review,
}

/// Comprehensive fraud detection result.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FraudResult {
    /// Account score indicating fraud likelihood (0-100, higher = more risky)
    pub account_score: Option<u8>,

    /// Check ID for tracking fraud assessments
    pub check_id: Option<i32>,

    /// Name of the fraud check provider
    pub name: Option<Box<str>>,

    /// Risk level assessment
    pub risk_level: Option<RiskLevel>,

    /// Recommended action based on fraud analysis
    pub action: Option<FraudAction>,

    /// Fraud score (0-100, higher = more risky)
    pub fraud_score: Option<u8>,

    /// Additional fraud check results from various providers
    pub results: Option<Vec<FraudCheckResult>>,

    /// Reason codes for the fraud assessment
    pub reason_codes: Option<Vec<Box<str>>>,
}

/// Individual fraud check result from a specific provider.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FraudCheckResult {
    /// Name of the fraud check provider
    pub name: Box<str>,

    /// Check ID for this specific fraud check
    pub check_id: i32,

    /// Account score from this provider (0-100)
    pub account_score: u8,

    /// Risk level from this provider
    pub risk_level: RiskLevel,

    /// Recommended action from this provider
    pub action: FraudAction,

    /// Additional metadata from the fraud check
    pub metadata: Option<HashMap<Box<str>, serde_json::Value>>,
}

/// Wrapper for fraud check results in payment responses.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FraudCheckResultWrapper {
    /// Fraud check results
    pub fraud_check_result: Option<FraudCheckResult>,

    /// Legacy fraud result format
    pub fraud_result: Option<FraudResult>,
}

/// Device fingerprinting data for fraud detection.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceFingerprint {
    /// Unique device identifier
    pub device_id: Option<Box<str>>,

    /// Device type (mobile, desktop, tablet)
    pub device_type: Option<Box<str>>,

    /// Operating system information
    pub operating_system: Option<Box<str>>,

    /// Browser information
    pub browser: Option<Box<str>>,

    /// Screen resolution
    pub screen_resolution: Option<Box<str>>,

    /// Timezone offset in minutes
    pub timezone_offset: Option<i16>,

    /// Language settings
    pub language: Option<Box<str>>,

    /// Color depth
    pub color_depth: Option<u8>,

    /// JavaScript enabled flag
    pub java_enabled: Option<bool>,

    /// Cookies enabled flag
    pub cookies_enabled: Option<bool>,
}

impl DeviceFingerprint {
    /// Create a new device fingerprint builder.
    pub fn builder() -> DeviceFingerprintBuilder {
        DeviceFingerprintBuilder::default()
    }
}

/// Builder for device fingerprint data.
#[derive(Debug, Default)]
pub struct DeviceFingerprintBuilder {
    device_id: Option<Box<str>>,
    device_type: Option<Box<str>>,
    operating_system: Option<Box<str>>,
    browser: Option<Box<str>>,
    screen_resolution: Option<Box<str>>,
    timezone_offset: Option<i16>,
    language: Option<Box<str>>,
    color_depth: Option<u8>,
    java_enabled: Option<bool>,
    cookies_enabled: Option<bool>,
}

impl DeviceFingerprintBuilder {
    /// Set the device ID.
    pub fn device_id<S: Into<Box<str>>>(mut self, device_id: S) -> Self {
        self.device_id = Some(device_id.into());
        self
    }

    /// Set the device type.
    pub fn device_type<S: Into<Box<str>>>(mut self, device_type: S) -> Self {
        self.device_type = Some(device_type.into());
        self
    }

    /// Set the operating system.
    pub fn operating_system<S: Into<Box<str>>>(mut self, os: S) -> Self {
        self.operating_system = Some(os.into());
        self
    }

    /// Set the browser information.
    pub fn browser<S: Into<Box<str>>>(mut self, browser: S) -> Self {
        self.browser = Some(browser.into());
        self
    }

    /// Set the screen resolution.
    pub fn screen_resolution<S: Into<Box<str>>>(mut self, resolution: S) -> Self {
        self.screen_resolution = Some(resolution.into());
        self
    }

    /// Set the timezone offset in minutes.
    pub fn timezone_offset(mut self, offset: i16) -> Self {
        self.timezone_offset = Some(offset);
        self
    }

    /// Set the language.
    pub fn language<S: Into<Box<str>>>(mut self, language: S) -> Self {
        self.language = Some(language.into());
        self
    }

    /// Set the color depth.
    pub fn color_depth(mut self, depth: u8) -> Self {
        self.color_depth = Some(depth);
        self
    }

    /// Set whether Java is enabled.
    pub fn java_enabled(mut self, enabled: bool) -> Self {
        self.java_enabled = Some(enabled);
        self
    }

    /// Set whether cookies are enabled.
    pub fn cookies_enabled(mut self, enabled: bool) -> Self {
        self.cookies_enabled = Some(enabled);
        self
    }

    /// Build the device fingerprint.
    pub fn build(self) -> DeviceFingerprint {
        DeviceFingerprint {
            device_id: self.device_id,
            device_type: self.device_type,
            operating_system: self.operating_system,
            browser: self.browser,
            screen_resolution: self.screen_resolution,
            timezone_offset: self.timezone_offset,
            language: self.language,
            color_depth: self.color_depth,
            java_enabled: self.java_enabled,
            cookies_enabled: self.cookies_enabled,
        }
    }
}

/// Risk assessment data for merchants.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiskData {
    /// Client data for fraud detection
    pub client_data: Option<Box<str>>,

    /// Custom fields for fraud detection
    pub custom_fields: Option<HashMap<Box<str>, Box<str>>>,

    /// Fraud offset for this transaction
    pub fraud_offset: Option<i32>,

    /// Profile reference for risk assessment
    pub profile_reference: Option<Box<str>>,

    /// Skip fraud checks flag
    pub skip_fraud: Option<bool>,
}

impl RiskData {
    /// Create a new risk data builder.
    pub fn builder() -> RiskDataBuilder {
        RiskDataBuilder::default()
    }
}

/// Builder for risk assessment data.
#[derive(Debug, Default)]
pub struct RiskDataBuilder {
    client_data: Option<Box<str>>,
    custom_fields: Option<HashMap<Box<str>, Box<str>>>,
    fraud_offset: Option<i32>,
    profile_reference: Option<Box<str>>,
    skip_fraud: Option<bool>,
}

impl RiskDataBuilder {
    /// Set the client data.
    pub fn client_data<S: Into<Box<str>>>(mut self, data: S) -> Self {
        self.client_data = Some(data.into());
        self
    }

    /// Add a custom field.
    pub fn custom_field<K, V>(mut self, key: K, value: V) -> Self
    where
        K: Into<Box<str>>,
        V: Into<Box<str>>,
    {
        self.custom_fields.get_or_insert_with(HashMap::new)
            .insert(key.into(), value.into());
        self
    }

    /// Set custom fields from a HashMap.
    pub fn custom_fields(mut self, fields: HashMap<Box<str>, Box<str>>) -> Self {
        self.custom_fields = Some(fields);
        self
    }

    /// Set the fraud offset.
    pub fn fraud_offset(mut self, offset: i32) -> Self {
        self.fraud_offset = Some(offset);
        self
    }

    /// Set the profile reference.
    pub fn profile_reference<S: Into<Box<str>>>(mut self, reference: S) -> Self {
        self.profile_reference = Some(reference.into());
        self
    }

    /// Set whether to skip fraud checks.
    pub fn skip_fraud(mut self, skip: bool) -> Self {
        self.skip_fraud = Some(skip);
        self
    }

    /// Build the risk data.
    pub fn build(self) -> RiskData {
        RiskData {
            client_data: self.client_data,
            custom_fields: self.custom_fields,
            fraud_offset: self.fraud_offset,
            profile_reference: self.profile_reference,
            skip_fraud: self.skip_fraud,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_fraud_result_serialization() {
        let fraud_result = FraudResult {
            account_score: Some(45),
            check_id: Some(12345),
            name: Some("FraudProvider".into()),
            risk_level: Some(RiskLevel::Medium),
            action: Some(FraudAction::Challenge),
            fraud_score: Some(60),
            results: None,
            reason_codes: Some(vec!["HIGH_VELOCITY".into(), "NEW_CARD".into()]),
        };

        let json = serde_json::to_string(&fraud_result).unwrap();
        let deserialized: FraudResult = serde_json::from_str(&json).unwrap();
        assert_eq!(fraud_result, deserialized);
    }

    #[test]
    fn test_fraud_check_result_serialization() {
        let mut metadata = HashMap::new();
        metadata.insert("provider".into(), serde_json::Value::String("TestProvider".to_string()));
        metadata.insert("version".into(), serde_json::Value::String("1.0".to_string()));

        let check_result = FraudCheckResult {
            name: "TestProvider".into(),
            check_id: 67890,
            account_score: 75,
            risk_level: RiskLevel::High,
            action: FraudAction::Block,
            metadata: Some(metadata),
        };

        let json = serde_json::to_string(&check_result).unwrap();
        let deserialized: FraudCheckResult = serde_json::from_str(&json).unwrap();
        assert_eq!(check_result, deserialized);
    }

    #[test]
    fn test_device_fingerprint_builder() {
        let fingerprint = DeviceFingerprint::builder()
            .device_id("device123")
            .device_type("mobile")
            .operating_system("iOS 15.0")
            .browser("Safari 15.0")
            .screen_resolution("1920x1080")
            .timezone_offset(-480)
            .language("en-US")
            .color_depth(24)
            .java_enabled(false)
            .cookies_enabled(true)
            .build();

        assert_eq!(fingerprint.device_id.as_deref(), Some("device123"));
        assert_eq!(fingerprint.device_type.as_deref(), Some("mobile"));
        assert_eq!(fingerprint.timezone_offset, Some(-480));
        assert_eq!(fingerprint.color_depth, Some(24));
        assert_eq!(fingerprint.java_enabled, Some(false));
        assert_eq!(fingerprint.cookies_enabled, Some(true));
    }

    #[test]
    fn test_risk_data_builder() {
        let risk_data = RiskData::builder()
            .client_data("encrypted_client_data")
            .custom_field("merchant_category", "retail")
            .custom_field("customer_tier", "premium")
            .fraud_offset(100)
            .profile_reference("profile_123")
            .skip_fraud(false)
            .build();

        assert_eq!(risk_data.client_data.as_deref(), Some("encrypted_client_data"));
        assert_eq!(risk_data.fraud_offset, Some(100));
        assert_eq!(risk_data.profile_reference.as_deref(), Some("profile_123"));
        assert_eq!(risk_data.skip_fraud, Some(false));

        let custom_fields = risk_data.custom_fields.unwrap();
        assert_eq!(custom_fields.get("merchant_category").map(|s| s.as_ref()), Some("retail"));
        assert_eq!(custom_fields.get("customer_tier").map(|s| s.as_ref()), Some("premium"));
    }

    #[test]
    fn test_risk_level_serialization() {
        assert_eq!(serde_json::to_string(&RiskLevel::Low).unwrap(), "\"low\"");
        assert_eq!(serde_json::to_string(&RiskLevel::Medium).unwrap(), "\"medium\"");
        assert_eq!(serde_json::to_string(&RiskLevel::High).unwrap(), "\"high\"");
        assert_eq!(serde_json::to_string(&RiskLevel::Critical).unwrap(), "\"critical\"");
    }

    #[test]
    fn test_fraud_action_serialization() {
        assert_eq!(serde_json::to_string(&FraudAction::Allow).unwrap(), "\"ALLOW\"");
        assert_eq!(serde_json::to_string(&FraudAction::Block).unwrap(), "\"BLOCK\"");
        assert_eq!(serde_json::to_string(&FraudAction::Challenge).unwrap(), "\"CHALLENGE\"");
        assert_eq!(serde_json::to_string(&FraudAction::Review).unwrap(), "\"REVIEW\"");
    }
}