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

//! # IPC 共享内存类型定义
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

use crate::task::task::TaskId;

/// 共享内存 ID
pub type SharedMemoryId = alloc::string::String;

/// 共享内存区域
#[derive(Debug, Clone)]
pub struct SharedMemoryRegion {
    pub id: SharedMemoryId,
    pub task_id: TaskId,
    pub size: usize,
    pub flags: ShmFlags,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_access: Option<chrono::DateTime<chrono::Utc>>,
}

/// 共享内存标志
#[derive(Debug, Clone)]
pub struct ShmFlags {
    /// 只读
    pub readonly: bool,

    /// 持久化 (重启后保留)
    pub persistent: bool,

    /// 自动销毁 (所有引用释放后)
    pub auto_destroy: bool,

    /// 使用大页
    pub huge_pages: bool,
}

impl Default for ShmFlags {
    fn default() -> Self {
        Self {
            readonly: false,
            persistent: false,
            auto_destroy: true,
            huge_pages: false,
        }
    }
}

/// 共享内存统计
#[derive(Debug, Clone, Default)]
pub struct ShmStats {
    pub total_regions: u64,
    pub total_size_bytes: u64,
    pub active_regions: u64,
    pub peak_size_bytes: u64,
}