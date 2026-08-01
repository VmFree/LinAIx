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

//! # Security 子系统 - 接口定义
//!
//! 按职责拆分为 6 个独立的 trait：
//!
//! | Trait | 职责 |
//! |-------|------|
//! | [`Authentication`] | 身份认证 |
//! | [`Authorization`] | 权限检查与策略管理 |
//! | [`Audit`] | 审计日志 |
//! | [`Crypto`] | 加密/解密/哈希 |
//! | [`KeyManagement`] | 密钥生命周期管理 |
//! | [`InjectionDetection`] | Prompt 注入检测 |
//!
//! ## 版本
//! 0.1.0
//!
//! ## Author
//! VmFree <vmfree@example.com>
//!
//! ## Date
//! 2026-07-31

pub mod auth;
pub mod authorization;
pub mod audit;
pub mod crypto;
pub mod key_management;
pub mod injection;

pub use auth::*;
pub use authorization::*;
pub use audit::*;
pub use crypto::*;
pub use key_management::*;
pub use injection::*;

/// 组合接口 (方便上层使用)
pub trait Security:
    Authentication + Authorization + Audit + Crypto + KeyManagement + InjectionDetection
{
    fn as_auth(&self) -> &dyn Authentication;
    fn as_authorization(&self) -> &dyn Authorization;
    fn as_audit(&self) -> &dyn Audit;
    fn as_crypto(&self) -> &dyn Crypto;
    fn as_key_management(&self) -> &dyn KeyManagement;
    fn as_injection(&self) -> &dyn InjectionDetection;
}