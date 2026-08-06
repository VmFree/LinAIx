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

//! # Event 类型定义
//!
//! ## Version
//! 0.1.0
//!
//! ## Author
//! VmFree <vmfree@example.com>
//!
//! ## Date
//! 2026-08-01

use serde::{Deserialize, Serialize};

use crate::obs::tracing::TraceId;

/// 事件类型
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventType {
    // ===== 系统事件 =====
    SystemStart,
    SystemShutdown,
    SystemError,

    // ===== Agent 事件 =====
    AgentStart,
    AgentStop,
    AgentPause,
    AgentResume,
    AgentError,
    AgentComplete,

    // ===== Task 事件 =====
    TaskCreate,
    TaskComplete,
    TaskTimeout,
    TaskCancel,
    TaskError,

    // ===== SubTask 事件 =====
    SubTaskCreate,
    SubTaskReady,
    SubTaskSchedule,
    SubTaskRun,
    SubTaskComplete,
    SubTaskError,
    SubTaskTimeout,

    // ===== Skill 事件 =====
    SkillInvoke,
    SkillComplete,
    SkillError,
    SkillTimeout,

    // ===== 模型事件 =====
    ModelInfer,
    ModelComplete,
    ModelError,

    // ===== 资源事件 =====
    ResourceAlloc,
    ResourceFree,
    QuotaExceeded,
    QuotaWarning,
    KvCacheSwapOut,
    KvCacheSwapIn,

    // ===== 安全事件 =====
    AuthFailed,
    AuthSuccess,
    PermissionDenied,
    InjectionDetected,

    // ===== IPC 事件 =====
    IpcMessageSend,
    IpcMessageReceive,
    IpcChannelCreate,
    IpcChannelClose,
}

/// 事件严重程度
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum EventSeverity {
    Debug = 0,
    Info = 1,
    Warning = 2,
    Error = 3,
    Critical = 4,
}

/// 事件
#[derive(Debug, Clone)]
pub struct Event {
    pub id: alloc::string::String,
    pub event_type: EventType,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub source: alloc::string::String,   // 来源模块
    pub severity: EventSeverity,
    pub payload: serde_json::Value,
    pub trace_id: Option<TraceId>,
}

/// 事件订阅者
pub type EventSubscriber = Box<dyn Fn(&Event) + Send + Sync>;

/// 事件订阅 ID
pub type SubscriptionId = alloc::string::String;

/// 事件过滤器
#[derive(Debug, Clone, Default)]
pub struct EventFilter {
    pub event_type: Option<EventType>,
    pub source: Option<alloc::string::String>,
    pub min_severity: Option<EventSeverity>,
    pub trace_id: Option<TraceId>,
    pub start_time: Option<chrono::DateTime<chrono::Utc>>,
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,
}