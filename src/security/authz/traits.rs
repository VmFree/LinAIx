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

//! # 授权接口
//!
//! ## Version
//! 0.1.0
//!
//! ## Author
//! VmFree <vmfree@example.com>
//!
//! ## Date
//! 2026-08-01

use crate::security::types::*;
use crate::security::error::SecurityError;

/// 授权接口
///
/// 职责：权限检查、策略管理
///
/// # 设计原则
/// - 权限检查基于认证上下文 (`AuthContext`)
/// - 策略采用 "默认拒绝" 原则
pub trait Authorization: Send + Sync {
    /// 检查单个权限
    fn check_permission(
        &self,
        context: &AuthContext,
        permission: &Permission,
    ) -> PermissionResult;

    /// 批量检查权限
    fn check_permissions(
        &self,
        context: &AuthContext,
        permissions: &[Permission],
    ) -> alloc::vec::Vec<PermissionResult>;

    /// 获取 Agent 的所有权限
    fn get_permissions(&self, context: &AuthContext) -> alloc::vec::Vec<Permission>;

    // ===== 策略管理 =====

    /// 添加策略
    fn add_policy(&mut self, policy: Policy) -> Result<(), SecurityError>;

    /// 删除策略
    fn remove_policy(&mut self, policy_id: &str) -> Result<(), SecurityError>;

    /// 获取所有策略
    fn list_policies(&self) -> alloc::vec::Vec<Policy>;

    /// 评估所有策略
    fn evaluate_policies(
        &self,
        context: &AuthContext,
        resource: &str,
        action: &Action,
    ) -> PermissionResult;

    // ===== 便捷方法 =====

    /// 检查是否有权删除文件
    fn can_delete_file(&self, context: &AuthContext, file_path: &str) -> bool {
        let perm = Permission {
            resource_type: ResourceType::File,
            resource_id: Some(file_path.to_string()),
            action: Action::Delete,
            scope: Some(context.tenant_id.clone()),
        };
        matches!(self.check_permission(context, &perm), PermissionResult::Allowed)
    }

    /// 检查是否有权修改文件
    fn can_modify_file(&self, context: &AuthContext, file_path: &str) -> bool {
        let perm = Permission {
            resource_type: ResourceType::File,
            resource_id: Some(file_path.to_string()),
            action: Action::Modify,
            scope: Some(context.tenant_id.clone()),
        };
        matches!(self.check_permission(context, &perm), PermissionResult::Allowed)
    }

    /// 检查是否有权读取文件
    fn can_read_file(&self, context: &AuthContext, file_path: &str) -> bool {
        let perm = Permission {
            resource_type: ResourceType::File,
            resource_id: Some(file_path.to_string()),
            action: Action::Read,
            scope: Some(context.tenant_id.clone()),
        };
        matches!(self.check_permission(context, &perm), PermissionResult::Allowed)
    }
}