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

//! # Profiling 类型定义
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

/// 剖析类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProfileType {
    /// CPU 使用
    Cpu,

    /// 内存分配
    Memory,

    /// I/O 阻塞
    Block,

    /// 锁竞争
    Lock,
}

/// 剖析采样点
#[derive(Debug, Clone)]
pub struct ProfileSample {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub value: f64,
    pub stack: alloc::vec::Vec<alloc::string::String>,
}

/// 剖析数据
#[derive(Debug, Clone)]
pub struct ProfileData {
    pub profile_type: ProfileType,
    pub target: alloc::string::String,  // task_id 或 agent_id
    pub samples: alloc::vec::Vec<ProfileSample>,
    pub duration_ms: u64,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// 剖析配置
#[derive(Debug, Clone)]
pub struct ProfileConfig {
    pub enabled: bool,
    pub sample_interval_ms: u64,
    pub max_samples: usize,
    pub max_duration_ms: u64,
}