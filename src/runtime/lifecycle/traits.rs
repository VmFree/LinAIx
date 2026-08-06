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

//! # Agent 生命周期接口
//!
//! ## Version
//! 0.1.0

use super::types::*;
use crate::runtime::error::RuntimeError;
use crate::runtime::agent::AgentId;

/// Agent 生命周期管理接口
///
/// 职责：Agent 的启动、暂停、恢复、终止
pub trait AgentLifecycle: Send + Sync {
    /// 启动 Agent
    fn start(&mut self, agent_id: &AgentId) -> Result<(), RuntimeError>;

    /// 暂停 Agent
    fn pause(&mut self, agent_id: &AgentId) -> Result<(), RuntimeError>;

    /// 恢复 Agent
    fn resume(&mut self, agent_id: &AgentId) -> Result<(), RuntimeError>;

    /// 终止 Agent
    fn terminate(&mut self, agent_id: &AgentId) -> Result<(), RuntimeError>;

    /// 获取 Agent 当前生命周期状态
    fn get_state(&self, agent_id: &AgentId) -> Result<LifecycleState, RuntimeError>;

    /// 获取 Agent 的生命周期事件历史
    fn get_events(&self, agent_id: &AgentId) -> Result<alloc::vec::Vec<LifecycleEvent>, RuntimeError>;
}