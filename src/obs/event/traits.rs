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

//! # Event 接口
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
use crate::obs::error::ObsError;

/// 事件总线接口
pub trait EventBus: Send + Sync {
    /// 发布事件
    fn publish(&self, event: Event) -> Result<(), ObsError>;

    /// 批量发布事件
    fn publish_batch(&self, events: &[Event]) -> Result<(), ObsError>;

    /// 订阅事件
    fn subscribe(
        &mut self,
        filter: EventFilter,
        subscriber: EventSubscriber,
    ) -> Result<SubscriptionId, ObsError>;

    /// 取消订阅
    fn unsubscribe(&mut self, subscription_id: &SubscriptionId) -> Result<(), ObsError>;

    /// 查询事件
    fn query(&self, filter: &EventFilter) -> Result<alloc::vec::Vec<Event>, ObsError>;

    /// 获取最近 N 个事件
    fn recent(&self, count: usize) -> alloc::vec::Vec<Event>;

    /// 获取事件统计
    fn stats(&self) -> EventStats;
}

/// 事件统计
#[derive(Debug, Clone, Default)]
pub struct EventStats {
    pub total_published: u64,
    pub total_dropped: u64,
    pub active_subscribers: usize,
    pub by_type: std::collections::HashMap<alloc::string::String, u64>,
}