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

//! # Skill 元数据类型定义
//!
//! ## Version
//! 0.1.0

use serde::{Deserialize, Serialize};

use crate::skill::id::SkillId;
use crate::skill::permission::SkillPermission;

/// Skill 元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillMetadata {
    pub id: SkillId,
    pub name: alloc::string::String,
    pub description: alloc::string::String,
    pub input_schema: serde_json::Value,
    pub output_schema: serde_json::Value,
    pub framework: alloc::string::String,
    pub namespace: alloc::string::String,
    pub version: semver::Version,
    pub tags: alloc::vec::Vec<alloc::string::String>,
    pub permissions: alloc::vec::Vec<SkillPermission>,
    pub timeout_ms: Option<u64>,
    pub retry_config: Option<RetryConfig>,
    pub status: SkillStatus,
    pub deprecated_replacement: Option<SkillId>,
    pub registered_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// 重试配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryConfig {
    pub max_retries: u32,
    pub initial_backoff_ms: u64,
    pub max_backoff_ms: u64,
    pub backoff_multiplier: f64,
    pub retryable_errors: alloc::vec::Vec<alloc::string::String>,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            initial_backoff_ms: 100,
            max_backoff_ms: 10000,
            backoff_multiplier: 2.0,
            retryable_errors: alloc::vec::Vec::new(),
        }
    }
}

/// Skill 状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SkillStatus {
    Active,
    Deprecated,
    Disabled,
}