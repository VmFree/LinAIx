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

//! # Tracing 接口
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
use crate::obs::error::ObsError;

/// 追踪提供者接口
pub trait TracingProvider: Send + Sync {
    /// 开始一个新的 Span
    fn start_span(
        &self,
        name: alloc::string::String,
        parent_context: Option<SpanContext>,
        kind: SpanKind,
    ) -> Span;

    /// 结束一个 Span
    fn end_span(&self, span: Span) -> Result<(), ObsError>;

    /// 获取当前 Span 上下文
    fn current_context(&self) -> Option<SpanContext>;

    /// 获取 Trace 的所有 Span
    fn get_trace(&self, trace_id: &TraceId) -> Result<alloc::vec::Vec<Span>, ObsError>;

    /// 决定是否采样
    fn should_sample(&self, trace_id: &TraceId) -> SamplingDecision;

    /// 设置采样率
    fn set_sampling_rate(&mut self, rate: f32) -> Result<(), ObsError>;

    /// 导出追踪数据 (OTLP 格式)
    fn export(&self, trace_id: &TraceId) -> Result<alloc::vec::Vec<u8>, ObsError>;
}