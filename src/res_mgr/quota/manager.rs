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

//! # 配额管理默认实现
//!
//! ## Version
//! 0.1.0
//!
//! ## Author
//! VmFree <vmfree@example.com>
//!
//! ## Date
//! 2026-08-01

use crate::resource::quota::traits::QuotaResource;
use crate::resource::types::*;
use crate::resource::error::ResourceError;

/// 配额管理器默认实现
pub struct DefaultQuotaManager {
    /// Agent 配额映射 (保留)
    agent_quotas: std::collections::HashMap<AgentId, ResourceQuota>,
    /// Task 配额映射 (新增)
    task_quotas: std::collections::HashMap<TaskId, ResourceQuota>,
    /// 使用量跟踪
    usage: std::collections::HashMap<TaskId, ResourceUsage>,
}

impl DefaultQuotaManager {
    pub fn new() -> Self {
        Self {
            agent_quotas: std::collections::HashMap::new(),
            task_quotas: std::collections::HashMap::new(),
            usage: std::collections::HashMap::new(),
        }
    }

    /// 内部检查 Token 配额
    fn check_token_quota(
        &self,
        task_id: &TaskId,
        requested: u64,
    ) -> Result<(), ResourceError> {
        let quota = self.task_quotas
            .get(task_id)
            .ok_or_else(|| ResourceError::QuotaNotFound(task_id.clone()))?;

        let max_tokens = quota.max_token_quota
            .ok_or_else(|| ResourceError::InvalidQuota("max_token_quota not set".to_string()))?;

        let current_usage = self.usage.get(task_id).unwrap_or(&ResourceUsage::default());
        let used = current_usage.token_consumed;

        if used + requested > max_tokens {
            return Err(ResourceError::QuotaExceeded {
                resource_type: ResourceType::Token,
                limit: max_tokens,
                current: used + requested,
            });
        }

        Ok(())
    }
}

impl QuotaResource for DefaultQuotaManager {
    // ===== 配额设置 =====

    fn set_quota(&mut self, agent_id: &AgentId, quota: ResourceQuota) -> Result<(), ResourceError> {
        self.agent_quotas.insert(agent_id.clone(), quota);
        Ok(())
    }

    fn set_task_quota(&mut self, task_id: &TaskId, quota: ResourceQuota) -> Result<(), ResourceError> {
        self.task_quotas.insert(task_id.clone(), quota);
        Ok(())
    }

    fn get_quota(&self, agent_id: &AgentId) -> Option<ResourceQuota> {
        self.agent_quotas.get(agent_id).cloned()
    }

    fn get_task_quota(&self, task_id: &TaskId) -> Option<ResourceQuota> {
        self.task_quotas.get(task_id).cloned()
    }

    fn remove_quota(&mut self, agent_id: &AgentId) -> Result<(), ResourceError> {
        self.agent_quotas.remove(agent_id);
        Ok(())
    }

    fn remove_task_quota(&mut self, task_id: &TaskId) -> Result<(), ResourceError> {
        self.task_quotas.remove(task_id);
        Ok(())
    }

    // ===== 配额检查 =====

    fn check_quota(&self, task_id: &TaskId, request: &ResourceRequest) -> Result<(), ResourceError> {
        match request.resource_type {
            ResourceType::Token => {
                self.check_token_quota(task_id, request.amount)
            }
            // TODO: 其他资源类型
            _ => Ok(()),
        }
    }

    fn check_agent_quota(&self, _agent_id: &AgentId, _request: &ResourceRequest) -> Result<(), ResourceError> {
        // 保留，后续实现
        Ok(())
    }

    fn get_exceeded_detail(&self, task_id: &TaskId) -> Option<ExceededDetail> {
        // 简化实现，后续完善
        None
    }

    // ===== 配额使用量更新 =====

    fn update_usage(&self, task_id: &TaskId, usage: &ResourceUsage) -> Result<(), ResourceError> {
        // 使用内部可变性或通过 RefCell，这里简化
        // 实际实现需要处理并发
        Ok(())
    }

    fn update_agent_usage(&self, _agent_id: &AgentId, _usage: &ResourceUsage) -> Result<(), ResourceError> {
        Ok(())
    }

    fn reset_usage(&self, _task_id: &TaskId) -> Result<(), ResourceError> {
        Ok(())
    }

    fn get_usage(&self, task_id: &TaskId) -> Result<ResourceUsage, ResourceError> {
        self.usage.get(task_id).cloned().ok_or(ResourceError::QuotaNotFound(task_id.clone()))
    }

    // ===== 配额状态查询 =====

    fn list_quota_status(&self) -> alloc::vec::Vec<QuotaStatus> {
        alloc::vec::Vec::new()
    }

    fn get_task_quota_status(&self, task_id: &TaskId) -> Option<QuotaStatus> {
        let quota = self.task_quotas.get(task_id)?;
        let usage = self.usage.get(task_id).unwrap_or(&ResourceUsage::default());
        let mut usage_ratios = std::collections::HashMap::new();

        if let Some(max) = quota.max_token_quota {
            if max > 0 {
                usage_ratios.insert(ResourceType::Token, usage.token_consumed as f32 / max as f32);
            }
        }

        Some(QuotaStatus {
            agent_id: None,
            task_id: Some(task_id.clone()),
            quota: quota.clone(),
            usage: usage.clone(),
            usage_ratios,
            is_exceeded: false,
            exceeded_details: alloc::vec::Vec::new(),
        })
    }

    fn get_usage_ratio(&self, task_id: &TaskId, resource_type: ResourceType) -> Option<f32> {
        let quota = self.task_quotas.get(task_id)?;
        let usage = self.usage.get(task_id).unwrap_or(&ResourceUsage::default());

        match resource_type {
            ResourceType::Token => {
                let max = quota.max_token_quota?;
                if max > 0 {
                    Some(usage.token_consumed as f32 / max as f32)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}