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

//! # 审计接口
//!
//! ## Version
//! 0.1.0
//!
//! ## Author
//! VmFree <vmfree@example.com>
//!
//! ## Date
//! 2026-08-01

use crate::security::types::*;
use crate::security::error::SecurityError;

/// 审计接口
///
/// 职责：审计日志的记录、查询、导出、统计
pub trait Audit: Send + Sync {
    /// 记录审计事件
    fn log(&self, event: &AuditEvent) -> Result<(), SecurityError>;

    /// 批量记录审计事件
    fn log_batch(&self, events: &[AuditEvent]) -> Result<(), SecurityError>;

    /// 查询审计日志
    fn query(&self, filter: &AuditFilter) -> Result<alloc::vec::Vec<AuditEvent>, SecurityError>;

    /// 导出审计日志
    fn export(
        &self,
        filter: &AuditFilter,
        format: AuditExportFormat,
    ) -> Result<alloc::vec::Vec<u8>, SecurityError>;

    /// 获取审计统计
    fn stats(
        &self,
        start: chrono::DateTime<chrono::Utc>,
        end: chrono::DateTime<chrono::Utc>,
    ) -> Result<AuditStats, SecurityError>;

    /// 获取最近 N 条审计记录
    fn recent(&self, count: usize) -> Result<alloc::vec::Vec<AuditEvent>, SecurityError> {
        self.query(&AuditFilter {
            limit: Some(count),
            ..Default::default()
        })
    }
}