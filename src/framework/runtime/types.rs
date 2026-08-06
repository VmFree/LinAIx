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

//! # 框架运行时类型定义
//!
//! ## Version
//! 0.1.0

use crate::task::TaskId;
use crate::framework::framework::{FrameworkId, FrameworkType};

/// 框架运行时句柄
pub type RuntimeHandle = alloc::string::String;

/// 框架运行时
#[derive(Debug, Clone)]
pub struct FrameworkRuntime {
    pub handle: RuntimeHandle,
    pub framework_id: FrameworkId,
    pub framework_type: FrameworkType,
    pub task_id: TaskId,
    pub pid: u32,
    pub status: RuntimeStatus,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// 运行时状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RuntimeStatus {
    Starting,
    Running,
    Paused,
    Stopping,
    Terminated,
    Error,
}