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

//! # 认证相关类型
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

/// Agent 身份标识
pub type AgentId = alloc::string::String;

/// 租户标识
pub type TenantId = alloc::string::String;

/// 用户标识
pub type UserId = alloc::string::String;

/// 认证凭证
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credentials {
    pub credential_type: CredentialType,
    pub credential_data: alloc::vec::Vec<u8>,
    pub metadata: std::collections::HashMap<alloc::string::String, alloc::string::String>,
}

/// 凭证类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CredentialType {
    ApiKey,
    BearerToken,
    OAuth2,
    ClientCertificate,
    BasicAuth,
    SessionToken,
}

/// 认证 Token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthToken {
    pub token: alloc::string::String,
    pub agent_id: AgentId,
    pub issued_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub token_type: AuthTokenType,
    pub claims: std::collections::HashMap<alloc::string::String, alloc::string::String>,
}

/// Token 类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuthTokenType {
    Access,
    Refresh,
    Service,
}

/// 认证上下文
#[derive(Debug, Clone)]
pub struct AuthContext {
    pub agent_id: AgentId,
    pub tenant_id: TenantId,
    pub user_id: Option<UserId>,
    pub roles: alloc::vec::Vec<Role>,
    pub permissions: alloc::vec::Vec<Permission>,
    pub authenticated_at: chrono::DateTime<chrono::Utc>,
    pub token: AuthToken,
    pub security_labels: alloc::vec::Vec<SecurityLabel>,
}

/// 角色
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    pub name: alloc::string::String,
    pub description: Option<alloc::string::String>,
}

/// 安全标签
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityLabel {
    pub key: alloc::string::String,
    pub value: alloc::string::String,
}