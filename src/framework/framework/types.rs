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

//! # 框架类型定义
//!
//! ## Version
//! 0.1.0

use serde::{Deserialize, Serialize};

pub type FrameworkId = alloc::string::String;

/// Agent 框架类型
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FrameworkType {
    LangChain,
    CrewAI,
    AutoGen,
    Custom(alloc::string::String),
}

/// 框架能力
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FrameworkCapability {
    TaskDecomposition,
    MultiAgent,
    ToolUse,
    Streaming,
    Memory,
    Reflection,
    HumanInLoop,
}

/// Agent 框架
#[derive(Debug, Clone)]
pub struct Framework {
    pub id: FrameworkId,
    pub name: alloc::string::String,
    pub framework_type: FrameworkType,
    pub version: alloc::string::String,
    pub description: alloc::string::String,
    pub capabilities: alloc::vec::Vec<FrameworkCapability>,
    pub installed: bool,
    pub default: bool,
    pub config: FrameworkConfig,
    pub installed_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// 框架配置
#[derive(Debug, Clone)]
pub struct FrameworkConfig {
    pub executable_path: Option<alloc::string::String>,
    pub environment: std::collections::HashMap<alloc::string::String, alloc::string::String>,
    pub default_timeout_ms: u64,
    pub max_retries: u32,
    pub extra_args: alloc::vec::Vec<alloc::string::String>,
}