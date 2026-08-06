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

//! # Agent 接口
//!
//! ## Version
//! 0.1.0

use super::types::*;
use crate::runtime::error::RuntimeError;

/// Agent 接口
///
/// 职责：Agent 的运行、状态查询、指标获取
pub trait Agent: Send + Sync {
    /// 获取 Agent ID
    fn id(&self) -> &AgentId;

    /// 获取 Agent 状态
    fn status(&self) -> AgentStatus;

    /// 获取生命周期状态
    fn lifecycle_state(&self) -> LifecycleState;

    /// 获取运行时指标
    fn metrics(&self) -> &AgentMetrics;

    /// 更新指标
    fn update_metrics(&mut self, metrics: AgentMetrics) -> Result<(), RuntimeError>;

    /// 检查 Agent 是否健康
    fn is_healthy(&self) -> bool;

    /// 检查 Agent 是否正在运行
    fn is_running(&self) -> bool;

    /// 获取 Agent 的 PID (如果存在)
    fn pid(&self) -> Option<u32>;
}