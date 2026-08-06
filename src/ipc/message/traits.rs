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

//! # IPC 消息接口
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
use crate::ipc::error::IpcError;

/// IPC 消息工厂接口
pub trait IpcMessageFactory: Send + Sync {
    /// 创建消息
    fn create(
        &self,
        channel_id: &ChannelId,
        from: &EndpointId,
        to: &EndpointId,
        msg_type: MessageType,
        payload: alloc::vec::Vec<u8>,
        priority: Option<MessagePriority>,
    ) -> Message;

    /// 创建请求消息
    fn create_request(
        &self,
        channel_id: &ChannelId,
        from: &EndpointId,
        to: &EndpointId,
        payload: alloc::vec::Vec<u8>,
    ) -> Message;

    /// 创建响应消息
    fn create_response(
        &self,
        reply_to: &MessageId,
        channel_id: &ChannelId,
        from: &EndpointId,
        to: &EndpointId,
        payload: alloc::vec::Vec<u8>,
    ) -> Message;

    /// 创建事件消息
    fn create_event(
        &self,
        channel_id: &ChannelId,
        from: &EndpointId,
        payload: alloc::vec::Vec<u8>,
    ) -> Message;
}