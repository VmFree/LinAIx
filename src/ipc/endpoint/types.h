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

//! # IPC 端点类型定义
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

use crate::task::subtask::SubTaskId;
use crate::ipc::channel::ChannelId;

/// 端点 ID
pub type EndpointId = alloc::string::String;

/// 端点角色
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EndpointRole {
    /// 发送者
    Sender,

    /// 接收者
    Receiver,

    /// 双向
    Both,
}

/// 端点状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EndpointStatus {
    /// 未绑定
    Unbound,

    /// 已绑定 (活跃)
    Bound,

    /// 已暂停
    Paused,

    /// 已断开
    Disconnected,
}

/// IPC 端点
#[derive(Debug, Clone)]
pub struct Endpoint {
    pub id: EndpointId,
    pub subtask_id: SubTaskId,
    pub channel_id: Option<ChannelId>,
    pub role: EndpointRole,
    pub status: EndpointStatus,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_activity: Option<chrono::DateTime<chrono::Utc>>,
}