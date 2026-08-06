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

//! # 框架注册中心
//!
//! 负责框架的注册、发现和索引。
//!
//! ## 与 FrameworkManager 的区别
//!
//! | 组件 | 职责 |
//! |------|------|
//! | FrameworkManager | 框架的生命周期管理 (安装/卸载/更新) |
//! | FrameworkRegistry | 框架的注册与发现 (索引/查询/匹配) |
//!
//! ## 版本
//! 0.1.0
//!
//! ## Author
//! VmFree <vmfree@example.com>
//!
//! ## Date
//! 2026-08-01

use super::types::*;
use crate::framework::error::FrameworkError;

/// 框架注册中心
///
/// 职责：框架的注册、发现、能力匹配
pub trait FrameworkRegistry: Send + Sync {
    // ===== 注册 =====

    /// 注册框架到注册中心
    fn register(&mut self, framework: Framework) -> Result<(), FrameworkError>;

    /// 从注册中心注销框架
    fn unregister(&mut self, framework_id: &FrameworkId) -> Result<(), FrameworkError>;

    // ===== 发现 =====

    /// 查找框架
    fn lookup(&self, framework_id: &FrameworkId) -> Option<Framework>;

    /// 按类型查找
    fn lookup_by_type(&self, framework_type: &FrameworkType) -> alloc::vec::Vec<Framework>;

    /// 按能力查找
    fn lookup_by_capability(&self, capability: FrameworkCapability) -> alloc::vec::Vec<Framework>;

    /// 按名称搜索
    fn search(&self, query: &str) -> alloc::vec::Vec<Framework>;

    /// 列出所有已注册框架
    fn list(&self) -> alloc::vec::Vec<Framework>;

    // ===== 匹配 =====

    /// 查找支持指定任务的框架
    ///
    /// 根据任务描述匹配最合适的框架
    fn find_best_match(&self, task_description: &str) -> Option<Framework>;

    /// 检查框架是否支持特定能力
    fn supports_capability(&self, framework_id: &FrameworkId, capability: FrameworkCapability) -> bool;

    /// 获取推荐的默认框架
    fn get_recommended(&self) -> Option<Framework>;
}