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

//! # 资源事件类型定义
//!
//! ## Version
//! 0.1.0
//!
//! ## Author
//! VmFree <vmfree@example.com>
//!
//! ## Date
//! 2026-08-01

use crate::task::{TaskId, SubTaskId};
use crate::res_mgr::quota::types::{ResourceType};
use crate::res_mgr::kv_cache::types::{SessionId, ModelId, StorageLocation};
use crate::res_mgr::stats::types::PressureLevel;

/// 资源事件
#[derive(Debug, Clone)]
pub enum ResourceEvent {
    QuotaExceeded {
        subtask_id: SubTaskId,
        resource_type: ResourceType,
        limit: u64,
        current: u64,
        exceeded_by: u64,
    },
    QuotaWarning {
        subtask_id: SubTaskId,
        resource_type: ResourceType,
        limit: u64,
        current: u64,
        threshold_percent: u8,
    },
    PressureChanged {
        resource_type: ResourceType,
        old_level: PressureLevel,
        new_level: PressureLevel,
        usage_ratio: f32,
    },
    KvCacheSwappedOut {
        session_id: SessionId,
        model_id: ModelId,
        size_bytes: usize,
        target: StorageLocation,
        duration_ms: u64,
    },
    KvCacheSwappedIn {
        session_id: SessionId,
        model_id: ModelId,
        size_bytes: usize,
        source: StorageLocation,
        duration_ms: u64,
    },
    KvCacheAllocated {
        session_id: SessionId,
        model_id: ModelId,
        size_bytes: usize,
        location: StorageLocation,
    },
    KvCacheFreed {
        session_id: SessionId,
        model_id: ModelId,
        size_bytes: usize,
        reason: FreeReason,
    },
    ModelLoaded {
        model_id: ModelId,
        load_time_ms: u64,
        memory_used_bytes: u64,
        device_id: u32,
    },
    ModelLoadFailed {
        model_id: ModelId,
        error: alloc::string::String,
        retry_count: u32,
    },
    ModelUnloaded {
        model_id: ModelId,
        reason: UnloadReason,
        memory_freed_bytes: u64,
    },
    ModelEvicted {
        model_id: ModelId,
        reason: EvictionReason,
        cached_duration_ms: u64,
    },
}

/// 释放原因
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FreeReason {
    SessionEnded,
    MemoryPressure,
    Manual,
    Timeout,
}

/// 卸载原因
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnloadReason {
    Manual,
    MemoryPressure,
    ModelUpdated,
    AgentTerminated,
    Timeout,
}

/// 淘汰原因
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvictionReason {
    Lru,
    Lfu,
    Capacity,
    Manual,
}

/// 资源事件回调
pub trait ResourceEventCallback: Send + Sync {
    fn on_event(&self, event: &ResourceEvent);
}