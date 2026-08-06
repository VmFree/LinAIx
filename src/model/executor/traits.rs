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

//! # 模型执行接口
//!
//! 职责：提供统一的推理执行抽象，不涉及模型加载/卸载/生命周期管理
//!
//! ## 版本
//! 0.1.0
//!
//! ## Author
//! VmFree <vmfree@example.com>
//!
//! ## Date
//! 2026-08-01

use super::types::*;
use crate::model::types::*;
use crate::model::error::ModelError;

/// 统一推理执行接口
///
/// 模型调用方 (L2/L3) 通过此接口执行推理，不关心模型是如何加载的。
/// 模型的实际加载/卸载由 `ModelScheduler` 模块管理。
pub trait ModelExecutor: Send + Sync {
    /// 执行推理 (同步)
    ///
    /// # 注意
    /// 调用方需要确保模型已加载 (由 ModelScheduler 保证)
    fn infer(&self, request: &InferenceRequest) -> Result<InferenceResult, ModelError>;

    /// 执行推理 (异步)
    async fn infer_async(&self, request: &InferenceRequest) -> Result<InferenceResult, ModelError>;

    /// 流式推理
    fn infer_stream(&self, request: &InferenceRequest) -> Result<Box<dyn StreamOutput>, ModelError>;

    /// 批量推理
    fn infer_batch(&self, requests: &[InferenceRequest]) -> Result<alloc::vec::Vec<InferenceResult>, ModelError>;

    /// 获取模型执行指标
    fn get_metrics(&self, model_id: &ModelId) -> Result<ModelMetrics, ModelError>;

    /// 获取所有模型的全局指标
    fn get_global_metrics(&self) -> Result<GlobalModelMetrics, ModelError>;

    /// 检查模型是否就绪 (已加载且可用)
    fn is_ready(&self, model_id: &ModelId) -> bool;

    /// 获取当前可用的后端列表
    fn list_backends(&self) -> alloc::vec::Vec<BackendInfo>;

    /// 获取当前模型使用的后端类型
    fn get_backend(&self, model_id: &ModelId) -> Result<BackendType, ModelError>;

    /// 检查模型是否支持特定能力
    fn supports_capability(&self, model_id: &ModelId, capability: ModelCapability) -> bool;
}