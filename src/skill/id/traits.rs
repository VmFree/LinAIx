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

//! # Skill ID 接口定义
//!
//! ## Version
//! 0.1.0

use super::types::*;
use crate::skill::error::SkillError;

/// Skill ID 解析器
pub trait SkillIdParser: Send + Sync {
    /// 从字符串解析 SkillId
    fn parse(&self, s: &str) -> Result<SkillId, SkillError>;

    /// 格式化 SkillId 为字符串
    fn format(&self, id: &SkillId) -> String;

    /// 验证 SkillId 是否有效
    fn validate(&self, id: &SkillId) -> bool;

    /// 获取 SkillId 的基础部分 (不含版本)
    fn base(&self, id: &SkillId) -> String;
}