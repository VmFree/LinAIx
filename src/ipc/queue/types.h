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

//! # IPC 队列类型定义
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

/// 队列 ID
pub type QueueId = alloc::string::String;

/// 队列配置
#[derive(Debug, Clone)]
pub struct QueueConfig {
    /// 最大队列大小
    pub max_size: usize,

    /// 是否启用消息优先级
    pub priority_enabled: bool,

    /// 是否启用持久化
    pub persistent: bool,

    /// 消息 TTL (毫秒)
    pub ttl_ms: Option<u64>,
}

impl Default for QueueConfig {
    fn default() -> Self {
        Self {
            max_size: 1000,
            priority_enabled: true,
            persistent: false,
            ttl_ms: None,
        }
    }
}

/// 队列统计信息
#[derive(Debug, Clone, Default)]
pub struct QueueStats {
    pub current_size: usize,
    pub max_size: usize,
    pub total_enqueued: u64,
    pub total_dequeued: u64,
    pub total_dropped: u64,
    pub last_activity: Option<chrono::DateTime<chrono::Utc>>,
}