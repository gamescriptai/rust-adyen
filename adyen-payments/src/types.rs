//! Types for Classic Payments API requests and responses.

pub mod payment;
pub mod modifications;
pub mod three_d_secure;
pub mod fraud;
pub mod additional;

// Re-export main types
pub use payment::{
    PaymentRequest, PaymentResult, PaymentResultCode, Card, PaymentMethod,
    ApplicationInfo, BrowserInfo, FraudResult, RecurringType,
};
pub use modifications::{
    CaptureRequest, CancelRequest, RefundRequest, CancelOrRefundRequest,
    ModificationResult, ModificationResponse,
};
pub use three_d_secure::{
    PaymentRequest3d, PaymentRequest3ds2, ThreeDSecureData, ThreeDS2RequestData,
    ThreeDS2Result, AuthenticationResultRequest, AuthenticationResultResponse,
};
pub use fraud::{FraudCheckResult, FraudCheckResultWrapper};
pub use additional::{
    AdjustAuthorisationRequest, DonateRequest,
    ThreeDSResultRequest, ThreeDSResultResponse, TechnicalCancelRequest, VoidPendingRefundRequest,
};