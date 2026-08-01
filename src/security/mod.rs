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
//! 提供认证、授权、审计、加密、密钥管理和注入检测能力。
//!
//! ## 核心接口
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

pub mod error;
pub mod traits;
pub mod types;

// 各模块的具体实现 (预留)
// pub mod auth;
// pub mod authorization;
// pub mod audit;
// pub mod crypto;
// pub mod injection;

pub use traits::*;
pub use types::*;
pub use error::SecurityError;