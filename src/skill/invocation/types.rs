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

//! # Skill 调用类型定义
//!
//! ## Version
//! 0.1.0

use serde::{Deserialize, Serialize};

use crate::skill::id::SkillId;
use crate::skill::metadata::RetryConfig;

/// Skill 调用
#[derive(Debug, Clone)]
pub struct SkillInvocation {
    pub skill_id: SkillId,
    pub input: serde_json::Value,
    pub context: SkillInvocationContext,
}

/// Skill 调用上下文
#[derive(Debug, Clone)]
pub struct SkillInvocationContext {
    pub subtask_id: SubTaskId,
    pub task_id: TaskId,
    pub trace_id: alloc::string::String,
    pub parent_span_id: Option<alloc::string::String>,
    pub timeout_ms: u64,
    pub retry_config: RetryConfig,
    pub security_labels: alloc::vec::Vec<SecurityLabel>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// 安全标签
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityLabel {
    pub key: alloc::string::String,
    pub value: alloc::string::String,
}