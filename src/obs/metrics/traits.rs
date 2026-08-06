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

//! # Metrics 接口
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

/// 指标采集器接口
///
/// 职责：从各模块采集指标数据
pub trait MetricsCollector: Send + Sync {
    /// 注册指标
    fn register(&mut self, key: MetricKey, kind: MetricKind, labels: MetricLabels) -> Result<(), ObsError>;

    /// 更新指标值
    fn update(&self, key: &MetricKey, value: MetricValue) -> Result<(), ObsError>;

    /// 增加计数器 (便捷方法)
    fn inc_counter(&self, key: &MetricKey, delta: u64) -> Result<(), ObsError>;

    /// 设置仪表盘值 (便捷方法)
    fn set_gauge(&self, key: &MetricKey, value: f64) -> Result<(), ObsError>;

    /// 记录直方图值 (便捷方法)
    fn record_histogram(&self, key: &MetricKey, value: f64) -> Result<(), ObsError>;

    /// 获取指标快照
    fn snapshot(&self, key: &MetricKey) -> Result<MetricSnapshot, ObsError>;

    /// 获取所有指标快照
    fn snapshot_all(&self) -> alloc::vec::Vec<MetricSnapshot>;

    /// 获取指标配置
    fn config(&self) -> MetricsConfig;

    /// 导出指标
    fn export(&self, target: &ExportTarget) -> Result<alloc::vec::Vec<u8>, ObsError>;
}