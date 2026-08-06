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

//! # KV Cache 类型定义
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

/// 会话 ID
pub type SessionId = alloc::string::String;

/// 模型 ID
pub type ModelId = alloc::string::String;

/// KV Cache 句柄
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KvCacheHandle {
    pub id: alloc::string::String,
    pub session_id: SessionId,
    pub model_id: ModelId,
    pub size_bytes: usize,
    pub location: StorageLocation,
    pub task_id: TaskId,
}

/// 存储位置
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StorageLocation {
    GpuMemory { device_id: u32, offset: u64 },
    SystemMemory { ptr: u64 },
    Disk { path: alloc::string::String },
}

/// KV Cache 状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum KvCacheState {
    Active,
    SwappedOut,
    Persisted,
    SwappingOut,
    SwappingIn,
    Freeing,
}

/// KV Cache 状态详情
#[derive(Debug, Clone)]
pub struct KvCacheStatus {
    pub handle: KvCacheHandle,
    pub state: KvCacheState,
    pub ref_count: u32,
    pub pinned: bool,
    pub last_used_at: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub access_count: u64,
}

/// KV Cache 统计信息
#[derive(Debug, Clone, Default)]
pub struct KvCacheStats {
    pub total_caches: u64,
    pub total_size_bytes: u64,
    pub active_count: u64,
    pub active_size_bytes: u64,
    pub swapped_out_count: u64,
    pub swapped_out_size_bytes: u64,
    pub persisted_count: u64,
    pub persisted_size_bytes: u64,
    pub swap_in_count: u64,
    pub swap_out_count: u64,
    pub hit_count: u64,
    pub miss_count: u64,
}