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

//! # 适配器接口
//!
//! ## Version
//! 0.1.0

use super::types::*;
use crate::task::{TaskId, SubTaskId};
use crate::framework::framework::FrameworkId;
use crate::framework::error::FrameworkError;
use crate::skill::types::SkillMetadata;
use crate::skill::types::SkillInvocation;

/// 框架适配器接口
pub trait FrameworkAdapter: Send + Sync {
    // ===== 框架标识 =====

    /// 框架名称
    fn framework_name(&self) -> &str;

    /// 适配方向
    fn direction(&self) -> AdapterDirection;

    // ===== Skill 适配 =====

    /// 反向适配: 将 LinAIx Skill 适配为框架原生工具
    fn adapt_to_framework(&self, metadata: &SkillMetadata) -> Result<Box<dyn core::any::Any>, FrameworkError>;

    /// 正向适配: 拦截框架 Skill 调用，转换为 LinAIx Skill 调用
    fn adapt_to_linAIx_skill(&self, context: &FrameworkContext, params: serde_json::Value) -> Result<SkillInvocation, FrameworkError>;

    // ===== 模型拦截 =====

    /// 拦截框架的模型调用
    ///
    /// 附加 TaskId 和 SubTaskId 后调用 L1 HAL
    fn intercept_model_call(
        &self,
        task_id: &TaskId,
        subtask_id: &SubTaskId,
        framework_id: &FrameworkId,
        model_request: &ModelCallRequest,
    ) -> Result<ModelCallResponse, FrameworkError>;

    // ===== 子任务检测 =====

    /// 检测框架内部子任务创建
    ///
    /// 通过 LinAIx TaskManager 创建对应的 SubTask
    fn detect_and_report_subtask(
        &self,
        framework_context: &FrameworkContext,
        task_id: &TaskId,
        framework_id: &FrameworkId,
    ) -> Result<Option<SubTaskInfo>, FrameworkError>;

    // ===== 配额错误处理 =====

    /// 处理 Token 配额超限错误
    fn handle_quota_exceeded(
        &self,
        task_id: &TaskId,
        subtask_id: &SubTaskId,
        remaining: u32,
        requested: u32,
    ) -> Result<(), FrameworkError>;
}