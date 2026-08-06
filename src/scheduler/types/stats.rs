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

//! # 调度器统计类型定义
//!
//! ## Version
//! 0.1.0
//!
//! ## Author
//! VmFree <vmfree@example.com>
//!
//! ## Date
//! 2026-08-01

/// 调度器统计信息
#[derive(Debug, Clone, Default)]
pub struct SchedulerStats {
    pub total_schedule_calls: u64,
    pub total_context_switches: u64,
    pub total_preemptions: u64,
    pub total_wakeups: u64,
    pub avg_schedule_latency_ms: f64,
    pub last_schedule_time: Option<chrono::DateTime<chrono::Utc>>,
}

/// 调度器状态
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SchedulerStatus {
    Running,
    Paused,
    Stopped,
    Error(alloc::string::String),
}