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

//! # 授权模块 (Authorization)
//!
//! 负责权限检查和策略管理。
//!
//! ## 核心组件
//!
//! | 组件 | 职责 |
//! |------|------|
//! | [`Authorization`] | 授权接口 trait |
//! | [`DefaultAuthorization`] | 默认实现 (RBAC + ABAC) |
//! | [`Permission`] | 权限定义 |
//! | [`Policy`] | 策略定义 |
//!
//! ## 版本
//! 0.1.0
//!
//! ## Author
//! VmFree <vmfree@example.com>
//!
//! ## Date
//! 2026-07-31

pub mod traits;
pub mod types;

// 具体实现 (预留)
// pub mod default;
// pub mod engine;

pub use traits::*;
pub use types::*;