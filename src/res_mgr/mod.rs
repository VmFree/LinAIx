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

//! # L2 Resource Manager
//!
//! 资源管理器，负责 Agent 的资源配额管理、KV Cache 生命周期管理、
//! 模型实例管理、资源统计与观测。
//!
//! ## 核心接口
//! - [`ModelResource`]：模型实例管理
//! - [`KvCacheResource`]：KV Cache 管理
//! - [`QuotaResource`]：配额管理
//! - [`StatsResource`]：资源统计与观测
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
pub mod traits;
pub mod types;
pub mod model;
pub mod kv_cache;
pub mod quota;
pub mod stats;

pub use traits::*;
pub use types::*;
pub use error::ResourceError;