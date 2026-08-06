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

//! # 框架管理接口
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
use crate::framework::error::FrameworkError;
use crate::framework::runtime::RuntimeHandle;

/// 框架管理接口
///
/// 职责：框架的安装、卸载、查询、默认框架管理
pub trait FrameworkManager: Send + Sync {
    // ===== 框架安装/卸载 =====

    /// 安装框架
    fn install(
        &mut self,
        framework_type: FrameworkType,
        version: alloc::string::String,
        config: FrameworkConfig,
    ) -> Result<FrameworkId, FrameworkError>;

    /// 卸载框架
    fn uninstall(&mut self, framework_id: &FrameworkId) -> Result<(), FrameworkError>;

    /// 更新框架
    fn update(
        &mut self,
        framework_id: &FrameworkId,
        config: FrameworkConfig,
    ) -> Result<(), FrameworkError>;

    // ===== 框架查询 =====

    /// 列出所有已安装框架
    fn list(&self) -> alloc::vec::Vec<Framework>;

    /// 获取指定框架
    fn get(&self, framework_id: &FrameworkId) -> Result<Framework, FrameworkError>;

    /// 按类型获取框架
    fn get_by_type(&self, framework_type: &FrameworkType) -> alloc::vec::Vec<Framework>;

    /// 获取默认框架
    fn get_default(&self) -> Option<Framework>;

    /// 设置默认框架
    fn set_default(&mut self, framework_id: &FrameworkId) -> Result<(), FrameworkError>;

    /// 检查框架是否已安装
    fn is_installed(&self, framework_id: &FrameworkId) -> bool;
}