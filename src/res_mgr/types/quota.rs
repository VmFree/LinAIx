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

//! # 配额类型定义
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

/// Agent ID
pub type AgentId = alloc::string::String;

/// 资源类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ResourceType {
    /// GPU 显存
    GpuMemory,
    /// 系统内存
    SystemMemory,
    /// 磁盘存储
    DiskStorage,
    /// Token 速率
    Token,
    /// 并发会话数
    Concurrency,
    /// 请求速率
    RequestRate,
    /// 模型实例数
    ModelInstances,
}

/// 资源配额
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceQuota {
    pub agent_id: AgentId,

    /// GPU 显存上限 (字节)
    pub max_gpu_memory_bytes: Option<u64>,

    /// 系统内存上限 (字节)
    pub max_system_memory_bytes: Option<u64>,

    /// 磁盘存储上限 (字节)
    pub max_disk_storage_bytes: Option<u64>,

    /// Token 速率上限 (每分钟)
    pub max_tokens_per_minute: Option<u64>,

    /// 并发会话数上限
    pub max_concurrent_sessions: Option<u32>,

    /// 请求速率上限 (每秒)
    pub max_requests_per_second: Option<u32>,

    /// 模型实例数上限
    pub max_model_instances: Option<u32>,

    /// 创建时间
    pub created_at: chrono::DateTime<chrono::Utc>,

    /// 最后更新时间
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// 资源请求
#[derive(Debug, Clone)]
pub struct ResourceRequest {
    pub resource_type: ResourceType,
    pub amount: u64,
    pub agent_id: AgentId,
}

/// 资源使用量
#[derive(Debug, Clone, Default)]
pub struct ResourceUsage {
    pub gpu_memory_used_bytes: u64,
    pub system_memory_used_bytes: u64,
    pub disk_storage_used_bytes: u64,
    pub tokens_used_this_minute: u64,
    pub concurrent_sessions: u32,
    pub requests_this_second: u32,
    pub model_instances: u32,
}

/// 超限详情
#[derive(Debug, Clone)]
pub struct ExceededDetail {
    pub resource_type: ResourceType,
    pub limit: u64,
    pub current: u64,
    pub exceeded_by: u64,
    pub agent_id: AgentId,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// 配额状态
#[derive(Debug, Clone)]
pub struct QuotaStatus {
    pub agent_id: AgentId,
    pub quota: ResourceQuota,
    pub usage: ResourceUsage,
    pub usage_ratios: std::collections::HashMap<ResourceType, f32>,
    pub is_exceeded: bool,
    pub exceeded_details: alloc::vec::Vec<ExceededDetail>,
}