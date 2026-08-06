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

//! # Skill 结果接口定义
//!
//! ## Version
//! 0.1.0

use super::types::*;
use crate::skill::error::SkillError;
use crate::skill::invocation::SkillInvocationContext;

/// 流式输出接口
pub trait SkillStreamOutput: Send + Sync {
    async fn next(&mut self) -> Option<Result<serde_json::Value, SkillError>>;
    fn cancel(&self) -> Result<(), SkillError>;
}

/// Skill 结果处理器
pub trait SkillResultHandler: Send + Sync {
    fn on_success(&self, context: &SkillInvocationContext, output: &serde_json::Value);
    fn on_error(&self, context: &SkillInvocationContext, error: &SkillExecutionError);
    fn on_retry(&self, context: &SkillInvocationContext, retry_count: u32);
}