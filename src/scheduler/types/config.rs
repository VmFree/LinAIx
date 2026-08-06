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

//! # 调度器配置
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

/// 调度器配置
#[derive(Debug, Clone)]
pub struct SchedulerConfig {
    /// 调度类配置
    pub sched_class: SchedClassConfig,

    /// 时间片配置
    pub timeslice: TimeSliceConfig,

    /// 运行队列最大长度
    pub max_runqueue_size: usize,

    /// 是否启用抢占
    pub preemption_enabled: bool,

    /// 抢占检查间隔 (毫秒)
    pub preemption_check_ms: u64,

    /// 是否启用优先级提升 (防饥饿)
    pub priority_boost_enabled: bool,

    /// 优先级提升间隔 (毫秒)
    pub priority_boost_ms: u64,

    /// 是否启用负载均衡
    pub load_balance_enabled: bool,
}

impl Default for SchedulerConfig {
    fn default() -> Self {
        Self {
            sched_class: SchedClassConfig::default(),
            timeslice: TimeSliceConfig::default(),
            max_runqueue_size: 10000,
            preemption_enabled: true,
            preemption_check_ms: 100,
            priority_boost_enabled: true,
            priority_boost_ms: 1000,
            load_balance_enabled: false,
        }
    }
}

/// 调度类配置
#[derive(Debug, Clone)]
pub struct SchedClassConfig {
    pub realtime_enabled: bool,
    pub realtime_priority: SchedulingPriority,
    pub fair_enabled: bool,
    pub idle_enabled: bool,
}

impl Default for SchedClassConfig {
    fn default() -> Self {
        Self {
            realtime_enabled: true,
            realtime_priority: SchedulingPriority::Realtime,
            fair_enabled: true,
            idle_enabled: true,
        }
    }
}