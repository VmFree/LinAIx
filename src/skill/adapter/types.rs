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

//! # Skill 适配器类型定义
//!
//! ## Version
//! 0.1.0

use crate::skill::id::SkillId;

/// 适配器方向
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdapterDirection {
    /// 反向适配: LinAIx Skill → 框架原生工具
    LinAIxToFramework,

    /// 正向适配: 框架工具调用 → LinAIx Skill
    FrameworkToLinAIx,

    /// 双向适配
    Both,
}

/// 适配器上下文
#[derive(Debug, Clone)]
pub struct AdapterContext {
    pub framework: alloc::string::String,
    pub skill_id: SkillId,
    pub direction: AdapterDirection,
}

/// 适配器结果
#[derive(Debug, Clone)]
pub struct AdapterResult {
    pub success: bool,
    pub converted_input: Option<serde_json::Value>,
    pub converted_output: Option<serde_json::Value>,
    pub error: Option<alloc::string::String>,
}