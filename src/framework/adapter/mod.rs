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

//! # 框架适配器
//!
//! 负责拦截框架调用，转换为 LinAIx 内核调用。
//!
//! ## 适配方向
//!
//! 1. **正向适配 (框架 → LinAIx)**：拦截框架 Skill/模型调用，转为 LinAIx 调用
//! 2. **反向适配 (LinAIx → 框架)**：将 LinAIx Skill 包装为框架原生工具
//!
//! ## 核心职责
//!
//! - 拦截框架的模型调用，附加 TaskId/SubTaskId
//! - 拦截框架的 Skill 调用，转为 LinAIx Skill 调用
//! - 检测框架子任务创建，上报 SubTaskManager
//! - 处理配额超限错误
//!
//! ## 版本
//! 0.1.0

pub mod types;
pub mod traits;

pub use types::*;
pub use traits::*;

// 具体框架适配器 (预留)
// pub mod langchain;
// pub mod crewai;
// pub mod autogen;