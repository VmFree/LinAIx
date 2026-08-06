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

//! # IPC 共享内存接口
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

/// 共享内存管理接口
pub trait IpcSharedMemory: Send + Sync {
    /// 分配共享内存
    fn allocate(
        &mut self,
        task_id: &TaskId,
        size: usize,
        flags: ShmFlags,
    ) -> Result<SharedMemoryId, IpcError>;

    /// 释放共享内存
    fn free(&mut self, shm_id: &SharedMemoryId) -> Result<(), IpcError>;

    /// 获取共享内存区域信息
    fn get_region(&self, shm_id: &SharedMemoryId) -> Result<SharedMemoryRegion, IpcError>;

    /// 映射到内存
    fn map(&self, shm_id: &SharedMemoryId) -> Result<*mut u8, IpcError>;

    /// 取消映射
    fn unmap(&self, ptr: *mut u8) -> Result<(), IpcError>;

    /// 获取共享内存大小
    fn size(&self, shm_id: &SharedMemoryId) -> Result<usize, IpcError>;

    /// 获取统计信息
    fn stats(&self) -> ShmStats;

    /// 列出 Task 下所有共享内存
    fn list_by_task(&self, task_id: &TaskId) -> alloc::vec::Vec<SharedMemoryRegion>;

    /// 检查共享内存是否存在
    fn exists(&self, shm_id: &SharedMemoryId) -> bool;
}