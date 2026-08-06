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

//! # Agent 生命周期类型定义
//!
//! ## Version
//! 0.1.0

use serde::{Deserialize, Serialize};

use crate::runtime::agent::AgentId;

/// 生命周期状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LifecycleState {
    Created,
    Starting,
    Running,
    Pausing,
    Paused,
    Resuming,
    Stopping,
    Terminated,
    Error,
}

/// 生命周期事件
#[derive(Debug, Clone)]
pub struct LifecycleEvent {
    pub agent_id: AgentId,
    pub from: LifecycleState,
    pub to: LifecycleState,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub reason: LifecycleEventReason,
}

/// 生命周期事件原因
#[derive(Debug, Clone)]
pub enum LifecycleEventReason {
    UserRequest,
    Scheduler,
    TaskCompletion,
    Error(alloc::string::String),
    Timeout,
    ResourceExhausted,
}