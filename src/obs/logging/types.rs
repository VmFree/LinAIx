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

//! # Logging 类型定义
//!
//! ## Version
//! 0.1.0
//!
//! ## Author
//! VmFree <vmfree@example.com>
//!
//! ## Date
//! 2026-08-01

use serde::{Deserialize, Serialize};

use crate::obs::tracing::{SpanId, TraceId};

/// 日志级别
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum LogLevel {
    Trace = 0,
    Debug = 1,
    Info = 2,
    Warn = 3,
    Error = 4,
}

/// 日志条目
#[derive(Debug, Clone)]
pub struct LogEntry {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub level: LogLevel,
    pub target: alloc::string::String,
    pub message: alloc::string::String,
    pub fields: std::collections::HashMap<alloc::string::String, serde_json::Value>,
    pub trace_id: Option<TraceId>,
    pub span_id: Option<SpanId>,
}

/// 日志过滤器
#[derive(Debug, Clone, Default)]
pub struct LogFilter {
    pub min_level: Option<LogLevel>,
    pub target: Option<alloc::string::String>,
    pub trace_id: Option<TraceId>,
    pub start_time: Option<chrono::DateTime<chrono::Utc>>,
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,
    pub limit: Option<usize>,
}

/// 日志配置
#[derive(Debug, Clone)]
pub struct LogConfig {
    pub min_level: LogLevel,
    pub max_size_bytes: u64,
    pub max_files: u32,
    pub output_path: alloc::string::String,
    pub structured: bool,  // JSON 格式
}