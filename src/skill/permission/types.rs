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

//! # Skill 权限类型定义
//!
//! ## Version
//! 0.1.0

use serde::{Deserialize, Serialize};

use crate::skill::id::SkillId;

/// Skill 权限定义
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SkillPermission {
    pub skill_id: SkillIdPattern,
    pub action: SkillAction,
    pub scope: Option<alloc::string::String>,
}

/// Skill ID 模式 (支持通配符)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SkillIdPattern {
    Exact(SkillId),
    Wildcard {
        framework: Option<alloc::string::String>,
        namespace: Option<alloc::string::String>,
        name: Option<alloc::string::String>,
    },
    All,
}

impl SkillIdPattern {
    pub fn matches(&self, skill_id: &SkillId) -> bool {
        match self {
            SkillIdPattern::Exact(id) => id == skill_id,
            SkillIdPattern::Wildcard { framework, namespace, name } => {
                let framework_match = framework.as_ref().map_or(true, |f| f == &skill_id.framework);
                let namespace_match = namespace.as_ref().map_or(true, |n| n == &skill_id.namespace);
                let name_match = name.as_ref().map_or(true, |n| n == &skill_id.name);
                framework_match && namespace_match && name_match
            }
            SkillIdPattern::All => true,
        }
    }
}

/// Skill 操作类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SkillAction {
    Invoke,
    Register,
    Unregister,
    View,
    Manage,
    All,
}