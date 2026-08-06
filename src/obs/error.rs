// SPDX-License-Identifier: GPL-2.0-only
//
// LinAIx - The Linux of the AI era
// Copyright (C) 2026 VmFree <vmfree@example.com>
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public License
// version 2 as published by the Free Software Foundation.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

//! # Observability 错误类型
//!
//! ## Version
//! 0.1.0
//!
//! ## Author
//! VmFree <vmfree@example.com>
//!
//! ## Date
//! 2026-08-01

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ObsError {
    // ===== Metrics 错误 =====
    #[error("Metric not found: {0}")]
    MetricNotFound(alloc::string::String),

    #[error("Metric already exists: {0}")]
    MetricAlreadyExists(alloc::string::String),

    #[error("Invalid metric value: {0}")]
    InvalidMetricValue(alloc::string::String),

    // ===== Tracing 错误 =====
    #[error("Trace not found: {0}")]
    TraceNotFound(TraceId),

    #[error("Span not found: {0}")]
    SpanNotFound(SpanId),

    #[error("Invalid span context: {0}")]
    InvalidSpanContext(alloc::string::String),

    // ===== Logging 错误 =====
    #[error("Log write failed: {0}")]
    LogWriteFailed(alloc::string::String),

    #[error("Log query failed: {0}")]
    LogQueryFailed(alloc::string::String),

    // ===== Dump 错误 =====
    #[error("Dump creation failed: {0}")]
    DumpCreationFailed(alloc::string::String),

    #[error("Dump not found: {0}")]
    DumpNotFound(alloc::string::String),

    #[error("Dump restore failed: {0}")]
    DumpRestoreFailed(alloc::string::String),

    // ===== Profiling 错误 =====
    #[error("Profiling failed: {0}")]
    ProfilingFailed(alloc::string::String),

    #[error("Profiling not supported: {0}")]
    ProfilingNotSupported(alloc::string::String),

    // ===== Event 错误 =====
    #[error("Event publish failed: {0}")]
    EventPublishFailed(alloc::string::String),

    #[error("Event subscription failed: {0}")]
    EventSubscriptionFailed(alloc::string::String),

    // ===== 通用错误 =====
    #[error("Internal error: {0}")]
    Internal(alloc::string::String),

    #[error("Configuration error: {0}")]
    ConfigError(alloc::string::String),

    #[error("I/O error: {0}")]
    IoError(alloc::string::String),
}