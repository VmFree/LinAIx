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

//! # Skill ID 类型定义
//!
//! ## Version
//! 0.1.0

use serde::{Deserialize, Serialize};

/// Skill 版本 (语义化版本)
pub type SkillVersion = semver::Version;

/// Skill ID
///
/// 格式: `{framework}/{namespace}/{name}@{version}`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SkillId {
    pub framework: alloc::string::String,
    pub namespace: alloc::string::String,
    pub name: alloc::string::String,
    pub version: Option<SkillVersion>,
}

impl SkillId {
    pub fn new(
        framework: impl Into<alloc::string::String>,
        namespace: impl Into<alloc::string::String>,
        name: impl Into<alloc::string::String>,
        version: Option<SkillVersion>,
    ) -> Self {
        Self {
            framework: framework.into(),
            namespace: namespace.into(),
            name: name.into(),
            version,
        }
    }

    pub fn to_string(&self) -> alloc::string::String {
        let base = format!("{}/{}/{}", self.framework, self.namespace, self.name);
        if let Some(v) = &self.version {
            format!("{}@{}", base, v)
        } else {
            base
        }
    }

    pub fn base_id(&self) -> alloc::string::String {
        format!("{}/{}/{}", self.framework, self.namespace, self.name)
    }
}