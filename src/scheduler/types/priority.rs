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

//! # 优先级类型定义
//!
//! 参考 Linux 的优先级设计 (静态优先级 + 动态优先级)
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

/// 调度优先级 (数值越小优先级越高)
///
/// 对应 Linux 的静态优先级 (static_prio)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SchedulingPriority {
    /// 最高优先级 (实时任务)
    Realtime = 0,

    /// 高优先级
    High = 1,

    /// 普通优先级 (默认)
    Normal = 2,

    /// 低优先级
    Low = 3,

    /// 后台任务 (最低)
    Background = 4,
}

impl Default for SchedulingPriority {
    fn default() -> Self {
        SchedulingPriority::Normal
    }
}

impl From<u8> for SchedulingPriority {
    fn from(value: u8) -> Self {
        match value {
            0 => SchedulingPriority::Realtime,
            1 => SchedulingPriority::High,
            2 => SchedulingPriority::Normal,
            3 => SchedulingPriority::Low,
            _ => SchedulingPriority::Background,
        }
    }
}

/// 动态优先级
///
/// 用于多级反馈队列中的优先级调整
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct DynamicPriority {
    pub static_priority: SchedulingPriority,
    pub bonus: i8,  // -5 .. +5
}

impl DynamicPriority {
    pub fn effective(&self) -> SchedulingPriority {
        // 根据 bonus 调整优先级
        match (self.static_priority, self.bonus) {
            (SchedulingPriority::Realtime, _) => SchedulingPriority::Realtime,
            (SchedulingPriority::High, b) if b >= 3 => SchedulingPriority::Realtime,
            (SchedulingPriority::Low, b) if b <= -3 => SchedulingPriority::Background,
            (SchedulingPriority::Background, _) => SchedulingPriority::Background,
            _ => self.static_priority,
        }
    }
}