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

//! # IPC 队列接口
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
use crate::ipc::error::IpcError;

/// IPC 队列接口
pub trait IpcQueue: Send + Sync {
    /// 获取队列 ID
    fn id(&self) -> &QueueId;

    /// 入队 (尾部)
    fn enqueue(&mut self, msg: Message) -> Result<(), IpcError>;

    /// 出队 (头部)
    fn dequeue(&mut self) -> Result<Option<Message>, IpcError>;

    /// 预览头部消息 (不移除)
    fn peek(&self) -> Result<Option<&Message>, IpcError>;

    /// 获取队列大小
    fn size(&self) -> usize;

    /// 检查队列是否为空
    fn is_empty(&self) -> bool;

    /// 检查队列是否已满
    fn is_full(&self) -> bool;

    /// 清空队列
    fn clear(&mut self) -> Result<(), IpcError>;

    /// 获取统计信息
    fn stats(&self) -> QueueStats;
}

/// 优先级队列接口
pub trait IpcPriorityQueue: IpcQueue {
    /// 按优先级入队 (高优先级优先出队)
    fn enqueue_with_priority(&mut self, msg: Message) -> Result<(), IpcError>;

    /// 获取消息按优先级的分布
    fn priority_distribution(&self) -> std::collections::HashMap<MessagePriority, usize>;
}