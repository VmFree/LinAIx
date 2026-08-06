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

//! # 权限相关类型 (细粒度)
//!
//! ## 版本
//! 0.1.0
//!
//! ## 作者
//! VmFree <vmfree@example.com>
//!
//! ## 日期
//! 2026-08-01

use serde::{Deserialize, Serialize};

use crate::task::{SubTaskId};

/// 权限定义
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Permission {
    /// 资源类型 (细粒度)
    pub resource_type: ResourceType,

    /// 资源 ID (支持通配符匹配)
    pub resource_id: Option<alloc::string::String>,

    /// 操作 (细粒度)
    pub action: Action,

    /// 作用域
    pub scope: Option<alloc::string::String>,
}

/// 资源类型 (细粒度)
///
/// 支持文件、目录等细粒度资源控制
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ResourceType {
    // ===== 推理相关 =====
    Model,
    Skill,
    KvCache,
    Device,

    // ===== 文件系统 (细粒度) =====
    /// 文件 (支持单个文件的权限控制)
    File,
    /// 目录 (支持目录及子目录的权限控制)
    Directory,
    /// 文件系统 (粗粒度，保留兼容)
    FileSystem,

    // ===== 系统 =====
    Network,
    AuditLog,
    SystemConfig,

    // ===== 自定义 =====
    Custom(alloc::string::String),
}

/// 操作类型 (细粒度)
///
/// 支持细粒度的文件操作控制：读、写、执行、创建、删除、修改
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Action {
    /// 读取文件/资源内容
    Read,

    /// 写入文件/资源内容
    Write,

    /// 执行文件/调用 Skill
    Execute,

    /// 创建新文件/资源
    Create,

    /// 删除文件/资源
    Delete,

    /// 修改文件/资源属性 (权限、所有者等)
    Modify,

    /// 管理 (包含所有管理操作)
    Manage,

    /// 所有操作
    All,

    /// 自定义操作
    Custom(alloc::string::String),
}

/// 权限检查结果
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PermissionResult {
    Allowed,
    Denied(DenyReason),
}

/// 拒绝原因
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DenyReason {
    /// 无此权限
    NoPermission,

    /// 资源不存在
    ResourceNotFound,

    /// 资源归属不匹配 (不是自己的文件)
    ResourceNotOwned,

    /// 租户不匹配
    TenantMismatch,

    /// 策略拒绝
    PolicyDenied,

    /// 其他原因
    Other(alloc::string::String),
}

/// 策略定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Policy {
    pub id: alloc::string::String,
    pub name: alloc::string::String,
    pub description: Option<alloc::string::String>,
    pub conditions: PolicyCondition,
    pub effect: PolicyEffect,
    pub priority: u32,
}

/// 策略条件 (增强版)
///
/// 支持：角色匹配、资源类型匹配、资源ID通配符、操作匹配、
/// 租户匹配、资源所有者匹配、自定义表达式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyCondition {
    /// 角色匹配
    pub roles: Option<alloc::vec::Vec<alloc::string::String>>,

    /// 资源类型匹配
    pub resource_types: Option<alloc::vec::Vec<ResourceType>>,

    /// 资源 ID 匹配 (支持通配符)
    ///
    /// # 示例
    /// - `/data/*`      : 匹配 /data/ 下所有文件
    /// - `model-*`      : 匹配所有 model- 开头的资源
    /// - `*.bin`        : 匹配所有 .bin 文件
    pub resource_id_pattern: Option<alloc::string::String>,

    /// 操作匹配
    pub actions: Option<alloc::vec::Vec<Action>>,

    /// 租户匹配
    pub tenants: Option<alloc::vec::Vec<alloc::string::String>>,

    /// 资源所有者匹配
    ///
    /// # 示例
    /// - 文件所有者: `["agent-123", "user-456"]`
    /// - 通配符: `["*"]` 表示任何所有者
    pub owners: Option<alloc::vec::Vec<SubTaskId>>,

    /// 自定义条件 (JSON 表达式)
    pub custom: Option<alloc::string::String>,
}

/// 策略效果
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyEffect {
    Allow,
    Deny,
}

/// 数据分类级别
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum DataClassification {
    Public = 0,
    Internal = 1,
    Confidential = 2,
    Restricted = 3,
    TopSecret = 4,
}

/// 合规标签
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComplianceTag {
    GDPR,
    HIPAA,
    SOC2,
    PCI,
    ISO27001,
    Custom(alloc::string::String),
}