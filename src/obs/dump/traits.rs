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

//! # Dump 接口
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
use crate::obs::error::ObsError;

/// 转储管理器接口
pub trait DumpManager: Send + Sync {
    /// 创建转储
    fn create_dump(
        &self,
        dump_type: DumpType,
        task_id: Option<&TaskId>,
        subtask_id: Option<&SubTaskId>,
        agent_id: Option<&AgentId>,
        reason: alloc::string::String,
    ) -> Result<DumpMetadata, ObsError>;

    /// 获取转储内容
    fn get_dump(&self, dump_path: &str) -> Result<DumpContent, ObsError>;

    /// 列出所有转储
    fn list_dumps(&self, filter: Option<DumpType>) -> alloc::vec::Vec<DumpMetadata>;

    /// 恢复转储
    fn restore_dump(&self, dump_path: &str) -> Result<(), ObsError>;

    /// 删除转储
    fn delete_dump(&self, dump_path: &str) -> Result<(), ObsError>;

    /// 清理过期转储
    fn clean_old_dumps(&self, max_age_days: u32) -> Result<usize, ObsError>;

    /// 获取转储配置
    fn config(&self) -> DumpConfig;
}