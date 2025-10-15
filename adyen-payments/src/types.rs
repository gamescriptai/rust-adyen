//! Types for Classic Payments API requests and responses.

pub mod additional;
pub mod fraud;
pub mod modifications;
pub mod payment;
pub mod three_d_secure;

// Re-export main types
pub use additional::{
    AdjustAuthorisationRequest, DonateRequest, TechnicalCancelRequest, ThreeDSResultRequest,
    ThreeDSResultResponse, VoidPendingRefundRequest,
};
pub use fraud::{FraudCheckResult, FraudCheckResultWrapper};
pub use modifications::{
    CancelOrRefundRequest, CancelRequest, CaptureRequest, ModificationResponse, ModificationResult,
    RefundRequest,
};
pub use payment::{
    ApplicationInfo, BrowserInfo, Card, FraudResult, PaymentMethod, PaymentRequest, PaymentResult,
    PaymentResultCode, RecurringType,
};
pub use three_d_secure::{
    AuthenticationResultRequest, AuthenticationResultResponse, PaymentRequest3d,
    PaymentRequest3ds2, ThreeDS2RequestData, ThreeDS2Result, ThreeDSecureData,
};
