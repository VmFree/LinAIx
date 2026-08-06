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

//! # 资源统计与观测接口
//!
//! ## Version
//! 0.1.0
//!
//! ## Author
//! VmFree <vmfree@example.com>
//!
//! ## Date
//! 2026-08-01

use super::types::*;
use crate::res_mgr::error::ResourceError;

use crate::task::{SubTaskId};

/// 回调 ID
pub type CallbackId = alloc::string::String;

/// 资源统计接口
///
/// 职责：使用量查询/压力指标/事件订阅
pub trait StatsResource: Send + Sync {
    /// 获取 Agent 资源使用情况
    fn get_agent_usage(&self, subtask_id: &SubTaskId) -> AgentResourceUsage;

    /// 获取全局资源使用情况
    fn get_global_usage(&self) -> GlobalResourceUsage;

    /// 获取特定资源类型的压力指标
    fn get_pressure(&self, resource_type: ResourceType) -> ResourcePressure;

    /// 获取所有资源类型的压力指标
    fn get_all_pressures(&self) -> alloc::vec::Vec<ResourcePressure>;

    /// 订阅资源事件 (主动通知)
    ///
    /// # 事件类型
    /// - 配额接近上限 (80%, 90%, 95%)
    /// - 资源压力变化 (Low → Medium → High → Critical)
    /// - KV Cache 换入/换出事件
    /// - 模型加载/卸载事件
    fn subscribe_events(&self, callback: Box<dyn ResourceEventCallback>) -> CallbackId;

    /// 取消订阅
    fn unsubscribe_events(&self, callback_id: &CallbackId) -> Result<(), ResourceError>;

    /// 获取历史统计数据 (时间窗口)
    fn get_history(
        &self,
        resource_type: ResourceType,
        start: chrono::DateTime<chrono::Utc>,
        end: chrono::DateTime<chrono::Utc>,
        interval: Duration,
    ) -> Result<alloc::vec::Vec<HistoricalDataPoint>, ResourceError>;

    /// 获取最近 N 个数据点
    fn get_recent(&self, resource_type: ResourceType, count: usize) -> alloc::vec::Vec<HistoricalDataPoint>;
}