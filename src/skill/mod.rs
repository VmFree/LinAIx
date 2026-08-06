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

//! # L4 Skill API
//!
//! Skill API 是 LinAIx 的"系统调用表"，类比 Linux 的系统调用机制。
//!
//! ## Skill ID 格式
//!
//! `{framework}/{namespace}/{name}@{version}`
//!
//! ## 模块结构
//!
//! | 模块 | 职责 |
//! |------|------|
//! | [`id`] | Skill ID 管理 |
//! | [`metadata`] | Skill 元数据 |
//! | [`invocation`] | Skill 调用 |
//! | [`result`] | Skill 结果 |
//! | [`permission`] | Skill 权限 |
//! | [`registry`] | 注册中心 |
//! | [`executor`] | 执行器 |
//! | [`adapter`] | 框架适配器 |
//!
//! ## 版本
//! 0.1.0
//!
//! ## Author
//! VmFree <vmfree@example.com>
//!
//! ## Date
//! 2026-08-01

pub mod error;
pub mod id;
pub mod metadata;
pub mod invocation;
pub mod result;
pub mod permission;
pub mod registry;
pub mod executor;
pub mod adapter;

// 统一导出类型
pub use id::*;
pub use metadata::*;
pub use invocation::*;
pub use result::*;
pub use permission::*;
pub use registry::*;
pub use executor::*;
pub use adapter::*;
pub use error::SkillError;