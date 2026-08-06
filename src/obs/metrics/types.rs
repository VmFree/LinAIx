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

//! # Metrics 类型定义
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

/// 指标键
pub type MetricKey = alloc::string::String;

/// 指标类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MetricKind {
    /// 计数器 (单调递增)
    Counter,

    /// 仪表盘 (瞬时值)
    Gauge,

    /// 直方图 (分布)
    Histogram,

    /// 摘要 (分位数)
    Summary,
}

/// 指标值
#[derive(Debug, Clone)]
pub enum MetricValue {
    Counter(u64),
    Gauge(f64),
    Histogram(alloc::vec::Vec<f64>),
    Summary {
        count: u64,
        sum: f64,
        quantiles: alloc::vec::Vec<(f64, f64)>,
    },
}

/// 指标标签 (维度)
pub type MetricLabels = std::collections::HashMap<alloc::string::String, alloc::string::String>;

/// 指标快照
#[derive(Debug, Clone)]
pub struct MetricSnapshot {
    pub key: MetricKey,
    pub kind: MetricKind,
    pub value: MetricValue,
    pub labels: MetricLabels,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// 指标采集配置
#[derive(Debug, Clone)]
pub struct MetricsConfig {
    /// 采集间隔 (毫秒)
    pub interval_ms: u64,

    /// 是否启用
    pub enabled: bool,

    /// 最大保留数据点数量
    pub max_data_points: usize,

    /// 导出目标 (文件/OTLP/Stdout)
    pub export_targets: alloc::vec::Vec<ExportTarget>,
}

impl Default for MetricsConfig {
    fn default() -> Self {
        Self {
            interval_ms: 10000,
            enabled: true,
            max_data_points: 10000,
            export_targets: alloc::vec::Vec::new(),
        }
    }
}

/// 导出目标
#[derive(Debug, Clone)]
pub enum ExportTarget {
    File { path: alloc::string::String },
    Otlp { endpoint: alloc::string::String },
    Stdout,
    Prometheus,
}