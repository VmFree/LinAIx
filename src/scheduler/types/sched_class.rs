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

//! # 调度类类型定义
//!
//! 参考 Linux 的 sched_class 设计。
//!
//! 调度类定义了不同调度策略的行为：
//! - Realtime: 实时调度 (SCHED_FIFO/SCHED_RR)
//! - Fair: 公平调度 (CFS)
//! - Idle: 空闲调度
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

/// 调度类类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SchedClassType {
    /// 实时调度类 (类似 SCHED_FIFO)
    Realtime,

    /// 公平调度类 (类似 CFS)
    Fair,

    /// 空闲调度类 (类似 SCHED_IDLE)
    Idle,
}

/// 调度类优先级 (数值越小优先级越高)
///
/// Linux 中调度类的优先级顺序:
/// stop_sched_class > dl_sched_class > rt_sched_class > fair_sched_class > idle_sched_class
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SchedClassPriority {
    Realtime = 0,
    Fair = 1,
    Idle = 2,
}

impl From<SchedClassType> for SchedClassPriority {
    fn from(class: SchedClassType) -> Self {
        match class {
            SchedClassType::Realtime => SchedClassPriority::Realtime,
            SchedClassType::Fair => SchedClassPriority::Fair,
            SchedClassType::Idle => SchedClassPriority::Idle,
        }
    }
}