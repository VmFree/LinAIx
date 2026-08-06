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

//! # Skill 执行器接口定义
//!
//! ## Version
//! 0.1.0

use super::types::*;
use crate::skill::error::SkillError;
use crate::skill::invocation::{SkillInvocation, SkillInvocationContext};
use crate::skill::metadata::SkillMetadata;

/// Skill 执行器接口
pub trait SkillExecutor: Send + Sync {
    fn execute(&self, input: serde_json::Value, context: &SkillInvocationContext) -> Result<serde_json::Value, SkillError>;
    fn execute_stream(&self, input: serde_json::Value, context: &SkillInvocationContext) -> Result<Box<dyn SkillStreamOutput>, SkillError>;
    fn get_metadata(&self) -> &SkillMetadata;
}