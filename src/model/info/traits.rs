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

//! # 模型信息管理接口
//!
//! ## Version
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

/// 模型信息提供者
///
/// 职责：模型的元数据查询，不涉及模型的实际加载/卸载
pub trait ModelInfoProvider: Send + Sync {
    /// 获取模型信息
    fn get_model_info(&self, model_id: &ModelId) -> Result<ModelInfo, ModelError>;

    /// 列出所有可用模型
    fn list_models(&self) -> alloc::vec::Vec<ModelInfo>;

    /// 按能力筛选模型
    fn find_models_by_capability(&self, capability: ModelCapability) -> alloc::vec::Vec<ModelInfo>;

    /// 按名称搜索模型 (支持模糊匹配)
    fn search_models_by_name(&self, name_pattern: &str) -> alloc::vec::Vec<ModelInfo>;

    /// 按标签筛选模型
    fn find_models_by_tags(&self, tags: &[alloc::string::String]) -> alloc::vec::Vec<ModelInfo>;

    /// 获取模型支持的精度列表
    fn get_supported_precisions(&self, model_id: &ModelId) -> Result<alloc::vec::Vec<Precision>, ModelError>;

    /// 获取模型上下文窗口大小
    fn get_context_window(&self, model_id: &ModelId) -> Result<usize, ModelError>;

    /// 估算文本的 Token 数量
    fn estimate_tokens(&self, model_id: &ModelId, text: &str) -> Result<usize, ModelError>;

    /// 批量估算 Token
    fn estimate_tokens_batch(&self, model_id: &ModelId, texts: &[&str]) -> Result<alloc::vec::Vec<usize>, ModelError>;

    /// 获取模型 Tokenizer 信息
    fn get_tokenizer_info(&self, model_id: &ModelId) -> Result<TokenizerInfo, ModelError>;

    /// 检查模型是否存在
    fn model_exists(&self, model_id: &ModelId) -> bool;
}