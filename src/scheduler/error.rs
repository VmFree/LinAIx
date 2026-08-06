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

//! # 调度器错误类型
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
pub enum SchedulerError {
    // ===== SubTask 调度 =====
    #[error("SubTask not found: {0}")]
    SubTaskNotFound(SubTaskId),

    #[error("SubTask is not ready for execution: {0}")]
    SubTaskNotReady(SubTaskId),

    #[error("SubTask is already running: {0}")]
    SubTaskAlreadyRunning(SubTaskId),

    #[error("SubTask is already in runqueue: {0}")]
    SubTaskAlreadyQueued(SubTaskId),

    // ===== 运行队列 =====
    #[error("Runqueue is empty")]
    RunQueueEmpty,

    #[error("Runqueue is full")]
    RunQueueFull,

    // ===== 等待队列 =====
    #[error("SubTask not in waitqueue: {0}")]
    SubTaskNotWaiting(SubTaskId),

    #[error("Waitqueue is empty")]
    WaitQueueEmpty,

    // ===== 资源 =====
    #[error("Insufficient resources for SubTask: {0}")]
    InsufficientResources(SubTaskId),

    #[error("Task quota exceeded: {0}")]
    TaskQuotaExceeded(TaskId),

    // ===== 抢占 =====
    #[error("Cannot preempt current SubTask: {0}")]
    CannotPreempt(SubTaskId),

    #[error("Preemption disabled")]
    PreemptionDisabled,

    // ===== 调度类 =====
    #[error("No matching scheduling class for SubTask: {0}")]
    NoMatchingSchedClass(SubTaskId),

    // ===== 通用 =====
    #[error("Internal scheduler error: {0}")]
    Internal(alloc::string::String),

    #[error("Configuration error: {0}")]
    ConfigError(alloc::string::String),
}