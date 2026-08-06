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

//! # KV Cache 管理接口
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
use crate::res_mgr::error::ResourceError;

/// KV Cache 管理接口
///
/// 职责：分配/释放/换入/换出/持久化/恢复
pub trait KvCacheResource: Send + Sync {
    /// 分配 KV Cache
    fn allocate(
        &self,
        session_id: &SessionId,
        model_id: &ModelId,
        size_bytes: usize,
    ) -> Result<KvCacheHandle, ResourceError>;

    /// 释放 KV Cache
    fn free(&self, handle: &KvCacheHandle) -> Result<(), ResourceError>;

    /// 换出 KV Cache (显存 → 内存/磁盘)
    ///
    /// # 用途
    /// 显存不足时，将不活跃的 KV Cache 换出
    fn swap_out(&self, handle: &KvCacheHandle) -> Result<StorageLocation, ResourceError>;

    /// 换入 KV Cache (内存/磁盘 → 显存)
    ///
    /// # 用途
    /// Agent 恢复时，将 KV Cache 换入显存
    fn swap_in(&self, handle: &KvCacheHandle) -> Result<StorageLocation, ResourceError>;

    /// 持久化到磁盘
    ///
    /// # 用途
    /// Agent 长时间暂停或关机时，持久化 KV Cache
    fn persist(&self, handle: &KvCacheHandle, path: &Path) -> Result<(), ResourceError>;

    /// 从磁盘恢复
    ///
    /// # 用途
    /// Agent 重启时，从磁盘加载 KV Cache
    fn restore(&self, session_id: &SessionId, path: &Path) -> Result<KvCacheHandle, ResourceError>;

    /// 获取 KV Cache 状态
    fn get_status(&self, handle: &KvCacheHandle) -> KvCacheStatus;

    /// 获取 KV Cache 统计信息
    fn get_stats(&self) -> KvCacheStats;

    /// 获取当前所有 KV Cache 句柄
    fn list_handles(&self) -> alloc::vec::Vec<KvCacheHandle>;

    /// 检查 KV Cache 是否被 pin (固定不可换出)
    fn is_pinned(&self, handle: &KvCacheHandle) -> bool;

    /// 设置 KV Cache 的 pin 状态
    fn set_pinned(&self, handle: &KvCacheHandle, pinned: bool) -> Result<(), ResourceError>;
}