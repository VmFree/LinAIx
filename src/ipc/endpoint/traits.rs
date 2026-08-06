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

//! # IPC 端点接口
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
use crate::ipc::channel::ChannelId;
use crate::ipc::error::IpcError;

/// IPC 端点管理接口
pub trait IpcEndpointManager: Send + Sync {
    /// 创建端点
    fn create(&mut self, subtask_id: &SubTaskId, role: EndpointRole) -> Result<EndpointId, IpcError>;

    /// 删除端点
    fn delete(&mut self, endpoint_id: &EndpointId) -> Result<(), IpcError>;

    /// 绑定端点到通道
    fn bind(&mut self, endpoint_id: &EndpointId, channel_id: &ChannelId) -> Result<(), IpcError>;

    /// 解绑端点
    fn unbind(&mut self, endpoint_id: &EndpointId) -> Result<(), IpcError>;

    /// 获取端点信息
    fn get(&self, endpoint_id: &EndpointId) -> Result<Endpoint, IpcError>;

    /// 获取 SubTask 的所有端点
    fn list_by_subtask(&self, subtask_id: &SubTaskId) -> alloc::vec::Vec<Endpoint>;

    /// 获取通道的所有端点
    fn list_by_channel(&self, channel_id: &ChannelId) -> alloc::vec::Vec<Endpoint>;

    /// 检查端点是否存在
    fn exists(&self, endpoint_id: &EndpointId) -> bool;

    /// 检查端点是否已绑定
    fn is_bound(&self, endpoint_id: &EndpointId) -> bool;
}

/// IPC 端点操作接口
pub trait IpcEndpoint: Send + Sync {
    /// 获取端点 ID
    fn id(&self) -> &EndpointId;

    /// 获取绑定的 SubTask ID
    fn subtask_id(&self) -> &SubTaskId;

    /// 获取绑定的通道 ID
    fn channel_id(&self) -> Option<ChannelId>;

    /// 获取端点角色
    fn role(&self) -> EndpointRole;

    /// 获取端点状态
    fn status(&self) -> EndpointStatus;

    /// 绑定到通道
    fn bind(&mut self, channel_id: &ChannelId) -> Result<(), IpcError>;

    /// 解绑
    fn unbind(&mut self) -> Result<(), IpcError>;

    /// 检查是否已绑定
    fn is_bound(&self) -> bool;

    /// 检查是否已连接
    fn is_connected(&self) -> bool;

    /// 获取端点统计
    fn stats(&self) -> EndpointStats;
}

/// 端点统计
#[derive(Debug, Clone, Default)]
pub struct EndpointStats {
    pub messages_sent: u64,
    pub messages_received: u64,
    pub bytes_sent: u64,
    pub bytes_received: u64,
}