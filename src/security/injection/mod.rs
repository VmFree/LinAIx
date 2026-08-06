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

//! # Prompt 注入检测模块 (Injection Detection)
//!
//! 负责检测并防护 Prompt 注入攻击。
//!
//! ## 核心组件
//!
//! | 组件 | 职责 |
//! |------|------|
//! | [`InjectionDetection`] | 注入检测接口 trait |
//! | [`DefaultInjectionDetection`] | 默认实现 |
//! | [`InjectionDetectionResult`] | 检测结果 |
//! | [`RiskLevel`] | 风险等级 |
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
// pub mod patterns;

pub use traits::*;
pub use types::*;