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

//! # 框架管理错误类型
//!
//! ## Version
//! 0.1.0
//!
//! ## Author
//! VmFree <vmfree@example.com>
//!
//! ## Date
//! 2026-08-01

use thiserror::Error;

#[derive(Debug, Error)]
pub enum FrameworkError {
    // ===== 框架管理 =====
    #[error("Framework not found: {0}")]
    FrameworkNotFound(FrameworkId),

    #[error("Framework already installed: {0}")]
    FrameworkAlreadyInstalled(alloc::string::String),

    #[error("Framework installation failed: {0}")]
    FrameworkInstallFailed(alloc::string::String),

    #[error("Framework uninstall failed: {0}")]
    FrameworkUninstallFailed(alloc::string::String),

    #[error("Framework version conflict: {0}")]
    FrameworkVersionConflict(alloc::string::String),

    #[error("Framework not compatible: {0}")]
    FrameworkNotCompatible(alloc::string::String),

    // ===== 框架运行时 =====
    #[error("Framework runtime not found: {0}")]
    RuntimeNotFound(RuntimeHandle),

    #[error("Framework runtime already exists for task: {0}")]
    RuntimeAlreadyExists(TaskId),

    #[error("Framework runtime start failed: {0}")]
    RuntimeStartFailed(alloc::string::String),

    #[error("Framework runtime stop failed: {0}")]
    RuntimeStopFailed(alloc::string::String),

    #[error("Framework runtime pause failed: {0}")]
    RuntimePauseFailed(alloc::string::String),

    #[error("Framework runtime resume failed: {0}")]
    RuntimeResumeFailed(alloc::string::String),

    #[error("Framework runtime in invalid state: {0:?}")]
    InvalidRuntimeState(RuntimeStatus),

    // ===== 框架适配器 =====
    #[error("Adapter not found for framework: {0}")]
    AdapterNotFound(alloc::string::String),

    #[error("Adapter conversion failed: {0}")]
    AdapterConversionFailed(alloc::string::String),

    #[error("Adapter intercept failed: {0}")]
    AdapterInterceptFailed(alloc::string::String),

    // ===== 框架配置 =====
    #[error("Invalid framework config: {0}")]
    InvalidConfig(alloc::string::String),

    #[error("Missing framework config: {0}")]
    MissingConfig(alloc::string::String),

    // ===== 通用 =====
    #[error("Internal framework error: {0}")]
    Internal(alloc::string::String),

    #[error("Timeout: {0}")]
    Timeout(alloc::string::String),
}