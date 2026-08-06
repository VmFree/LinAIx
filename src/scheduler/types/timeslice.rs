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

//! # 时间片类型定义
//!
//! 参考 Linux 的时间片 (timeslice) 设计。
//!
//! ## Version
//! 0.1.0
//!
//! ## Author
//! VmFree <vmfree@example.com>
//!
//! ## Date
//! 2026-08-01

use core::time::Duration;

/// 时间片配置
#[derive(Debug, Clone, Copy)]
pub struct TimeSliceConfig {
    /// 默认时间片长度
    pub default_ms: u64,

    /// 时间片最小值
    pub min_ms: u64,

    /// 时间片最大值
    pub max_ms: u64,

    /// 是否启用动态时间片调整
    pub dynamic: bool,
}

impl Default for TimeSliceConfig {
    fn default() -> Self {
        Self {
            default_ms: 100,
            min_ms: 10,
            max_ms: 500,
            dynamic: true,
        }
    }
}

impl TimeSliceConfig {
    pub fn default_timeslice(&self) -> Duration {
        Duration::from_millis(self.default_ms)
    }

    pub fn timeslice_for_priority(&self, priority: SchedulingPriority) -> Duration {
        // 高优先级获得较长的时间片
        let ms = match priority {
            SchedulingPriority::Realtime => self.max_ms,
            SchedulingPriority::High => self.default_ms + 50,
            SchedulingPriority::Normal => self.default_ms,
            SchedulingPriority::Low => self.default_ms - 30,
            SchedulingPriority::Background => self.min_ms,
        };
        Duration::from_millis(ms)
    }
}