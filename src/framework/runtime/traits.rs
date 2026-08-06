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

//! # 框架运行时管理接口
//!
//! ## Version
//! 0.1.0

use super::types::*;
use crate::framework::framework::FrameworkId;
use crate::framework::error::FrameworkError;

/// 框架运行时管理接口
pub trait FrameworkRuntimeManager: Send + Sync {
    /// 启动框架运行时
    fn start(
        &mut self,
        framework_id: &FrameworkId,
        task_id: &TaskId,
        args: alloc::vec::Vec<alloc::string::String>,
    ) -> Result<RuntimeHandle, FrameworkError>;

    /// 停止运行时
    fn stop(&mut self, handle: &RuntimeHandle) -> Result<(), FrameworkError>;

    /// 暂停运行时
    fn pause(&mut self, handle: &RuntimeHandle) -> Result<(), FrameworkError>;

    /// 恢复运行时
    fn resume(&mut self, handle: &RuntimeHandle) -> Result<(), FrameworkError>;

    /// 获取运行时状态
    fn status(&self, handle: &RuntimeHandle) -> Result<RuntimeStatus, FrameworkError>;

    /// 列出所有运行时
    fn list(&self) -> alloc::vec::Vec<FrameworkRuntime>;

    /// 获取 Task 关联的运行时
    fn get_by_task(&self, task_id: &TaskId) -> Option<FrameworkRuntime>;
}