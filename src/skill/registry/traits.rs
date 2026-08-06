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

//! # Skill 注册中心接口定义
//!
//! ## Version
//! 0.1.0

use super::types::*;
use crate::skill::error::SkillError;
use crate::skill::id::SkillId;
use crate::skill::metadata::SkillMetadata;
use crate::skill::metadata::SkillStatus;
use crate::skill::executor::SkillExecutor;

/// Skill 注册中心接口
pub trait SkillRegistry: Send + Sync {
    // ===== 注册与注销 =====

    fn register(&mut self, metadata: SkillMetadata, executor: Box<dyn SkillExecutor>) -> Result<(), SkillError>;
    fn unregister(&mut self, skill_id: &SkillId) -> Result<(), SkillError>;

    // ===== 查询与发现 =====

    fn lookup(&self, skill_id: &SkillId) -> Result<SkillEntry, SkillError>;
    fn lookup_version(&self, skill_id: &SkillId, version: &semver::Version) -> Result<SkillEntry, SkillError>;
    fn list(&self, filter: Option<SkillFilter>) -> alloc::vec::Vec<SkillEntry>;
    fn list_by_framework(&self, framework: &str) -> alloc::vec::Vec<SkillEntry>;
    fn list_by_namespace(&self, namespace: &str) -> alloc::vec::Vec<SkillEntry>;
    fn get_versions(&self, skill_id: &SkillId) -> alloc::vec::Vec<semver::Version>;
    fn exists(&self, skill_id: &SkillId) -> bool;

    // ===== 版本管理 =====

    fn deprecate_version(&mut self, skill_id: &SkillId, version: &semver::Version, replacement: Option<SkillId>) -> Result<(), SkillError>;
    fn get_status(&self, skill_id: &SkillId) -> Option<SkillStatus>;
}