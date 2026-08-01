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

//! # 运行时指标类型
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

/// 模型运行时指标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelMetrics {
    pub model_id: super::model::ModelId,

    /// 总推理次数
    pub total_inferences: u64,

    /// 总生成 Token 数
    pub total_tokens_generated: u64,

    /// 总输入 Token 数
    pub total_tokens_input: u64,

    /// 平均延迟 (毫秒)
    pub avg_latency_ms: f32,

    /// P50 延迟 (毫秒)
    pub p50_latency_ms: f32,

    /// P95 延迟 (毫秒)
    pub p95_latency_ms: f32,

    /// P99 延迟 (毫秒)
    pub p99_latency_ms: f32,

    /// 吞吐量 (Token/秒)
    pub throughput_tps: f32,

    /// 错误率
    pub error_rate: f32,

    /// 最大并发数
    pub max_concurrency: usize,

    /// 当前并发数
    pub current_concurrency: usize,

    /// 内存使用 (字节)
    pub memory_used_bytes: u64,

    /// 最大内存使用 (字节)
    pub memory_peak_bytes: u64,

    /// 最后更新时间
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// 全局模型指标 (所有模型汇总)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalModelMetrics {
    pub total_models_loaded: usize,
    pub total_inferences_all: u64,
    pub total_tokens_all: u64,
    pub active_inferences: usize,
    pub avg_latency_all_ms: f32,
    pub total_memory_used_bytes: u64,
    pub total_memory_available_bytes: u64,
}