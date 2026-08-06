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

//! # 任务错误类型
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
pub enum TaskError {
    // ===== Task 错误 =====
    #[error("Task not found: {0}")]
    TaskNotFound(TaskId),

    #[error("Task already exists: {0}")]
    TaskAlreadyExists(TaskId),

    #[error("Task is not active: {0}")]
    TaskNotActive(TaskId),

    #[error("Task quota exceeded: {0}")]
    TaskQuotaExceeded(alloc::string::String),

    // ===== SubTask 错误 =====
    #[error("SubTask not found: {0}")]
    SubTaskNotFound(SubTaskId),

    #[error("SubTask already exists: {0}")]
    SubTaskAlreadyExists(SubTaskId),

    #[error("SubTask dependency cycle detected: {0:?}")]
    DependencyCycle(Vec<SubTaskId>),

    #[error("SubTask dependency not satisfied: {0} depends on {1}")]
    DependencyNotSatisfied(SubTaskId, SubTaskId),

    #[error("Invalid SubTask state transition: {0} -> {1}")]
    InvalidStateTransition(SubTaskStatus, SubTaskStatus),

    // ===== 通用 =====
    #[error("Internal error: {0}")]
    Internal(alloc::string::String),

    #[error("Configuration error: {0}")]
    ConfigError(alloc::string::String),
}