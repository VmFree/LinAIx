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

//! # Agent 沙箱类型定义
//!
//! ## Version
//! 0.1.0

use serde::{Deserialize, Serialize};

/// 沙箱配置
#[derive(Debug, Clone)]
pub struct SandboxConfig {
    pub enabled: bool,
    pub limits: SandboxLimits,
    pub mount_points: alloc::vec::Vec<MountPoint>,
    pub network: NetworkConfig,
    pub seccomp_profile: Option<alloc::string::String>,
}

/// 沙箱限制
#[derive(Debug, Clone)]
pub struct SandboxLimits {
    pub max_memory_bytes: u64,
    pub max_cpu_time_ms: u64,
    pub max_file_size_bytes: u64,
    pub max_open_files: u32,
    pub max_network_connections: u32,
}

/// 挂载点
#[derive(Debug, Clone)]
pub struct MountPoint {
    pub source: alloc::string::String,
    pub target: alloc::string::String,
    pub readonly: bool,
}

/// 网络配置
#[derive(Debug, Clone)]
pub struct NetworkConfig {
    pub enabled: bool,
    pub allow_host: bool,
    pub allowed_hosts: alloc::vec::Vec<alloc::string::String>,
    pub blocked_ports: alloc::vec::Vec<u32>,
}