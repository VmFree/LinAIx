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

//! # 推理后端类型
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

/// 推理后端类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BackendType {
    /// vLLM 推理引擎
    VLLM,

    /// llama.cpp 推理引擎
    LlamaCpp,

    /// TensorRT-LLM 推理引擎
    TensorRTLLM,

    /// HuggingFace Transformers
    Transformers,

    /// OpenAI API
    OpenAI,

    /// Anthropic API
    Anthropic,

    /// 自定义后端
    Custom(alloc::string::String),
}

/// 后端信息
#[derive(Debug, Clone)]
pub struct BackendInfo {
    pub backend_type: BackendType,
    pub name: alloc::string::String,
    pub version: alloc::string::String,
    pub description: Option<alloc::string::String>,

    /// 支持的能力
    pub capabilities: alloc::vec::Vec<BackendCapability>,

    /// 最大并发数
    pub max_concurrency: usize,

    /// 是否可用
    pub available: bool,

    /// 后端状态
    pub status: BackendStatus,
}

/// 后端状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BackendStatus {
    /// 就绪
    Ready,
    /// 繁忙
    Busy,
    /// 错误
    Error,
    /// 离线
    Offline,
}

/// 后端能力
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BackendCapability {
    Streaming,
    BatchInference,
    FunctionCalling,
    Embedding,
    Multimodal,
    ToolUse,
    Vision,
    Audio,
    ParallelGeneration,
}