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

//! # Agent 沙箱接口
//!
//! ## Version
//! 0.1.0

use super::types::*;
use crate::runtime::error::RuntimeError;
use crate::runtime::agent::AgentId;

/// Agent 沙箱接口
///
/// 职责：创建、配置、执行沙箱环境
pub trait AgentSandbox: Send + Sync {
    /// 创建沙箱
    fn create(&mut self, agent_id: &AgentId, config: SandboxConfig) -> Result<Box<dyn SandboxInstance>, RuntimeError>;

    /// 获取沙箱配置
    fn get_config(&self, agent_id: &AgentId) -> Result<SandboxConfig, RuntimeError>;

    /// 更新沙箱配置
    fn update_config(&mut self, agent_id: &AgentId, config: SandboxConfig) -> Result<(), RuntimeError>;

    /// 检查沙箱是否存在
    fn exists(&self, agent_id: &AgentId) -> bool;

    /// 删除沙箱
    fn remove(&mut self, agent_id: &AgentId) -> Result<(), RuntimeError>;
}

/// 沙箱实例
pub trait SandboxInstance: Send + Sync {
    /// 获取 Agent ID
    fn agent_id(&self) -> &AgentId;

    /// 检查沙箱是否健康
    fn is_healthy(&self) -> bool;

    /// 获取沙箱内的进程 ID
    fn pid(&self) -> Option<u32>;

    /// 获取资源使用情况
    fn resource_usage(&self) -> SandboxResourceUsage;

    /// 执行命令 (在沙箱内)
    fn exec(&self, command: alloc::string::String, args: alloc::vec::Vec<alloc::string::String>) -> Result<(), RuntimeError>;
}

/// 沙箱资源使用
#[derive(Debug, Clone)]
pub struct SandboxResourceUsage {
    pub memory_used_bytes: u64,
    pub cpu_time_ms: u64,
    pub disk_used_bytes: u64,
    pub network_rx_bytes: u64,
    pub network_tx_bytes: u64,
}