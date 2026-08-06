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

//! # 任务管理子系统 (Task Management)
//!
//! ## 模块结构
//!
//! | 模块 | 职责 |
//! |------|------|
//! | [`task`] | Task 管理 (资源分配单位) |
//! | [`subtask`] | SubTask 管理 (执行调度单位) |
//!
//! ## 核心概念
//!
//! | 概念 | 说明 | 类比 |
//! |------|------|------|
//! | Task | 资源分配的基本单位，拥有配额 | Linux 进程 |
//! | SubTask | 执行调度的基本单位，共享 Task 资源 | Linux 线程 |
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
pub mod task;
pub mod subtask;

// 具体实现 (预留)
// pub mod default;

pub use task::*;
pub use subtask::*;
pub use error::TaskError;