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

//! # 账户管理接口
//!
//! ## Version
//! 0.1.0

use super::types::*;
use crate::security::error::SecurityError;

/// 账户提供者接口
///
/// 职责：作为账户数据的抽象层，屏蔽底层账户源（OS / 独立DB）的差异。
/// 该接口只负责"查询"和"认证"，不包含权限决策逻辑。
pub trait AccountProvider: Send + Sync {
    /// 根据用户名获取账户信息
    fn get_account(&self, identifier: &str) -> Result<Option<Account>, SecurityError>;

    /// 根据UID获取账户信息
    fn get_account_by_uid(&self, uid: u32) -> Result<Option<Account>, SecurityError>;

    /// 列出所有账户
    fn list_accounts(&self, filter: Option<AccountFilter>) -> Result<alloc::vec::Vec<Account>, SecurityError>;

    /// 获取账户所属的组
    fn get_groups_for_account(&self, account_id: &UserId) -> Result<alloc::vec::Vec<Group>, SecurityError>;

    /// 验证账户凭证
    fn verify_credentials(
        &self,
        account_id: &UserId,
        credential: &CredentialData,
    ) -> Result<bool, SecurityError>;

    // ===== 可选：生命周期管理 =====

    fn create_account(&mut self, account: Account) -> Result<(), SecurityError> {
        Err(SecurityError::Unsupported("create_account".to_string()))
    }

    fn update_account(&mut self, account: Account) -> Result<(), SecurityError> {
        Err(SecurityError::Unsupported("update_account".to_string()))
    }

    fn delete_account(&mut self, account_id: &UserId) -> Result<(), SecurityError> {
        Err(SecurityError::Unsupported("delete_account".to_string()))
    }
}