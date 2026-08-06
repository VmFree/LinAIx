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

//! # 认证接口
//!
//! ## Version
//! 0.1.0

use super::types::*;
use super::super::error::SecurityError;

/// 认证接口
///
/// 职责：Agent 身份认证、Token 管理
pub trait Authentication: Send + Sync {
    /// 认证 Agent 身份
    fn authenticate(&self, credentials: &Credentials) -> Result<AuthToken, SecurityError>;

    /// 验证 Token，返回认证上下文
    fn verify_token(&self, token: &AuthToken) -> Result<AuthContext, SecurityError>;

    /// 刷新 Token
    fn refresh_token(&self, token: &AuthToken) -> Result<AuthToken, SecurityError>;

    /// 撤销 Token
    fn revoke_token(&self, token: &AuthToken) -> Result<(), SecurityError>;

    /// 获取 Token 信息
    fn get_token_info(&self, token: &AuthToken) -> Result<AuthToken, SecurityError>;
}