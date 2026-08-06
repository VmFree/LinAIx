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

//! # Skill 结果类型定义
//!
//! ## Version
//! 0.1.0

use serde::{Deserialize, Serialize};

use crate::skill::id::SkillId;
use crate::skill::invocation::SkillInvocationContext;

/// Skill 执行结果
#[derive(Debug, Clone)]
pub struct SkillResult {
    pub skill_id: SkillId,
    pub context: SkillInvocationContext,
    pub success: bool,
    pub output: Option<serde_json::Value>,
    pub error: Option<SkillExecutionError>,
    pub tokens_consumed: u64,
    pub execution_ms: u64,
    pub retry_count: u32,
    pub completed_at: chrono::DateTime<chrono::Utc>,
}

/// Skill 执行错误
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillExecutionError {
    pub code: SkillErrorCode,
    pub message: alloc::string::String,
    pub details: Option<serde_json::Value>,
}

/// Skill 错误码
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SkillErrorCode {
    PermissionDenied,
    InvalidInput,
    InvalidOutput,
    QuotaExceeded,
    InjectionDetected,
    ExecutionFailed,
    Timeout,
    Canceled,
    SkillNotFound,
    Internal,
}

/// Skill 统计信息
#[derive(Debug, Clone, Default)]
pub struct SkillStats {
    pub total_invocations: u64,
    pub total_success: u64,
    pub total_failure: u64,
    pub total_timeout: u64,
    pub total_tokens_consumed: u64,
    pub avg_execution_ms: f64,
    pub last_invocation: Option<chrono::DateTime<chrono::Utc>>,
}