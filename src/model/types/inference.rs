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

//! # 推理相关类型
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

/// 推理请求
#[derive(Debug, Clone)]
pub struct InferenceRequest {
    /// 请求唯一标识 (全链路追踪)
    pub request_id: alloc::string::String,

    /// 目标模型 ID
    pub model_id: super::model::ModelId,

    /// 消息列表
    pub messages: alloc::vec::Vec<Message>,

    /// 推理参数
    pub parameters: InferenceParameters,

    /// 是否流式输出
    pub stream: bool,

    /// 推理选项
    pub options: InferenceOptions,

    /// 请求创建时间 (由调用方填充)
    pub created_at: core::time::Instant,

    /// 本次请求的最大 Token 数
    pub max_total_tokens: Option<u32>,

    /// 输入 Token 数限制
    pub max_input_tokens: Option<u32>,
}

/// 消息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: MessageRole,
    pub content: alloc::string::String,
    pub name: Option<alloc::string::String>,
}

/// 消息角色
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessageRole {
    System,
    User,
    Assistant,
    Tool,
}

/// 推理参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferenceParameters {
    pub temperature: Option<f32>,
    pub top_p: Option<f32>,
    pub top_k: Option<u32>,
    pub max_tokens: Option<u32>,
    pub stop_sequences: Option<alloc::vec::Vec<alloc::string::String>>,
    pub presence_penalty: Option<f32>,
    pub frequency_penalty: Option<f32>,
    pub seed: Option<u64>,
    pub logprobs: Option<bool>,
    pub top_logprobs: Option<u32>,
}

/// 推理选项
#[derive(Debug, Clone, Default)]
pub struct InferenceOptions {
    pub timeout_ms: Option<u64>,
    pub priority_hint: Option<u8>,
    pub retry_on_failure: bool,
    pub max_retries: u8,
    pub trace_id: Option<alloc::string::String>,
}

/// 推理结果
#[derive(Debug, Clone)]
pub struct InferenceResult {
    pub request_id: alloc::string::String,
    pub model_id: super::model::ModelId,
    pub backend: super::backend::BackendType,
    pub response: alloc::string::String,

    /// Token 统计
    pub tokens_prompt: u32,
    pub tokens_completion: u32,
    pub tokens_total: u32,

    /// 是否截断
    pub truncated: bool,
    pub truncation_reason: Option<TruncationReason>,

    /// 上下文使用情况
    pub context_window_used: usize,
    pub context_window_max: usize,

    /// 时间线
    pub created_at: core::time::Instant,
    pub started_at: core::time::Instant,
    pub first_token_at: Option<core::time::Instant>,
    pub completed_at: core::time::Instant,

    /// 计算字段
    pub queue_wait_ms: u64,
    pub time_to_first_token_ms: Option<u64>,
    pub total_time_ms: u64,
    pub inference_time_ms: u64,

    /// 扩展元数据
    pub metadata: std::collections::HashMap<alloc::string::String, alloc::string::String>,
}

/// 截断原因
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TruncationReason {
    MaxTokensReached,
    MaxTotalTokensReached,
    MaxInputTokensReached,
    ContextWindowFull,
}

/// 流式输出
#[derive(Debug, Clone)]
pub struct StreamChunk {
    pub request_id: alloc::string::String,
    pub token: alloc::string::String,
    pub is_final: bool,
    pub tokens_so_far: u32,
    pub timestamp: core::time::Instant,
    pub latency_ms: u64,
}

/// 流式输出接口
#[async_trait::async_trait]
pub trait StreamOutput: Send + Sync {
    async fn next(&mut self) -> Option<Result<StreamChunk, super::error::ModelError>>;
    fn cancel(&self) -> Result<(), super::error::ModelError>;
}