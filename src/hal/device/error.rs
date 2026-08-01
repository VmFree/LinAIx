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

//! # 设备错误类型
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

/// 设备错误
#[derive(Debug, Error)]
pub enum DeviceError {
    #[error("Device not found: {0}")]
    DeviceNotFound(alloc::string::String),

    #[error("Device not ready: {0}")]
    DeviceNotReady(alloc::string::String),

    #[error("Allocation failed: {0}")]
    AllocationFailed(alloc::string::String),

    #[error("Read failed: {0}")]
    ReadFailed(alloc::string::String),

    #[error("Write failed: {0}")]
    WriteFailed(alloc::string::String),

    #[error("Invalid offset: {0}")]
    InvalidOffset(u64),

    #[error("Invalid size: {0}")]
    InvalidSize(usize),

    #[error("Out of memory: requested={requested}, available={available}")]
    OutOfMemory { requested: u64, available: u64 },

    #[error("Unsupported control command: {0}")]
    UnsupportedControl,

    #[error("Control command failed: {0}")]
    ControlFailed(alloc::string::String),

    #[error("I/O error: {0}")]
    IoError(alloc::string::String),

    #[error("Internal error: {0}")]
    Internal(alloc::string::String),
}