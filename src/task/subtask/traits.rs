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

//! # SubTask 管理接口
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
use crate::task::error::TaskError;

/// SubTask 管理接口
pub trait SubTaskManager: Send + Sync {
    /// 在 Task 中创建 SubTask
    fn create_subtask(
        &mut self,
        task_id: &TaskId,
        name: alloc::string::String,
        dependencies: alloc::vec::Vec<SubTaskId>,
    ) -> Result<SubTaskId, TaskError>;

    /// 获取 SubTask 信息
    fn get_subtask(&self, subtask_id: &SubTaskId) -> Result<SubTask, TaskError>;

    /// 更新 SubTask 状态
    fn update_subtask_status(
        &mut self,
        subtask_id: &SubTaskId,
        status: SubTaskStatus,
    ) -> Result<(), TaskError>;

    /// 标记 SubTask 为完成，存储结果
    fn complete_subtask(
        &mut self,
        subtask_id: &SubTaskId,
        result: SubTaskResult,
    ) -> Result<(), TaskError>;

    /// 获取 Task 下所有 SubTask
    fn list_subtasks(&self, task_id: &TaskId) -> Result<alloc::vec::Vec<SubTask>, TaskError>;

    /// 检查 SubTask 依赖是否满足
    fn check_dependencies(&self, subtask_id: &SubTaskId) -> Result<bool, TaskError>;

    /// 获取所有就绪的 SubTask (依赖已满足)
    fn get_ready_subtasks(&self, task_id: &TaskId) -> Result<alloc::vec::Vec<SubTask>, TaskError>;

    /// 验证 Task 内 SubTask 依赖图是否有环
    fn validate_dependency_graph(&self, task_id: &TaskId) -> Result<(), TaskError>;

    /// 获取 SubTask 的拓扑排序 (执行顺序)
    fn get_topological_order(&self, task_id: &TaskId) -> Result<alloc::vec::Vec<SubTaskId>, TaskError>;

    /// 获取 SubTask 执行指标
    fn get_subtask_metrics(&self, subtask_id: &SubTaskId) -> Result<SubTaskMetrics, TaskError>;
}