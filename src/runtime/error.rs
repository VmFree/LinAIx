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

//! # Agent 运行时错误类型
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
pub enum RuntimeError {
    // ===== Agent 管理 =====
    #[error("Agent not found: {0}")]
    AgentNotFound(AgentId),

    #[error("Agent already exists: {0}")]
    AgentAlreadyExists(AgentId),

    #[error("Agent is not running: {0}")]
    AgentNotRunning(AgentId),

    #[error("Agent is already running: {0}")]
    AgentAlreadyRunning(AgentId),

    // ===== 生命周期 =====
    #[error("Invalid lifecycle transition: {0:?} -> {1:?}")]
    InvalidLifecycleTransition(LifecycleState, LifecycleState),

    #[error("Agent lifecycle timeout: {0}")]
    LifecycleTimeout(alloc::string::String),

    #[error("Agent lifecycle failed: {0}")]
    LifecycleFailed(alloc::string::String),

    // ===== 沙箱 =====
    #[error("Sandbox creation failed: {0}")]
    SandboxCreationFailed(alloc::string::String),

    #[error("Sandbox limit exceeded: {0}")]
    SandboxLimitExceeded(alloc::string::String),

    #[error("Sandbox operation not permitted: {0}")]
    SandboxNotPermitted(alloc::string::String),

    // ===== 系统调用 =====
    #[error("System call failed: {0}")]
    SystemCallFailed(alloc::string::String),

    #[error("System call not allowed: {0}")]
    SystemCallNotAllowed(alloc::string::String),

    #[error("System call timeout: {0}")]
    SystemCallTimeout(alloc::string::String),

    // ===== 配置 =====
    #[error("Invalid agent config: {0}")]
    InvalidConfig(alloc::string::String),

    #[error("Missing agent config: {0}")]
    MissingConfig(alloc::string::String),

    // ===== 通用 =====
    #[error("Internal runtime error: {0}")]
    Internal(alloc::string::String),

    #[error("I/O error: {0}")]
    IoError(alloc::string::String),
}