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

//! # Skill 权限接口定义
//!
//! ## Version
//! 0.1.0

use super::types::*;
use crate::skill::error::SkillError;
use crate::skill::invocation::SkillInvocationContext;

/// Skill 权限检查器
///
/// 默认拒绝策略：如果没有配置权限，则拒绝访问。
pub trait SkillPermissionChecker: Send + Sync {
    /// 检查 SubTask 是否有权调用 Skill
    fn check_permission(
        &self,
        context: &SkillInvocationContext,
        skill_id: &SkillId,
        action: SkillAction,
    ) -> Result<(), SkillError>;
}