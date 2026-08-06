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

//! # 模型实例类型定义
//!
//! ## Version
//! 0.1.0
//!
//! ## Author
//! VmFree <vmfree@example.com>
//!
//! ## Date
//! 2026-08-01

use serde::{Deserialize, Serialize};

/// 模型 ID
pub type ModelId = alloc::string::String;

/// 模型实例句柄
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModelInstanceHandle {
    pub id: alloc::string::String,
    pub model_id: ModelId,
    pub device_id: u32,
}

/// 模型实例信息
#[derive(Debug, Clone)]
pub struct ModelInstanceInfo {
    pub handle: ModelInstanceHandle,
    pub ref_count: u32,
    pub state: ModelInstanceState,
    pub loaded_at: chrono::DateTime<chrono::Utc>,
    pub last_used_at: chrono::DateTime<chrono::Utc>,
    pub memory_used_bytes: u64,
    pub backend: alloc::string::String,
}

/// 模型实例状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModelInstanceState {
    Loading,
    Ready,
    InUse,
    Cached,
    Unloading,
    Error,
}

/// 模型加载配置
#[derive(Debug, Clone)]
pub struct LoadConfig {
    pub device_id: Option<u32>,
    pub max_memory_bytes: Option<u64>,
    pub precision: Option<alloc::string::String>,
    pub load_options: std::collections::HashMap<alloc::string::String, alloc::string::String>,
    pub timeout_ms: Option<u64>,
}

/// 实例池状态
#[derive(Debug, Clone)]
pub struct InstancePoolStatus {
    pub total_instances: usize,
    pub active_instances: usize,
    pub cached_instances: usize,
    pub loading_instances: usize,
    pub total_memory_used_bytes: u64,
    pub max_memory_bytes: u64,
    pub available_memory_bytes: u64,
    pub hit_rate: f32,
}