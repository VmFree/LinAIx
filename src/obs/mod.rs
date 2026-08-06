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

//! # Observability (可观测性) 子系统
//!
//! 提供指标采集、链路追踪、日志记录、状态转储、性能剖析和事件总线能力。
//!
//! ## 模块结构
//!
//! | 模块 | 职责 |
//! |------|------|
//! | [`metrics`] | 指标采集 (从 resource/model 拉取) |
//! | [`tracing`] | 链路追踪 (调用链) |
//! | [`logging`] | 系统日志 |
//! | [`dump`] | 状态转储 |
//! | [`profiling`] | 性能剖析 |
//! | [`event`] | 事件总线 (整合 resource/event) |
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
pub mod metrics;
pub mod tracing;
pub mod logging;
pub mod dump;
pub mod profiling;
pub mod event;

pub use metrics::*;
pub use tracing::*;
pub use logging::*;
pub use dump::*;
pub use profiling::*;
pub use event::*;
pub use error::ObsError;