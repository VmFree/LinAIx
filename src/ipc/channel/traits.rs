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

//! # IPC 通道接口
//!
//! ## Version
//! 0.1.0
//!
//! ## Author
//! VmFree <vmfree@example.com>
//!
//! ## Date
//! 2026-08-01

use super::types::*;
use crate::ipc::message::Message;
use crate::ipc::message::MessageId;
use crate::ipc::error::IpcError;

/// IPC 通道管理接口
pub trait IpcChannelManager: Send + Sync {
    /// 创建通道
    fn create(&mut self, config: ChannelConfig) -> Result<ChannelId, IpcError>;

    /// 删除通道
    fn delete(&mut self, channel_id: &ChannelId) -> Result<(), IpcError>;

    /// 获取通道配置
    fn get_config(&self, channel_id: &ChannelId) -> Result<ChannelConfig, IpcError>;

    /// 获取通道状态
    fn get_status(&self, channel_id: &ChannelId) -> Result<ChannelStatus, IpcError>;

    /// 获取通道统计
    fn get_stats(&self, channel_id: &ChannelId) -> Result<ChannelStats, IpcError>;

    /// 列出所有通道
    fn list(&self) -> alloc::vec::Vec<ChannelId>;

    /// 列出 Task 下的所有通道
    fn list_by_task(&self, task_id: &TaskId) -> alloc::vec::Vec<ChannelId>;

    /// 暂停通道 (停止消息传递)
    fn pause(&mut self, channel_id: &ChannelId) -> Result<(), IpcError>;

    /// 恢复通道
    fn resume(&mut self, channel_id: &ChannelId) -> Result<(), IpcError>;

    /// 关闭通道
    fn close(&mut self, channel_id: &ChannelId) -> Result<(), IpcError>;
}

/// IPC 通道操作接口
pub trait IpcChannel: Send + Sync {
    /// 获取通道 ID
    fn id(&self) -> &ChannelId;

    /// 获取通道配置
    fn config(&self) -> &ChannelConfig;

    /// 获取通道状态
    fn status(&self) -> ChannelStatus;

    /// 发送消息 (同步)
    fn send(&self, msg: Message) -> Result<MessageId, IpcError>;

    /// 发送消息 (异步)
    async fn send_async(&self, msg: Message) -> Result<MessageId, IpcError>;

    /// 接收消息 (同步，阻塞)
    fn recv(&self) -> Result<Message, IpcError>;

    /// 接收消息 (异步)
    async fn recv_async(&self) -> Result<Message, IpcError>;

    /// 尝试接收消息 (非阻塞)
    fn try_recv(&self) -> Result<Option<Message>, IpcError>;

    /// 确认消息已接收 (用于可靠消息)
    fn ack(&self, message_id: &MessageId) -> Result<(), IpcError>;

    /// 获取通道统计
    fn stats(&self) -> ChannelStats;

    /// 检查通道是否已关闭
    fn is_closed(&self) -> bool;

    /// 检查通道是否为空
    fn is_empty(&self) -> bool;
}