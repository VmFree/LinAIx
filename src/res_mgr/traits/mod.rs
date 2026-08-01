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

//! # L2 Resource Manager - 接口定义
//!
//! 按职责拆分为四个独立的 trait：
//! - [`ModelResource`]：模型实例管理
//! - [`KvCacheResource`]：KV Cache 管理
//! - [`QuotaResource`]：配额管理 (检测/报告)
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

pub mod model;
pub mod kv_cache;
pub mod quota;
pub mod stats;

pub use model::*;
pub use kv_cache::*;
pub use quota::*;
pub use stats::*;

/// 组合接口 (方便上层使用)
pub trait ResourceManager:
    ModelResource + KvCacheResource + QuotaResource + StatsResource
{
    fn as_model(&self) -> &dyn ModelResource;
    fn as_kv_cache(&self) -> &dyn KvCacheResource;
    fn as_quota(&self) -> &dyn QuotaResource;
    fn as_stats(&self) -> &dyn StatsResource;
}