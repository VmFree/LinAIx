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

//! # 运行队列类型定义
//!
//! 参考 Linux 的 runqueue (rq) 结构。
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
use super::priority::SchedulingPriority;

/// 运行队列条目
#[derive(Debug, Clone)]
pub struct RunQueueEntry {
    pub subtask_id: SubTaskId,
    pub enqueued_at: chrono::DateTime<chrono::Utc>,
    pub priority: SchedulingPriority,
    pub deadline: Option<chrono::DateTime<chrono::Utc>>,
}

/// 运行队列统计信息
#[derive(Debug, Clone, Default)]
pub struct RunQueueStats {
    pub total_enqueued: u64,
    pub total_dequeued: u64,
    pub current_size: usize,
    pub max_size: usize,
    pub average_wait_time_ms: f64,
}