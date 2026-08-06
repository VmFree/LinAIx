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

//! # L5 Agent Runtime
//!
//! Agent Runtime 是 Agent 代码的实际执行环境，类比 Linux 的用户态进程。
//!
//! ## 核心概念
//!
//! | 概念 | 说明 | Linux 对应 |
//! |------|------|-----------|
//! | Agent Runtime | Agent 代码的执行环境 | 用户态进程 |
//! | Agent Sandbox | 内存/文件系统/网络隔离边界 | 进程地址空间 + namespace |
//! | Agent Lifecycle | 启动/暂停/恢复/终止 | 进程生命周期 |
//! | System Call Bridge | Agent 调用内核能力的桥梁 | syscall 指令 |
//!
//! ## 模块结构
//!
//! | 模块 | 职责 |
//! |------|------|
//! | [`agent`] | Agent 定义和接口 |
//! | [`lifecycle`] | Agent 生命周期管理 |
//! | [`sandbox`] | Agent 沙箱配置和管理 |
//! | [`bridge`] | 系统调用桥 |
//! | [`config`] | Agent 运行时配置 |
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
pub mod agent;
pub mod lifecycle;
pub mod sandbox;
pub mod bridge;
pub mod config;

pub use agent::*;
pub use lifecycle::*;
pub use sandbox::*;
pub use bridge::*;
pub use config::*;
pub use error::RuntimeError;