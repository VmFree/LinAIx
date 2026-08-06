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

//! # 审计模块 (Audit)
//!
//! 负责审计日志的记录、查询、导出和统计。
//!
//! ## 核心组件
//!
//! | 组件 | 职责 |
//! |------|------|
//! | [`Audit`] | 审计接口 trait |
//! | [`DefaultAudit`] | 默认实现 |
//! | [`AuditEvent`] | 审计事件 |
//! | [`AuditFilter`] | 查询过滤器 |
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
// pub mod backend;

pub use traits::*;
pub use types::*;