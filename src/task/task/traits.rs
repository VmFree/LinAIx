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

//! # Task 管理接口
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

/// Task 管理接口
pub trait TaskManager: Send + Sync {
    /// 创建 Task
    fn create_task(
        &mut self,
        name: alloc::string::String,
        quota: TaskQuota,
    ) -> Result<TaskId, TaskError>;

    /// 获取 Task 信息
    fn get_task(&self, task_id: &TaskId) -> Result<Task, TaskError>;

    /// 更新 Task 状态
    fn update_task_status(
        &mut self,
        task_id: &TaskId,
        status: TaskStatus,
    ) -> Result<(), TaskError>;

    /// 终止 Task (强制回收所有资源)
    fn terminate_task(&mut self, task_id: &TaskId) -> Result<(), TaskError>;

    /// 列出所有 Task
    fn list_tasks(&self) -> alloc::vec::Vec<Task>;

    /// 检查 Task 配额是否充足
    fn check_task_quota(
        &self,
        task_id: &TaskId,
        request: &TaskResourceRequest,
    ) -> Result<(), TaskError>;

    /// 更新 Task 资源使用量
    fn update_task_usage(
        &mut self,
        task_id: &TaskId,
        usage: TaskUsage,
    ) -> Result<(), TaskError>;

    /// 获取 Task 资源使用量
    fn get_task_usage(&self, task_id: &TaskId) -> Result<TaskUsage, TaskError>;
}