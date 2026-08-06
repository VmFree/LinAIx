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

//! # IPC 消息类型定义
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

use super::super::channel::ChannelId;
use super::super::endpoint::EndpointId;

/// 消息 ID
pub type MessageId = alloc::string::String;

/// 消息类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessageType {
    /// 请求 (请求-响应模式)
    Request,

    /// 响应 (请求-响应模式)
    Response,

    /// 事件 (单向通知)
    Event,

    /// 流式数据块 (流式传输)
    StreamChunk,

    /// 流结束标志
    StreamEnd,

    /// 心跳
    Heartbeat,

    /// 确认
    Ack,

    /// 自定义
    Custom(alloc::string::String),
}

/// 消息优先级
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum MessagePriority {
    /// 最高 (实时)
    Realtime = 0,

    /// 高
    High = 1,

    /// 普通 (默认)
    Normal = 2,

    /// 低
    Low = 3,

    /// 最低 (背景)
    Background = 4,
}

impl Default for MessagePriority {
    fn default() -> Self {
        MessagePriority::Normal
    }
}

/// IPC 消息
#[derive(Debug, Clone)]
pub struct Message {
    pub id: MessageId,
    pub channel_id: ChannelId,
    pub from: EndpointId,
    pub to: EndpointId,
    pub msg_type: MessageType,
    pub priority: MessagePriority,
    pub payload: alloc::vec::Vec<u8>,
    pub trace_id: alloc::string::String,
    pub reply_to: Option<MessageId>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub ttl_ms: Option<u64>,
}

/// 消息确认
#[derive(Debug, Clone)]
pub struct MessageAck {
    pub message_id: MessageId,
    pub channel_id: ChannelId,
    pub from: EndpointId,
    pub status: AckStatus,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// 确认状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AckStatus {
    /// 成功接收
    Received,

    /// 已处理
    Processed,

    /// 拒绝
    Rejected,

    /// 超时
    Timeout,

    /// 错误
    Error,
}