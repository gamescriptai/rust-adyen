//! Types for the Adyen Recurring API v68.
//!
//! This module contains all request and response types for recurring payment operations.

use serde::{Deserialize, Serialize};

/// Placeholder for RecurringDetailsRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecurringDetailsRequest;

/// Placeholder for RecurringDetailsResult
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecurringDetailsResult;

/// Placeholder for DisableRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisableRequest;

/// Placeholder for DisableResult
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisableResult;

/// Placeholder for NotifyShopperRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotifyShopperRequest;

/// Placeholder for NotifyShopperResult
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotifyShopperResult;

/// Placeholder for ScheduleAccountUpdaterRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleAccountUpdaterRequest;

/// Placeholder for ScheduleAccountUpdaterResult
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleAccountUpdaterResult;

/// Placeholder for RecurringDetail
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecurringDetail;

/// Placeholder for Recurring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recurring;

/// Placeholder for RecurringContract
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecurringContract;