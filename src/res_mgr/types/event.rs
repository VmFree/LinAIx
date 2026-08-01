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

use super::*;

/// 资源事件
#[derive(Debug, Clone)]
pub enum ResourceEvent {
    /// 配额超限
    QuotaExceeded {
        agent_id: AgentId,
        resource_type: ResourceType,
        limit: u64,
        current: u64,
        exceeded_by: u64,
    },

    /// 配额接近上限 (阈值触发)
    QuotaWarning {
        agent_id: AgentId,
        resource_type: ResourceType,
        limit: u64,
        current: u64,
        threshold_percent: u8,
    },

    /// 资源压力变化
    PressureChanged {
        resource_type: ResourceType,
        old_level: PressureLevel,
        new_level: PressureLevel,
        usage_ratio: f32,
    },

    /// KV Cache 换出
    KvCacheSwappedOut {
        session_id: SessionId,
        model_id: ModelId,
        size_bytes: usize,
        target: StorageLocation,
        duration_ms: u64,
    },

    /// KV Cache 换入
    KvCacheSwappedIn {
        session_id: SessionId,
        model_id: ModelId,
        size_bytes: usize,
        source: StorageLocation,
        duration_ms: u64,
    },

    /// KV Cache 分配
    KvCacheAllocated {
        session_id: SessionId,
        model_id: ModelId,
        size_bytes: usize,
        location: StorageLocation,
    },

    /// KV Cache 释放
    KvCacheFreed {
        session_id: SessionId,
        model_id: ModelId,
        size_bytes: usize,
        reason: FreeReason,
    },

    /// 模型加载完成
    ModelLoaded {
        model_id: ModelId,
        load_time_ms: u64,
        memory_used_bytes: u64,
        device_id: u32,
    },

    /// 模型加载失败
    ModelLoadFailed {
        model_id: ModelId,
        error: alloc::string::String,
        retry_count: u32,
    },

    /// 模型卸载
    ModelUnloaded {
        model_id: ModelId,
        reason: UnloadReason,
        memory_freed_bytes: u64,
    },

    /// 模型从缓存中淘汰
    ModelEvicted {
        model_id: ModelId,
        reason: EvictionReason,
        cached_duration_ms: u64,
    },
}

/// 释放原因
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FreeReason {
    /// 会话结束
    SessionEnded,
    /// 显存不足
    MemoryPressure,
    /// 手动释放
    Manual,
    /// 超时
    Timeout,
}

/// 卸载原因
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnloadReason {
    /// 手动卸载
    Manual,
    /// 显存不足
    MemoryPressure,
    /// 模型更新
    ModelUpdated,
    /// Agent 终止
    AgentTerminated,
    /// 超时
    Timeout,
}

/// 淘汰原因
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvictionReason {
    /// LRU 策略淘汰
    Lru,
    /// LFU 策略淘汰
    Lfu,
    /// 容量限制
    Capacity,
    /// 手动淘汰
    Manual,
}

/// 资源事件回调
pub trait ResourceEventCallback: Send + Sync {
    fn on_event(&self, event: &ResourceEvent);
}