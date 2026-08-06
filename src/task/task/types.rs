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

//! # Task 类型定义
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

use crate::framework::types::FrameworkId;

/// Task ID
pub type TaskId = alloc::string::String;

/// 任务 — 资源管理的基本单位
#[derive(Debug, Clone)]
pub struct Task {
    pub id: TaskId,
    pub name: alloc::string::String,
    pub description: Option<alloc::string::String>,

    pub framework_id: Option<FrameworkId>,
    /// 该 Task 下的所有 SubTask
    pub subtasks: alloc::vec::Vec<SubTaskId>,

    /// 任务配额
    pub quota: TaskQuota,

    /// 任务当前资源使用量
    pub usage: TaskUsage,

    /// 共享 KV Cache 句柄
    pub shared_kv_cache_handle: Option<alloc::string::String>,

    /// 任务级优先级
    pub priority: TaskPriority,

    /// 任务状态
    pub status: TaskStatus,

    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// 任务配额
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskQuota {
    pub max_gpu_memory_bytes: Option<u64>,
    pub max_system_memory_bytes: Option<u64>,
    pub max_tokens: Option<u64>,
    pub max_tokens_per_minute: Option<u64>,
    pub max_concurrent_subtasks: Option<u32>,
    pub max_subtask_count: Option<u32>,
    pub max_execution_seconds: Option<u64>,
}

/// 任务资源使用量
#[derive(Debug, Clone, Default)]
pub struct TaskUsage {
    pub gpu_memory_used_bytes: u64,
    pub system_memory_used_bytes: u64,
    pub tokens_used_this_minute: u64,
    pub active_subtasks: u32,
}

/// 任务资源请求
#[derive(Debug, Clone)]
pub struct TaskResourceRequest {
    pub task_id: TaskId,
    pub gpu_memory_bytes: Option<u64>,
    pub system_memory_bytes: Option<u64>,
    pub tokens: Option<u64>,
}

/// 任务优先级
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum TaskPriority {
    Critical = 0,
    High = 1,
    Normal = 2,
    Low = 3,
    Background = 4,
}

/// 任务状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskStatus {
    Created,
    Active,
    Suspended,
    Terminated,
}