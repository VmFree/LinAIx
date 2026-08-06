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

//! # Dump 类型定义
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

use crate::task::task::TaskId;
use crate::task::subtask::SubTaskId;
use crate::runtime::agent::AgentId;

/// 转储类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DumpType {
    /// Agent 崩溃转储 (核心)
    Core,

    /// 任务状态快照
    State,

    /// KV Cache 转储
    KVCache,

    /// 调用栈
    Stack,
}

/// 转储元数据
#[derive(Debug, Clone)]
pub struct DumpMetadata {
    pub dump_type: DumpType,
    pub task_id: Option<TaskId>,
    pub subtask_id: Option<SubTaskId>,
    pub agent_id: Option<AgentId>,
    pub reason: alloc::string::String,
    pub size_bytes: u64,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub path: alloc::string::String,
}

/// 转储内容 (用于恢复)
#[derive(Debug, Clone)]
pub struct DumpContent {
    pub metadata: DumpMetadata,
    pub data: alloc::vec::Vec<u8>,
    pub checksum: alloc::string::String,  // SHA256
}

/// 转储配置
#[derive(Debug, Clone)]
pub struct DumpConfig {
    pub enabled: bool,
    pub max_dumps: usize,
    pub max_size_bytes: u64,
    pub dump_dir: alloc::string::String,
    pub auto_dump_on_crash: bool,
}