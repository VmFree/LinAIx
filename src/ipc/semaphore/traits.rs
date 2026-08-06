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

//! # IPC 信号量接口
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

/// IPC 信号量管理接口
pub trait IpcSemaphoreManager: Send + Sync {
    /// 创建信号量
    fn create(
        &mut self,
        semaphore_type: SemaphoreType,
        initial_value: usize,
    ) -> Result<SemaphoreId, IpcError>;

    /// 删除信号量
    fn delete(&mut self, semaphore_id: &SemaphoreId) -> Result<(), IpcError>;

    /// 获取信号量状态
    fn get_status(&self, semaphore_id: &SemaphoreId) -> Result<SemaphoreStatus, IpcError>;

    /// 列出所有信号量
    fn list(&self) -> alloc::vec::Vec<SemaphoreId>;
}

/// IPC 信号量操作接口
pub trait IpcSemaphore: Send + Sync {
    /// 获取信号量 ID
    fn id(&self) -> &SemaphoreId;

    /// 获取信号量类型
    fn semaphore_type(&self) -> SemaphoreType;

    /// 获取当前值
    fn value(&self) -> usize;

    /// 获取信号量状态
    fn status(&self) -> SemaphoreStatus;

    /// 等待 (P 操作)
    fn wait(&self) -> Result<(), IpcError>;

    /// 超时等待
    fn wait_timeout(&self, timeout_ms: u64) -> Result<bool, IpcError>;

    /// 尝试等待 (非阻塞)
    fn try_wait(&self) -> Result<bool, IpcError>;

    /// 释放 (V 操作)
    fn signal(&self) -> Result<(), IpcError>;

    /// 获取等待的 Agent 数量
    fn waiters(&self) -> usize;
}