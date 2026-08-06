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

//! # 框架定义
//!
//! 定义 LinAIx 支持的 Agent 框架类型和元数据。
//!
//! ## 模块结构
//!
//! | 模块 | 职责 |
//! |------|------|
//! | [`types`] | 框架类型定义 |
//! | [`traits`] | 框架管理接口 |
//! | [`registry`] | 框架注册中心 |
//!
//! ## 版本
//! 0.1.0

pub mod types;
pub mod traits;
pub mod registry;

pub use types::*;
pub use traits::*;
pub use registry::*;