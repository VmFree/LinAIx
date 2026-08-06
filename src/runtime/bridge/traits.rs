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

//! # 系统调用桥接口
//!
//! ## Version
//! 0.1.0

use super::types::*;
use crate::runtime::error::RuntimeError;
use crate::runtime::agent::AgentId;

/// 系统调用桥接口
///
/// 职责：Agent 调用 LinAIx 内核能力的桥梁
pub trait SystemCallBridge: Send + Sync {
    /// 执行系统调用
    fn syscall(&self, call: &SystemCall) -> Result<SystemCallResult, RuntimeError>;

    /// 异步执行系统调用
    async fn syscall_async(&self, call: &SystemCall) -> Result<SystemCallResult, RuntimeError>;

    /// 检查系统调用是否被允许
    fn is_allowed(&self, agent_id: &AgentId, call_type: &SystemCallType) -> bool;

    /// 获取 Agent 可用的系统调用列表
    fn allowed_calls(&self, agent_id: &AgentId) -> alloc::vec::Vec<SystemCallType>;
}