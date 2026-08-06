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
//! ## 模块结构
//!
//! | 模块 | 职责 |
//! |------|------|
//! | [`model`] | 模型实例管理 |
//! | [`kv_cache`] | KV Cache 管理 |
//! | [`quota`] | 配额管理 (检测/报告) |
//! | [`stats`] | 资源统计与观测 |
//! | [`event`] | 资源事件定义 |
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

pub mod model;
pub mod kv_cache;
pub mod quota;
pub mod stats;
pub mod event;

// ===== 统一类型导出 =====

pub mod types {
    pub use crate::res_mgr::model::types::*;
    pub use crate::res_mgr::kv_cache::types::*;
    pub use crate::res_mgr::quota::types::*;
    pub use crate::res_mgr::stats::types::*;
    pub use crate::res_mgr::event::types::*;
}

// ===== 统一 Trait 导出 =====

pub mod traits {
    pub use crate::res_mgr::model::ModelResource;
    pub use crate::res_mgr::kv_cache::KvCacheResource;
    pub use crate::res_mgr::quota::QuotaResource;
    pub use crate::res_mgr::stats::StatsResource;
}

/// 组合接口 (方便上层使用)
pub trait ResourceManager:
    model::ModelResource + kv_cache::KvCacheResource + quota::QuotaResource + stats::StatsResource
{
    fn as_model(&self) -> &dyn model::ModelResource;
    fn as_kv_cache(&self) -> &dyn kv_cache::KvCacheResource;
    fn as_quota(&self) -> &dyn quota::QuotaResource;
    fn as_stats(&self) -> &dyn stats::StatsResource;
}

pub use error::ResourceError;