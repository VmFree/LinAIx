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

//! # 审计相关类型
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

use crate::task::{TaskId, SubTaskId};

/// 审计事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEvent {
    pub id: alloc::string::String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub event_type: AuditEventType,
    pub tenant_id: super::auth::TenantId,
    pub resource: AuditResource,
    pub result: AuditResult,
    pub request_id: alloc::string::String,
    pub source: AuditSource,
    pub details: std::collections::HashMap<alloc::string::String, alloc::string::String>,
    pub signature: Option<alloc::vec::Vec<u8>>,
    pub data_classification: super::permission::DataClassification,
    pub task_id: Option<TaskId>,
    pub actor: Option<SubTaskId>,
}

/// 审计事件类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuditEventType {
    // ===== 认证相关 =====
    AuthLogin,
    AuthLogout,
    AuthTokenRefresh,
    AuthTokenRevoke,
    AuthFailed,

    // ===== 授权相关 =====
    PermissionCheck,
    PermissionDenied,  // 权限被拒绝 (重要)

    // ===== 推理相关 =====
    InferenceStart,
    InferenceComplete,
    InferenceFailed,
    InferenceStreamStart,
    InferenceStreamComplete,

    // ===== Skill 相关 =====
    SkillInvoke,
    SkillInvokeComplete,
    SkillInvokeFailed,

    // ===== 模型相关 =====
    ModelLoad,
    ModelUnload,
    ModelLoadFailed,

    // ===== 资源相关 =====
    ResourceAllocate,
    ResourceFree,
    ResourceQuotaExceeded,

    // ===== KV Cache 相关 =====
    KvCacheAllocate,
    KvCacheFree,
    KvCacheSwapOut,
    KvCacheSwapIn,
    KvCachePersist,
    KvCacheRestore,

    // ===== 文件系统相关 (新增) =====
    FileCreate,
    FileDelete,
    FileWrite,
    FileRead,
    FileModify,
    FileAccessDenied,  // 文件访问被拒绝

    // ===== 系统管理 =====
    ConfigChange,
    PolicyChange,
    AgentCreate,
    AgentDelete,
    AgentSuspend,
    AgentResume,

    // ===== 安全事件 =====
    InjectionDetected,
    SecurityViolation,

    // ===== 自定义 =====
    Custom(alloc::string::String),
}

/// 审计资源
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditResource {
    pub resource_type: super::permission::ResourceType,
    pub resource_id: Option<alloc::string::String>,
    pub resource_name: Option<alloc::string::String>,
}

/// 审计结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuditResult {
    Success,
    Failure,
    Partial,
    Timeout,
    Canceled,
}

/// 审计来源
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditSource {
    pub ip: Option<alloc::string::String>,
    pub node_id: Option<alloc::string::String>,
    pub user_agent: Option<alloc::string::String>,
    pub region: Option<alloc::string::String>,
}

/// 审计过滤器
#[derive(Debug, Clone, Default)]
pub struct AuditFilter {
    pub event_type: Option<AuditEventType>,
    pub task_id: Option<TaskId>,
    pub actor: Option<SubTaskId>,
    pub tenant_id: Option<super::auth::TenantId>,
    pub resource_type: Option<super::permission::ResourceType>,
    pub resource_id: Option<alloc::string::String>,
    pub start_time: Option<chrono::DateTime<chrono::Utc>>,
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,
    pub result: Option<AuditResult>,
    pub request_id: Option<alloc::string::String>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

/// 审计导出格式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuditExportFormat {
    Json,
    Csv,
    Parquet,
    Text,
}

/// 审计统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditStats {
    pub total_events: u64,
    pub success_count: u64,
    pub failure_count: u64,
    pub by_event_type: std::collections::HashMap<alloc::string::String, u64>,
    pub by_actor: std::collections::HashMap<alloc::string::String, u64>,
    pub by_resource_type: std::collections::HashMap<alloc::string::String, u64>,
    pub time_range_start: chrono::DateTime<chrono::Utc>,
    pub time_range_end: chrono::DateTime<chrono::Utc>,
}