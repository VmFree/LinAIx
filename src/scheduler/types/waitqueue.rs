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

//! # 等待队列类型定义
//!
//! 参考 Linux 的 waitqueue (等待队列) 设计。
//!
//! 等待队列用于存放因等待特定事件（IPC、资源、依赖）而阻塞的 SubTask。
//!
//! ## Version
//! 0.1.0
//!
//! ## Author
//! VmFree <vmfree@example.com>
//!
//! ## Date
//! 2026-08-01

use crate::task::SubTaskId;
use crate::ipc::types::MessageId;

/// 等待原因
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WaitReason {
    /// 等待 IPC 消息
    IpcMessage { message_id: Option<MessageId>, from_subtask: Option<SubTaskId> },

    /// 等待资源
    Resource { resource_type: alloc::string::String },

    /// 等待依赖 SubTask
    Dependency { depends_on: SubTaskId },

    /// 等待 Task 配额
    TaskQuota,

    /// 等待定时器
    Timer { wake_at: chrono::DateTime<chrono::Utc> },

    /// 自定义等待事件
    Custom { event: alloc::string::String },
}

/// 等待队列条目
#[derive(Debug, Clone)]
pub struct WaitQueueEntry {
    pub subtask_id: SubTaskId,
    pub wait_reason: WaitReason,
    pub enqueued_at: chrono::DateTime<chrono::Utc>,
    pub timeout: Option<chrono::DateTime<chrono::Utc>>,
}

/// 等待队列统计信息
#[derive(Debug, Clone, Default)]
pub struct WaitQueueStats {
    pub total_waiting: u64,
    pub total_wakeups: u64,
    pub current_waiting: usize,
}