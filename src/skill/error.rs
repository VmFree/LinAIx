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

//! # Skill 错误类型
//!
//! ## Version
//! 0.1.0

use thiserror::Error;

#[derive(Debug, Error)]
pub enum SkillError {
    // ===== 注册与发现 =====
    #[error("Skill not found: {0}")]
    SkillNotFound(alloc::string::String),

    #[error("Skill already registered: {0}")]
    SkillAlreadyRegistered(alloc::string::String),

    #[error("Invalid Skill ID: {0}")]
    InvalidSkillId(alloc::string::String),

    #[error("Invalid Skill metadata: {0}")]
    InvalidMetadata(alloc::string::String),

    // ===== 调用 =====
    #[error("Permission denied: {0}")]
    PermissionDenied(alloc::string::String),

    #[error("Input validation failed: {0}")]
    InputValidationFailed(alloc::string::String),

    #[error("Output validation failed: {0}")]
    OutputValidationFailed(alloc::string::String),

    #[error("Quota exceeded: {0}")]
    QuotaExceeded(alloc::string::String),

    #[error("Injection detected: {0}")]
    InjectionDetected(alloc::string::String),

    #[error("Execution failed: {0}")]
    ExecutionFailed(alloc::string::String),

    #[error("Execution timeout")]
    ExecutionTimeout,

    #[error("Execution canceled")]
    ExecutionCanceled,

    // ===== 版本 =====
    #[error("Version not found: {0}@{1}")]
    VersionNotFound(alloc::string::String, alloc::string::String),

    #[error("Version deprecated: {0}")]
    VersionDeprecated(alloc::string::String),

    // ===== 适配器 =====
    #[error("Adapter not found for framework: {0}")]
    AdapterNotFound(alloc::string::String),

    #[error("Adapter conversion failed: {0}")]
    AdapterConversionFailed(alloc::string::String),

    // ===== 通用 =====
    #[error("Internal error: {0}")]
    Internal(alloc::string::String),

    #[error("Configuration error: {0}")]
    ConfigError(alloc::string::String),
}