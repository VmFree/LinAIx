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

//! # Agent 类型定义
//!
//! ## Version
//! 0.1.0

use serde::{Deserialize, Serialize};

use crate::task::subtask::SubTaskId;
use crate::task::task::TaskId;
use crate::framework::framework::FrameworkId;
use super::super::config::AgentConfig;
use super::super::lifecycle::LifecycleState;

/// Agent ID (运行时实例标识)
pub type AgentId = alloc::string::String;

/// Agent 运行时实例
///
/// Agent 是 SubTask 执行时的具体存在，绑定到 SubTask (1:1)。
#[derive(Debug, Clone)]
pub struct Agent {
    pub id: AgentId,
    pub subtask_id: SubTaskId,
    pub task_id: TaskId,
    pub framework_id: Option<FrameworkId>,
    pub config: AgentConfig,
    pub status: AgentStatus,
    pub lifecycle_state: LifecycleState,
    pub metrics: AgentMetrics,
    pub pid: Option<u32>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// Agent 状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentStatus {
    Created,      // 已创建，未启动
    Starting,     // 正在初始化
    Running,      // 正在执行
    Blocked,      // 阻塞等待
    Pausing,      // 正在暂停
    Paused,       // 已暂停
    Resuming,     // 正在恢复
    Stopping,     // 正在停止
    Terminated,   // 已终止
    Error,        // 错误状态
}

/// Agent 运行时指标
#[derive(Debug, Clone, Default)]
pub struct AgentMetrics {
    pub tokens_consumed: u64,
    pub gpu_memory_used_bytes: u64,
    pub system_memory_used_bytes: u64,
    pub cpu_time_ms: u64,
    pub skill_calls: u64,
    pub model_calls: u64,
    pub ipc_messages_sent: u64,
    pub ipc_messages_received: u64,
    pub last_updated: Option<chrono::DateTime<chrono::Utc>>,
}