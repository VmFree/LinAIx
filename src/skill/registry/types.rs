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

//! # Skill 注册中心类型定义
//!
//! ## Version
//! 0.1.0

use crate::skill::metadata::SkillMetadata;
use crate::skill::executor::SkillExecutor;

/// Skill 条目 (包含元数据和执行器)
pub struct SkillEntry {
    pub metadata: SkillMetadata,
    pub executor: Box<dyn SkillExecutor>,
}

/// Skill 过滤器
#[derive(Debug, Clone, Default)]
pub struct SkillFilter {
    pub framework: Option<alloc::string::String>,
    pub namespace: Option<alloc::string::String>,
    pub tags: Option<alloc::vec::Vec<alloc::string::String>>,
    pub status: Option<SkillStatus>,
    pub name_contains: Option<alloc::string::String>,
}