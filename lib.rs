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

//! # LinAIx
//!
//! The Linux of the AI era - A Rust-based kernel for AI Agents.
//!
//! ## Architecture
//!
//! LinAIx is organized as a set of subsystems, each with a clear responsibility:
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────────────────┐
//! │                      Application Layer                                 │
//! ├─────────────────────────────────────────────────────────────────────────┤
//! │  runtime/    │  skill/      │  framework/   │  scheduler/              │
//! │  (L5)        │  (L4)        │               │  (L3)                    │
//! ├─────────────────────────────────────────────────────────────────────────┤
//! │  resource/   │  task/       │  ipc/         │  obs/                    │
//! │  (L2)        │              │               │                          │
//! ├─────────────────────────────────────────────────────────────────────────┤
//! │  device/     │  model/      │  security/                              │
//! │  (L1)        │              │                                         │
//! └─────────────────────────────────────────────────────────────────────────┘
//! ```
//!
//! ## Subsystem Overview
//!
//! | Subsystem | Directory | Responsibility |
//! |-----------|-----------|----------------|
//! | Device HAL | `device/` | Hardware resource read/write abstraction |
//! | Model Runtime | `model/` | Model info + inference execution |
//! | Security | `security/` | Authentication/Authorization/Audit/Encryption |
//! | Resource Manager | `resource/` | Quota / KV Cache / Resource stats |
//! | Task | `task/` | Task/SubTask definition and management |
//! | Scheduler | `scheduler/` | SubTask scheduling |
//! | Skill API | `skill/` | Skill registration/invocation/adapter |
//! | Agent Runtime | `runtime/` | Agent execution environment |
//! | Framework | `framework/` | Agent framework management |
//! | IPC | `ipc/` | Inter-Agent communication |
//! | Observability | `obs/` | Metrics/Tracing/Logging/Dump/Events |
//!
//! ## Version
//! 0.1.0
//!
//! ## Author
//! VmFree <vmfree@example.com>
//!
//! ## Date
//! 2026-08-01

#![no_std]
#![deny(missing_docs)]
#![deny(unsafe_code)]
#![warn(clippy::all, clippy::pedantic)]

#[cfg(feature = "std")]
extern crate std;

#[macro_use]
extern crate alloc;

// ============================================================================
// L1 - Hardware Abstraction Layer
// ============================================================================

/// Device HAL - Hardware resource read/write abstraction
pub mod device;

/// Model Runtime - Model info + inference execution
pub mod model;

/// Security - Authentication/Authorization/Audit/Encryption
pub mod security;

// ============================================================================
// L2 - Resource Management
// ============================================================================

/// Resource Manager - Quota / KV Cache / Resource stats
pub mod resource;

/// Task - Task/SubTask definition and management
pub mod task;

/// IPC - Inter-Agent communication
pub mod ipc;

/// Observability - Metrics/Tracing/Logging/Dump/Events
pub mod obs;

// ============================================================================
// L3 - Scheduler
// ============================================================================

/// Scheduler - SubTask scheduling
pub mod scheduler;

// ============================================================================
// L4 - Skill API
// ============================================================================

/// Skill API - Skill registration/invocation/adapter
pub mod skill;

// ============================================================================
// L5 - Runtime & Framework
// ============================================================================

/// Agent Runtime - Agent execution environment
pub mod runtime;

/// Framework - Agent framework management
pub mod framework;

// ============================================================================
// Re-exports (可选)
// ============================================================================

// 各子系统类型可通过统一路径导入:
// use linAIx::prelude::*;

// pub mod prelude {
//     pub use crate::device::*;
//     pub use crate::model::*;
//     pub use crate::security::*;
//     pub use crate::resource::*;
//     pub use crate::task::*;
//     pub use crate::scheduler::*;
//     pub use crate::skill::*;
//     pub use crate::runtime::*;
//     pub use crate::framework::*;
//     pub use crate::ipc::*;
//     pub use crate::obs::*;
// }

// ============================================================================
// 版本信息
// ============================================================================

/// 项目版本
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// 项目名称
pub const NAME: &str = env!("CARGO_PKG_NAME");

/// 许可证
pub const LICENSE: &str = env!("CARGO_PKG_LICENSE");