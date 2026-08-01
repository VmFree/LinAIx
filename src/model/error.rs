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

//! # 模型推理错误类型
//!
//! ## Version
//! 0.1.0
//!
//! ## Author
//! VmFree <vmfree@example.com>
//!
//! ## Date
//! 2026-08-01

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ModelError {
    // ===== 模型信息查询 =====
    #[error("Model not found: {0}")]
    ModelNotFound(alloc::string::String),

    #[error("Model info query failed: {0}")]
    ModelInfoQueryFailed(alloc::string::String),

    // ===== 推理执行 =====
    #[error("Inference failed: {0}")]
    InferenceFailed(alloc::string::String),

    #[error("Inference timeout")]
    InferenceTimeout,

    #[error("Inference canceled")]
    InferenceCanceled,

    #[error("Model not ready: {0}")]
    ModelNotReady(alloc::string::String),

    #[error("Context window exceeded: max={max}, requested={requested}")]
    ContextWindowExceeded { max: usize, requested: usize },

    #[error("Invalid input: {0}")]
    InvalidInput(alloc::string::String),

    // ===== Token 估算 =====
    #[error("Token estimation failed: {0}")]
    TokenEstimationFailed(alloc::string::String),

    #[error("Max tokens exceeded: limit={limit}, requested={requested}")]
    MaxTokensExceeded { limit: u32, requested: u32 },

    #[error("Tokenizer not available for model: {0}")]
    TokenizerNotAvailable(alloc::string::String),

    // ===== 后端 =====
    #[error("Backend not found: {0}")]
    BackendNotFound(alloc::string::String),

    #[error("Backend unavailable: {0}")]
    BackendUnavailable(alloc::string::String),

    #[error("Unsupported operation on backend: {0}")]
    UnsupportedBackendOperation(alloc::string::String),

    // ===== 安全 =====
    #[error("Security check failed: {0}")]
    SecurityCheckFailed(alloc::string::String),

    // ===== 资源 =====
    #[error("Insufficient memory: requested={requested}, available={available}")]
    InsufficientMemory { requested: u64, available: u64 },

    #[error("Resource exhausted: {0}")]
    ResourceExhausted(alloc::string::String),

    // ===== 通用 =====
    #[error("Internal error: {0}")]
    Internal(alloc::string::String),

    #[error("Configuration error: {0}")]
    ConfigError(alloc::string::String),

    #[error("I/O error: {0}")]
    IoError(alloc::string::String),
}