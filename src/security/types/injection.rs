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

//! # Prompt 注入检测相关类型
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InjectionDetectionResult {
    pub is_safe: bool,
    pub risk_level: RiskLevel,
    pub detected_patterns: alloc::vec::Vec<InjectionPattern>,
    pub sanitized_text: alloc::string::String,
    pub was_modified: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum RiskLevel {
    Safe = 0,
    Low = 1,
    Medium = 2,
    High = 3,
    Critical = 4,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InjectionPattern {
    pub pattern_type: InjectionPatternType,
    pub matched_text: alloc::string::String,
    pub start_pos: usize,
    pub end_pos: usize,
    pub severity: RiskLevel,
    pub description: alloc::string::String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InjectionPatternType {
    SystemInstructionOverride,
    RolePlayInjection,
    PrivilegeEscalation,
    ShellCommandInjection,
    SqlInjection,
    TemplateInjection,
    Jailbreak,
    PromptLeakage,
    SensitiveInfoExtraction,
    CodeInjection,
    Custom(alloc::string::String),
}