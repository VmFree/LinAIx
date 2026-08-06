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

//! # Skill 调用接口定义
//!
//! ## Version
//! 0.1.0

use super::types::*;
use crate::skill::error::SkillError;
use crate::skill::result::SkillResult;

/// Skill 调用接口
pub trait SkillInvoker: Send + Sync {
    /// 同步调用 Skill
    fn invoke(&self, invocation: &SkillInvocation) -> Result<SkillResult, SkillError>;

    /// 异步调用 Skill
    async fn invoke_async(&self, invocation: &SkillInvocation) -> Result<SkillResult, SkillError>;

    /// 流式调用 Skill
    fn invoke_stream(&self, invocation: &SkillInvocation) -> Result<Box<dyn SkillStreamOutput>, SkillError>;

    /// 取消正在执行的 Skill 调用
    fn cancel(&self, trace_id: &str) -> Result<(), SkillError>;
}