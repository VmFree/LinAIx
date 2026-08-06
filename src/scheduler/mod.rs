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

//! # L3 调度器 (Scheduler)
//!
//! 参考 Linux 调度器设计，负责 SubTask 的调度执行。
//!
//! ## 核心概念
//!
//! | 概念 | 说明 | Linux 对应 |
//! |------|------|-----------|
//! | RunQueue | 存放就绪的 SubTask | runqueue |
//! | WaitQueue | 存放等待事件的 SubTask | waitqueue |
//! | SchedClass | 调度策略类 | sched_class |
//! | SchedulingPriority | 调度优先级 | priority/nice |
//! | TimeSlice | 时间片 | timeslice |
//! | Preemption | 抢占 | preempt |
//!
//! ## 核心接口
//! - [`Scheduler`]：调度器主接口
//! - [`SchedClass`]：调度类接口
//!
//! ## 版本
//! 0.1.0
//!
//! ## Author
//! VmFree <vmfree@example.com>
//!
//! ## Date
//! 2026-08-01

pub mod error;
pub mod types;
pub mod traits;

// 具体实现 (预留)
// pub mod default;

pub use traits::*;
pub use types::*;
pub use error::SchedulerError;