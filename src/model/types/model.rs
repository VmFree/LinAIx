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

//! # 模型信息类型 (纯元数据)
//!
//! ## Version
//! 0.1.0
//!
//! ## Author
//! VmFree <vmfree@example.com>
//!
//! ## Date
//! 2026-08-01

use serde::{Deserialize, Serialize};

/// 模型唯一标识
pub type ModelId = alloc::string::String;

/// 模型信息 (纯元数据)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub id: ModelId,
    pub name: alloc::string::String,
    pub version: alloc::string::String,
    pub description: Option<alloc::string::String>,

    /// 模型架构 (如 "llama", "gpt", "bert")
    pub architecture: alloc::string::String,

    /// 参数量
    pub parameter_count: u64,

    /// 上下文窗口大小
    pub context_window: usize,

    /// 量化方式 (如 "Q4_K_M", "FP16")
    pub quantization: Option<alloc::string::String>,

    /// 支持的精度
    pub supported_precisions: alloc::vec::Vec<Precision>,

    /// 模型能力
    pub capabilities: alloc::vec::Vec<ModelCapability>,

    /// 模型格式
    pub format: ModelFormat,

    /// 标签 (用于搜索和分类)
    pub tags: alloc::vec::Vec<alloc::string::String>,

    /// 注册时间
    pub registered_at: chrono::DateTime<chrono::Utc>,

    /// 最后更新时间
    pub updated_at: chrono::DateTime<chrono::Utc>,

    /// 模型大小 (字节)
    pub size_bytes: u64,
}

/// 精度类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Precision {
    F32,
    F16,
    BF16,
    F8,
    INT8,
    INT4,
    INT3,
    INT2,
}

/// 模型能力
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModelCapability {
    TextCompletion,
    ChatCompletion,
    FunctionCalling,
    ToolUse,
    Multimodal,
    Streaming,
    Embedding,
    FineTuning,
    Vision,
    Audio,
}

/// 模型格式
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModelFormat {
    Gguf,
    Safetensors,
    Onnx,
    TensorRT,
    OpenAi,
    Custom(alloc::string::String),
}