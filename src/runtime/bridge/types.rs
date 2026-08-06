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

//! # 系统调用桥类型定义
//!
//! ## Version
//! 0.1.0

use serde::{Deserialize, Serialize};

/// 系统调用类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SystemCallType {
    /// Skill 调用
    SkillInvoke,

    /// 模型推理
    ModelInfer,

    /// 资源操作
    ResourceAllocate,
    ResourceFree,

    /// IPC 操作
    IpcSend,
    IpcRecv,

    /// 任务操作
    TaskStatus,

    /// 自定义
    Custom(alloc::string::String),
}

/// 系统调用
#[derive(Debug, Clone)]
pub struct SystemCall {
    pub call_type: SystemCallType,
    pub agent_id: AgentId,
    pub subtask_id: SubTaskId,
    pub task_id: TaskId,
    pub params: serde_json::Value,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// 系统调用结果
#[derive(Debug, Clone)]
pub struct SystemCallResult {
    pub success: bool,
    pub data: Option<serde_json::Value>,
    pub error: Option<alloc::string::String>,
    pub duration_ms: u64,
}