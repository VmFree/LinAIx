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

//! # Prompt 注入检测接口
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

/// Prompt 注入检测接口
///
/// 职责：检测并防护 Prompt 注入攻击
pub trait InjectionDetection: Send + Sync {
    /// 检测并清理输入
    fn detect(&self, input: &str) -> InjectionDetectionResult;

    /// 检查输入是否安全
    fn is_safe(&self, input: &str) -> bool {
        self.detect(input).is_safe
    }

    /// 获取输入的风险等级
    fn risk_level(&self, input: &str) -> RiskLevel {
        self.detect(input).risk_level
    }

    /// 注册自定义注入模式
    fn register_pattern(&mut self, pattern: InjectionPattern) -> Result<(), SecurityError>;

    /// 获取所有已注册的模式
    fn list_patterns(&self) -> alloc::vec::Vec<InjectionPattern>;
}