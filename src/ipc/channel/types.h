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

//! # IPC 通道类型定义
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

/// 通道 ID
pub type ChannelId = alloc::string::String;

/// 通道模式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChannelMode {
    /// 点对点 (两个端点)
    P2P,

    /// 广播 (一个发送者，多个接收者)
    Broadcast,

    /// 组播 (多个发送者，组内多个接收者)
    Multicast,

    /// 流式 (单向数据流)
    Stream,
}

/// 通道状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChannelStatus {
    /// 活跃
    Active,

    /// 已暂停
    Paused,

    /// 已关闭
    Closed,

    /// 错误
    Error,
}

/// 通道配置
#[derive(Debug, Clone)]
pub struct ChannelConfig {
    /// 通道模式
    pub mode: ChannelMode,

    /// 归属 Task
    pub task_id: TaskId,

    /// 最大消息数 (队列容量)
    pub max_messages: usize,

    /// 最大消息大小 (字节)
    pub max_message_size: usize,

    /// 是否启用消息确认
    pub ack_enabled: bool,

    /// 是否持久化 (重启后恢复)
    pub persistent: bool,

    /// 超时时间 (毫秒)
    pub timeout_ms: u64,
}

impl Default for ChannelConfig {
    fn default() -> Self {
        Self {
            mode: ChannelMode::P2P,
            task_id: TaskId::default(),
            max_messages: 1000,
            max_message_size: 1024 * 1024, // 1MB
            ack_enabled: true,
            persistent: false,
            timeout_ms: 5000,
        }
    }
}

/// 通道统计信息
#[derive(Debug, Clone, Default)]
pub struct ChannelStats {
    pub messages_sent: u64,
    pub messages_received: u64,
    pub messages_dropped: u64,
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub current_queue_size: usize,
    pub peak_queue_size: usize,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_activity: Option<chrono::DateTime<chrono::Utc>>,
}