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
//! 模型的实际加载/卸载由 `ModelScheduler` 模块管理。
//!
//! ## 版本
//! 0.1.0
//!
//! ## 作者
//! VmFree <vmfree@example.com>
//!
//! ## 日期
//! 2026-08-01

pub mod traits;

pub use traits::*;