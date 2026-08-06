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

//! # 账户类型定义
//!
//! ## Version
//! 0.1.0
//!
//! ## Author
//! VmFree <vmfree@example.com>
//!
//! ## Date
//! 2026-07-31

use serde::{Deserialize, Serialize};

/// 用户 ID
pub type UserId = alloc::string::String;

/// 组 ID
pub type GroupId = alloc::string::String;

/// 账户信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: UserId,
    pub name: alloc::string::String,
    pub uid: u32,
    pub gid: u32,
    pub home_dir: Option<alloc::string::String>,
    pub shell: Option<alloc::string::String>,
    pub account_type: AccountType,
    pub status: AccountStatus,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub tags: alloc::vec::Vec<alloc::string::String>,
    pub metadata: std::collections::HashMap<alloc::string::String, alloc::string::String>,
}

/// 账户类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccountType {
    /// 底层 OS 的用户 (来自 /etc/passwd)
    OsUser,
    /// LinAIx 独立管理的用户
    LinAIxUser,
    /// 服务/应用账户 (非交互式)
    ServiceAccount,
}

/// 账户状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccountStatus {
    Active,
    Locked,
    Expired,
    Pending,
}

/// 组信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
    pub name: alloc::string::String,
    pub gid: u32,
    pub members: alloc::vec::Vec<UserId>,
    pub metadata: std::collections::HashMap<alloc::string::String, alloc::string::String>,
}

/// 账户过滤器
#[derive(Debug, Clone, Default)]
pub struct AccountFilter {
    pub account_type: Option<AccountType>,
    pub status: Option<AccountStatus>,
    pub name_contains: Option<alloc::string::String>,
    pub tags: Option<alloc::vec::Vec<alloc::string::String>>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

/// 凭证数据 (用于验证)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CredentialData {
    pub credential_type: CredentialType,
    pub data: alloc::vec::Vec<u8>,
}

/// 凭证类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CredentialType {
    Password,
    ApiKey,
    SshKey,
    Certificate,
}