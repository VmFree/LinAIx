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

//! # 适配器类型定义
//!
//! ## Version
//! 0.1.0

use crate::task::{TaskId, SubTaskId};
use crate::framework::framework::FrameworkId;

/// 适配器方向
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdapterDirection {
    /// 正向适配: 框架 → LinAIx
    FrameworkToLinAIx,

    /// 反向适配: LinAIx → 框架
    LinAIxToFramework,

    /// 双向适配
    Both,
}

/// 模型调用请求 (适配器拦截到框架模型调用时的格式)
#[derive(Debug, Clone)]
pub struct ModelCallRequest {
    pub model_name: alloc::string::String,
    pub messages: alloc::vec::Vec<Message>,
    pub parameters: ModelCallParameters,
}

/// 模型调用响应
#[derive(Debug, Clone)]
pub struct ModelCallResponse {
    pub content: alloc::string::String,
    pub tokens_prompt: u32,
    pub tokens_completion: u32,
    pub tokens_total: u32,
}

/// 模型调用参数
#[derive(Debug, Clone)]
pub struct ModelCallParameters {
    pub temperature: Option<f32>,
    pub max_tokens: Option<u32>,
    pub top_p: Option<f32>,
}

/// 框架上下文
#[derive(Debug, Clone)]
pub struct FrameworkContext {
    pub framework_type: alloc::string::String,
    pub subtask_mapping: std::collections::HashMap<alloc::string::String, SubTaskId>,
}

/// 子任务信息 (适配器检测到框架子任务创建时上报)
#[derive(Debug, Clone)]
pub struct SubTaskInfo {
    pub name: alloc::string::String,
    pub description: Option<alloc::string::String>,
    pub dependencies: alloc::vec::Vec<SubTaskId>,
}