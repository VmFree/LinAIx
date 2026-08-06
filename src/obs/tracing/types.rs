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

//! # Tracing 类型定义
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

/// Trace ID (全局唯一)
pub type TraceId = alloc::string::String;

/// Span ID (局部唯一)
pub type SpanId = alloc::string::String;

/// Span 类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpanKind {
    /// 内部操作
    Internal,

    /// 客户端请求
    Client,

    /// 服务端处理
    Server,

    /// 消息生产者
    Producer,

    /// 消息消费者
    Consumer,
}

/// Span 状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpanStatus {
    Unset,
    Ok,
    Error,
    Timeout,
    Canceled,
}

/// Span 上下文 (用于传播)
#[derive(Debug, Clone)]
pub struct SpanContext {
    pub trace_id: TraceId,
    pub span_id: SpanId,
    pub parent_span_id: Option<SpanId>,
    pub trace_flags: u8,
}

impl Default for SpanContext {
    fn default() -> Self {
        Self {
            trace_id: TraceId::default(),
            span_id: SpanId::default(),
            parent_span_id: None,
            trace_flags: 0,
        }
    }
}

/// Span
#[derive(Debug, Clone)]
pub struct Span {
    pub context: SpanContext,
    pub name: alloc::string::String,
    pub kind: SpanKind,
    pub status: SpanStatus,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,
    pub attributes: std::collections::HashMap<alloc::string::String, serde_json::Value>,
    pub events: alloc::vec::Vec<SpanEvent>,
}

/// Span 事件
#[derive(Debug, Clone)]
pub struct SpanEvent {
    pub name: alloc::string::String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub attributes: std::collections::HashMap<alloc::string::String, serde_json::Value>,
}

/// 采样决策
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SamplingDecision {
    /// 采样 (记录)
    Sample,

    /// 不采样 (丢弃)
    DontSample,

    /// 按策略决策
    DeferToPolicy,
}