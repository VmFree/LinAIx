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

//! # Agent 框架管理
//!
//! 管理 LinAIx 上安装和运行的 Agent 框架 (LangChain/CrewAI/AutoGen 等)。
//!
//! ## 模块结构
//!
//! | 模块 | 职责 |
//! |------|------|
//! | [`framework`] | 框架定义、管理、注册中心 |
//! | [`adapter`] | 框架适配器 (拦截+转换) |
//! | [`runtime`] | 框架运行时管理 |
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
pub mod framework;
pub mod adapter;
pub mod runtime;

pub use error::FrameworkError;