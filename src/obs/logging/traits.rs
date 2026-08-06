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

//! # Logging 接口
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

/// 日志记录器接口
pub trait Logger: Send + Sync {
    /// 记录一条日志
    fn log(&self, entry: LogEntry) -> Result<(), ObsError>;

    /// 批量记录日志
    fn log_batch(&self, entries: &[LogEntry]) -> Result<(), ObsError>;

    /// 查询日志
    fn query(&self, filter: &LogFilter) -> Result<alloc::vec::Vec<LogEntry>, ObsError>;

    /// 获取日志配置
    fn config(&self) -> LogConfig;

    /// 更新日志级别 (动态)
    fn set_level(&mut self, level: LogLevel) -> Result<(), ObsError>;

    /// 刷新日志缓冲区
    fn flush(&self) -> Result<(), ObsError>;
}