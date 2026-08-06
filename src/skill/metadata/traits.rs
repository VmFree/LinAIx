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

//! # Skill 元数据接口定义
//!
//! ## Version
//! 0.1.0

use super::types::*;
use crate::skill::error::SkillError;

/// Skill 元数据验证器
pub trait SkillMetadataValidator: Send + Sync {
    /// 验证 Skill 元数据是否完整有效
    fn validate(&self, metadata: &SkillMetadata) -> Result<(), SkillError>;

    /// 验证输入是否符合 Schema
    fn validate_input(&self, metadata: &SkillMetadata, input: &serde_json::Value) -> Result<(), SkillError>;

    /// 验证输出是否符合 Schema
    fn validate_output(&self, metadata: &SkillMetadata, output: &serde_json::Value) -> Result<(), SkillError>;
}