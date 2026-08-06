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

//! # IPC 子系统
//!
//! 提供 Agent 间通信的基础设施。
//!
//! ## 模块结构
//!
//! | 模块 | 职责 |
//! |------|------|
//! | [`channel`] | IPC 通道管理 |
//! | [`message`] | 消息定义 |
//! | [`queue`] | 消息队列 |
//! | [`semaphore`] | 同步原语 |
//! | [`shm`] | 共享内存 |
//! | [`endpoint`] | 端点管理 |
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
pub mod channel;
pub mod message;
pub mod queue;
pub mod semaphore;
pub mod shm;
pub mod endpoint;

pub use channel::*;
pub use message::*;
pub use queue::*;
pub use semaphore::*;
pub use shm::*;
pub use endpoint::*;
pub use error::IpcError;