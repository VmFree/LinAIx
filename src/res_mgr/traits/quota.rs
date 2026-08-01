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

//! # 配额管理接口
//!
//! ## Version
//! 0.1.0
//!
//! ## Author
//! VmFree <vmfree@example.com>
//!
//! ## Date
//! 2026-08-01

use crate::resource::types::*;
use crate::resource::error::ResourceError;

/// 配额管理接口
///
/// 职责：设置/获取/检查/报告配额状态
///
/// # 重要设计原则
/// 此模块只负责**检测和报告**配额状态，不负责决策。
/// 决策（拒绝/排队/降级/抢占）由 L3 Scheduler 负责。
pub trait QuotaResource: Send + Sync {
    /// 设置 Agent 配额
    fn set_quota(&mut self, agent_id: &AgentId, quota: ResourceQuota) -> Result<(), ResourceError>;

    /// 获取 Agent 配额
    fn get_quota(&self, agent_id: &AgentId) -> Option<ResourceQuota>;

    /// 删除 Agent 配额
    fn remove_quota(&mut self, agent_id: &AgentId) -> Result<(), ResourceError>;

    /// 检查配额是否充足
    ///
    /// # 返回
    /// - `Ok(())`: 配额充足
    /// - `Err(ResourceError::QuotaExceeded)`: 配额超限
    fn check_quota(&self, agent_id: &AgentId, request: &ResourceRequest) -> Result<(), ResourceError>;

    /// 获取配额超限详情 (供 L3 决策使用)
    fn get_exceeded_detail(&self, agent_id: &AgentId) -> Option<ExceededDetail>;

    /// 更新配额使用量
    ///
    /// # 调用时机
    /// 推理完成后，由 L3 调用更新
    fn update_usage(&self, agent_id: &AgentId, usage: ResourceUsage) -> Result<(), ResourceError>;

    /// 重置配额使用量 (用于周期重置)
    fn reset_usage(&self, agent_id: &AgentId) -> Result<(), ResourceError>;

    /// 获取所有 Agent 的配额状态
    fn list_quota_status(&self) -> alloc::vec::Vec<QuotaStatus>;

    /// 获取配额使用率 (0.0 - 1.0)
    fn get_usage_ratio(&self, agent_id: &AgentId, resource_type: ResourceType) -> Option<f32>;
}