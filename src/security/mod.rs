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

//! # Security 子系统
//!
//! 提供账户管理、认证、授权、审计、加密、密钥管理和注入检测能力。
//!
//! ## 模块结构
//!
//! | 模块 | 职责 |
//! |------|------|
//! | [`account`] | 账户数据源抽象 (可插拔) |
//! | [`auth`] | 身份认证与 Token 管理 |
//! | [`authorization`] | 权限检查与策略管理 |
//! | [`audit`] | 审计日志记录与查询 |
//! | [`crypto`] | 加密/解密/哈希 |
//! | [`key_management`] | 密钥生命周期管理 |
//! | [`injection`] | Prompt 注入检测 |
//!
//! ## 版本
//! 0.1.0
//!
//! ## Author
//! VmFree <vmfree@example.com>
//!
//! ## Date
//! 2026-07-31

pub mod error;
pub mod types;              // 跨模块共享类型

pub mod account;
pub mod auth;
pub mod authorization;
pub mod audit;
pub mod crypto;
pub mod key_management;
pub mod injection;

// ===== 统一类型导出 =====

pub mod all_types {
    pub use crate::security::types::*;
    pub use crate::security::account::types::*;
    pub use crate::security::auth::types::*;
    pub use crate::security::authorization::types::*;
    pub use crate::security::audit::types::*;
    pub use crate::security::crypto::types::*;
    pub use crate::security::key_management::types::*;
    pub use crate::security::injection::types::*;
}

// ===== 统一 Trait 导出 =====

pub mod traits {
    pub use crate::security::account::AccountProvider;
    pub use crate::security::auth::Authentication;
    pub use crate::security::authorization::Authorization;
    pub use crate::security::audit::Audit;
    pub use crate::security::crypto::Crypto;
    pub use crate::security::key_management::KeyManagement;
    pub use crate::security::injection::InjectionDetection;
}

pub use error::SecurityError;
pub use types::{KeyId, TenantId};