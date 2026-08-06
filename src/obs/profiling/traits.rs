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

//! # Profiling 接口
//!
//! ## Version
//! 0.1.0
//!
//! ## Author
//! VmFree <vmfree@example.com>
//!
//! ## Date
//! 2026-08-01

use super::types::*;
use crate::obs::error::ObsError;

/// 性能剖析器接口
pub trait Profiler: Send + Sync {
    /// 开始剖析
    fn start(
        &self,
        profile_type: ProfileType,
        target: alloc::string::String,
        duration_ms: Option<u64>,
    ) -> Result<ProfileId, ObsError>;

    /// 停止剖析
    fn stop(&self, profile_id: &ProfileId) -> Result<ProfileData, ObsError>;

    /// 获取剖析数据
    fn get_data(&self, profile_id: &ProfileId) -> Result<ProfileData, ObsError>;

    /// 列出所有剖析
    fn list(&self) -> alloc::vec::Vec<ProfileId>;

    /// 获取剖析配置
    fn config(&self) -> ProfileConfig;

    /// 更新剖析配置
    fn update_config(&mut self, config: ProfileConfig) -> Result<(), ObsError>;
}

/// 剖析 ID
pub type ProfileId = alloc::string::String;