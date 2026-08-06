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

//! # 统计类型定义
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

use crate::res_mgr::quota::types::{ResourceType};
use crate::task::{TaskId, SubTaskId};

/// Agent 资源使用情况
#[derive(Debug, Clone)]
pub struct AgentResourceUsage {
    pub task_id: TaskId,
    pub subtask_id: SubTaskId,
    pub gpu_memory_used_bytes: u64,
    pub system_memory_used_bytes: u64,
    pub disk_storage_used_bytes: u64,
    pub tokens_used_this_minute: u64,
    pub concurrent_sessions: u32,
    pub active_model_instances: u32,
    pub kv_cache_count: u32,
    pub kv_cache_total_bytes: u64,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// 全局资源使用情况
#[derive(Debug, Clone)]
pub struct GlobalResourceUsage {
    pub total_gpu_memory_bytes: u64,
    pub used_gpu_memory_bytes: u64,
    pub total_system_memory_bytes: u64,
    pub used_system_memory_bytes: u64,
    pub total_disk_storage_bytes: u64,
    pub used_disk_storage_bytes: u64,
    pub active_agents: u32,
    pub total_sessions: u32,
    pub total_model_instances: u32,
    pub swap_in_rate: f32,
    pub swap_out_rate: f32,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// 资源压力等级
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum PressureLevel {
    Low = 0,
    Medium = 1,
    High = 2,
    Critical = 3,
}

/// 资源压力趋势
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PressureTrend {
    Rising,
    Stable,
    Falling,
}

/// 资源压力指标
#[derive(Debug, Clone)]
pub struct ResourcePressure {
    pub resource_type: ResourceType,
    pub pressure_level: PressureLevel,
    pub usage_ratio: f32,
    pub trend: PressureTrend,
    pub predicted_exhaustion_time: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// 历史数据点
#[derive(Debug, Clone)]
pub struct HistoricalDataPoint {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub resource_type: ResourceType,
    pub value: f64,
    pub metadata: std::collections::HashMap<alloc::string::String, alloc::string::String>,
}

/// 时间间隔
#[derive(Debug, Clone, Copy)]
pub struct Duration {
    pub seconds: u64,
}