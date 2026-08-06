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

//! # 模型执行器
//!
//! 提供统一的推理执行接口，不涉及模型的加载/卸载/生命周期管理。
//!
//! ## 核心组件
//!
//! | 组件 | 职责 |
//! |------|------|
//! | [`ModelExecutor`] | 推理执行接口 |
//! | [`StreamOutput`] | 流式输出接口 |
//!
//! ## 版本
//! 0.1.0
//!
//! ## Author
//! VmFree <vmfree@example.com>
//!
//! ## Date
//! 2026-08-01

pub mod traits;
pub mod types;

pub use traits::*;
pub use types::*;