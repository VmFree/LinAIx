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

//! # Agent 运行时配置类型定义
//!
//! ## Version
//! 0.1.0

use serde::{Deserialize, Serialize};

/// Agent 运行时配置
#[derive(Debug, Clone)]
pub struct RuntimeConfig {
    pub default_timeout_ms: u64,
    pub max_retries: u32,
    pub enable_sandbox: bool,
    pub enable_audit: bool,
    pub log_level: LogLevel,
    pub max_log_size_bytes: u64,
}

/// Agent 配置
#[derive(Debug, Clone)]
pub struct AgentConfig {
    pub language: AgentLanguage,
    pub entry_point: alloc::string::String,
    pub args: alloc::vec::Vec<alloc::string::String>,
    pub env: std::collections::HashMap<alloc::string::String, alloc::string::String>,
    pub timeout_ms: u64,
    pub retry_on_failure: bool,
    pub max_retries: u32,
}

/// Agent 语言
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentLanguage {
    Python,
    Go,
    JavaScript,
    Rust,
    Custom(alloc::string::String),
}

/// 日志级别
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum LogLevel {
    Error = 0,
    Warn = 1,
    Info = 2,
    Debug = 3,
    Trace = 4,
}