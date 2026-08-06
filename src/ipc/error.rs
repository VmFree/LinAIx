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

//! # IPC 错误类型
//!
//! ## Version
//! 0.1.0
//!
//! ## Author
//! VmFree <vmfree@example.com>
//!
//! ## Date
//! 2026-08-01

use thiserror::Error;

#[derive(Debug, Error)]
pub enum IpcError {
    // ===== 通道错误 =====
    #[error("Channel not found: {0}")]
    ChannelNotFound(ChannelId),

    #[error("Channel already exists: {0}")]
    ChannelAlreadyExists(ChannelId),

    #[error("Channel closed")]
    ChannelClosed,

    #[error("Channel full")]
    ChannelFull,

    #[error("Channel empty")]
    ChannelEmpty,

    #[error("Channel mode mismatch: expected {expected:?}, actual {actual:?}")]
    ChannelModeMismatch {
        expected: ChannelMode,
        actual: ChannelMode,
    },

    // ===== 端点错误 =====
    #[error("Endpoint not found: {0}")]
    EndpointNotFound(EndpointId),

    #[error("Endpoint already bound: {0}")]
    EndpointAlreadyBound(EndpointId),

    #[error("Endpoint not bound")]
    EndpointNotBound,

    #[error("Endpoint disconnected")]
    EndpointDisconnected,

    // ===== 消息错误 =====
    #[error("Message too large: {size} bytes (max: {max})")]
    MessageTooLarge { size: usize, max: usize },

    #[error("Invalid message: {0}")]
    InvalidMessage(alloc::string::String),

    #[error("Message not acknowledged")]
    MessageNotAcknowledged,

    // ===== 队列错误 =====
    #[error("Queue full")]
    QueueFull,

    #[error("Queue empty")]
    QueueEmpty,

    #[error("Queue not found: {0}")]
    QueueNotFound(alloc::string::String),

    // ===== 信号量错误 =====
    #[error("Semaphore not found: {0}")]
    SemaphoreNotFound(SemaphoreId),

    #[error("Semaphore would block")]
    SemaphoreWouldBlock,

    #[error("Semaphore timeout")]
    SemaphoreTimeout,

    // ===== 共享内存错误 =====
    #[error("Shared memory not found: {0}")]
    SharedMemoryNotFound(SharedMemoryId),

    #[error("Shared memory allocation failed: {0}")]
    SharedMemoryAllocFailed(alloc::string::String),

    #[error("Shared memory permission denied")]
    SharedMemoryPermissionDenied,

    // ===== 权限错误 =====
    #[error("Permission denied: {0}")]
    PermissionDenied(alloc::string::String),

    // ===== 配额错误 =====
    #[error("Quota exceeded: {0}")]
    QuotaExceeded(alloc::string::String),

    // ===== 通用错误 =====
    #[error("Internal error: {0}")]
    Internal(alloc::string::String),

    #[error("Timeout: {0}")]
    Timeout(alloc::string::String),

    #[error("I/O error: {0}")]
    IoError(alloc::string::String),
}