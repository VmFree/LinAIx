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

//! # Skill 适配器接口定义
//!
//! ## Version
//! 0.1.0

use super::types::*;
use crate::skill::error::SkillError;
use crate::skill::metadata::SkillMetadata;
use crate::skill::invocation::SkillInvocation;

/// Skill 适配器接口
pub trait SkillAdapter: Send + Sync {
    /// 获取适配器支持的框架名称
    fn framework_name(&self) -> &str;

    /// 获取适配方向
    fn direction(&self) -> AdapterDirection;

    /// 反向适配: 将 LinAIx Skill 适配为框架原生工具
    ///
    /// 返回框架原生工具对象 (Box<dyn Any> 由调用方转换)
    fn adapt_to_framework(&self, metadata: &SkillMetadata) -> Result<Box<dyn core::any::Any>, SkillError>;

    /// 正向适配: 拦截框架工具调用，转换为 LinAIx Skill 调用
    ///
    /// 返回 LinAIx SkillInvocation
    fn intercept_framework_call(
        &self,
        framework_tool: &dyn core::any::Any,
        params: serde_json::Value,
    ) -> Result<SkillInvocation, SkillError>;

    /// 适配 LinAIx 结果到框架原生格式
    fn adapt_result_to_framework(
        &self,
        result: serde_json::Value,
    ) -> Result<AdapterResult, SkillError>;

    /// 适配框架错误到 LinAIx 错误
    fn adapt_error_to_linAIx(&self, framework_error: &str) -> SkillError;
}