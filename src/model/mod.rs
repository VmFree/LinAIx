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

//! # 模型推理子系统 (Model Runtime)
//!
//! 提供模型信息的统一查询和推理执行的统一接口。
//! 模型加载/卸载/生命周期由 `model_scheduler` 模块管理。
//!
//! ## 模块结构
//!
//! | 模块 | 职责 |
//! |------|------|
//! | [`types`] | 共享类型定义 |
//! | [`info`] | 模型信息查询 |
//! | [`executor`] | 推理执行 |
//!
//! ## 核心接口
//! - [`ModelInfoProvider`]：模型信息查询
//! - [`ModelExecutor`]：推理执行
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
pub mod types;
pub mod info;
pub mod executor;

pub use info::*;
pub use executor::*;
pub use types::*;
pub use error::ModelError;