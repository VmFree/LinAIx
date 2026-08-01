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

//! # 模型资源管理接口
//!
//! ## Version
//! 0.1.0
//!
//! ## Author
//! VmFree <vmfree@example.com>
//!
//! ## Date
//! 2026-08-01

use crate::resource::types::*;
use crate::resource::error::ResourceError;

/// 模型资源管理接口
///
/// 职责：模型实例的获取/释放/预加载/卸载
pub trait ModelResource: Send + Sync {
    /// 获取或加载模型实例
    ///
    /// # 返回
    /// - 如果实例已在池中，直接返回
    /// - 如果实例不存在，触发加载流程
    fn get_model_instance(
        &self,
        model_id: &ModelId,
        config: Option<LoadConfig>,
    ) -> Result<ModelInstanceHandle, ResourceError>;

    /// 释放模型实例 (减少引用计数)
    ///
    /// # 注意
    /// 引用计数归零后，实例可能被缓存或直接释放
    fn release_model_instance(&self, handle: &ModelInstanceHandle) -> Result<(), ResourceError>;

    /// 预加载模型 (提前加载到缓存)
    ///
    /// # 用途
    /// 在流量高峰前预热模型，减少冷启动延迟
    fn preload_model(&self, model_id: &ModelId) -> Result<(), ResourceError>;

    /// 卸载模型 (强制释放)
    ///
    /// # 用途
    /// 运维场景下强制释放模型，释放内存资源
    fn unload_model(&self, model_id: &ModelId) -> Result<(), ResourceError>;

    /// 获取实例池状态
    fn get_instance_pool_status(&self) -> InstancePoolStatus;

    /// 获取当前所有活跃的模型实例
    fn list_active_instances(&self) -> alloc::vec::Vec<ModelInstanceInfo>;
}