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

//! # SubTask 类型定义
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

use crate::task::task::TaskId;

/// SubTask ID
pub type SubTaskId = alloc::string::String;

/// 子任务 — 执行调度的基本单位
#[derive(Debug, Clone)]
pub struct SubTask {
    pub id: SubTaskId,
    pub name: alloc::string::String,
    pub description: Option<alloc::string::String>,

    /// 所属 Task
    pub task_id: TaskId,

    /// 依赖的 SubTask ID 列表 (DAG)
    pub dependencies: alloc::vec::Vec<SubTaskId>,

    /// 被依赖的 SubTask ID 列表 (DAG)
    pub dependents: alloc::vec::Vec<SubTaskId>,

    /// 调度优先级
    pub priority: Option<SubTaskPriority>,

    /// 当前状态
    pub status: SubTaskStatus,

    /// 执行结果
    pub result: Option<SubTaskResult>,

    /// 执行日志
    pub logs: alloc::vec::Vec<alloc::string::String>,

    pub created_at: chrono::DateTime<chrono::Utc>,
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// SubTask 优先级
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SubTaskPriority {
    High = 0,
    Normal = 1,
    Low = 2,
}

/// SubTask 状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SubTaskStatus {
    Pending,      // 等待依赖满足
    Ready,        // 依赖已满足，等待调度
    Running,      // 执行中
    Blocked,      // 阻塞等待
    Completed,    // 执行完成
    Failed,       // 执行失败
    Canceled,     // 被取消
}

/// SubTask 执行结果
#[derive(Debug, Clone)]
pub struct SubTaskResult {
    pub status: SubTaskStatus,
    pub data: Option<alloc::vec::Vec<u8>>,
    pub error: Option<alloc::string::String>,
    pub metrics: SubTaskMetrics,
}

/// SubTask 执行指标
#[derive(Debug, Clone, Default)]
pub struct SubTaskMetrics {
    pub tokens_consumed: u64,
    pub gpu_memory_used_bytes: u64,
    pub execution_ms: u64,
    pub retry_count: u32,
}