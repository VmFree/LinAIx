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

//! # 资源管理错误类型
//!
//! ## Version
//! 0.1.0
//!
//! ## Author
//! VmFree <vmfree@example.com>
//!
//! ## Date
//! 2026-08-01

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ResourceError {
    // ===== 模型资源 =====
    #[error("Model not found: {0}")]
    ModelNotFound(alloc::string::String),

    #[error("Model load failed: {0}")]
    ModelLoadFailed(alloc::string::String),

    #[error("Model unload failed: {0}")]
    ModelUnloadFailed(alloc::string::String),

    #[error("Model instance pool exhausted: max={max}, requested={requested}")]
    InstancePoolExhausted { max: usize, requested: usize },

    #[error("Model instance not found: {0}")]
    InstanceNotFound(alloc::string::String),

    #[error("Model already loaded: {0}")]
    ModelAlreadyLoaded(alloc::string::String),

    #[error("Model not loaded: {0}")]
    ModelNotLoaded(alloc::string::String),

    // ===== KV Cache =====
    #[error("KV Cache allocation failed: {0}")]
    KvCacheAllocFailed(alloc::string::String),

    #[error("KV Cache not found: {0}")]
    KvCacheNotFound(alloc::string::String),

    #[error("KV Cache swap out failed: {0}")]
    KvCacheSwapOutFailed(alloc::string::String),

    #[error("KV Cache swap in failed: {0}")]
    KvCacheSwapInFailed(alloc::string::String),

    #[error("KV Cache persist failed: {0}")]
    KvCachePersistFailed(alloc::string::String),

    #[error("KV Cache restore failed: {0}")]
    KvCacheRestoreFailed(alloc::string::String),

    // ===== 配额 =====
    #[error("Quota exceeded: {resource_type} limit={limit}, current={current}")]
    QuotaExceeded {
        resource_type: crate::res_mgr::types::ResourceType,
        limit: u64,
        current: u64,
    },

    #[error("Quota not found for agent: {0}")]
    QuotaNotFound(alloc::string::String),

    #[error("Invalid quota: {0}")]
    InvalidQuota(alloc::string::String),

    // ===== 统计 =====
    #[error("Statistics collection failed: {0}")]
    StatsCollectionFailed(alloc::string::String),

    #[error("Event subscription failed: {0}")]
    EventSubscriptionFailed(alloc::string::String),

    // ===== 内存 =====
    #[error("Insufficient memory: requested={requested}, available={available}")]
    InsufficientMemory { requested: u64, available: u64 },

    #[error("Memory allocation failed: {0}")]
    MemoryAllocationFailed(alloc::string::String),

    // ===== 通用 =====
    #[error("Internal error: {0}")]
    Internal(alloc::string::String),

    #[error("Configuration error: {0}")]
    ConfigError(alloc::string::String),

    #[error("Timeout: {0}")]
    Timeout(alloc::string::String),
}