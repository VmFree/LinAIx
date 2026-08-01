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

//! # 统一设备接口定义
//!
//! ## Version
//! 0.1.0
//!
//! ## Author
//! VmFree <vmfree@example.com>
//!
//! ## Date
//! 2026-08-01

use crate::device::types::*;
use crate::device::error::DeviceError;

/// 统一设备接口
///
/// 所有设备（GPU、NPU、CPU 等）都实现此接口，提供统一的读写和控制能力。
pub trait Device: Send + Sync {
    // ========== 设备信息 ==========

    /// 获取设备信息
    fn info(&self) -> DeviceInfo;

    /// 获取设备句柄
    fn handle(&self) -> DeviceHandle;

    // ========== 读操作 ==========

    /// 从设备读取数据到缓冲区
    ///
    /// # 参数
    /// - `region`: 设备内存区域
    /// - `offset`: 读取偏移量 (相对于 region.offset)
    /// - `buf`: 目标缓冲区
    ///
    /// # 返回
    /// - `Ok(usize)`: 实际读取的字节数
    fn read(
        &self,
        region: &DeviceMemoryRegion,
        offset: u64,
        buf: &mut [u8],
    ) -> Result<usize, DeviceError>;

    // ========== 写操作 ==========

    /// 从缓冲区写入数据到设备
    ///
    /// # 参数
    /// - `region`: 设备内存区域
    /// - `offset`: 写入偏移量 (相对于 region.offset)
    /// - `buf`: 源缓冲区
    ///
    /// # 返回
    /// - `Ok(usize)`: 实际写入的字节数
    fn write(
        &self,
        region: &DeviceMemoryRegion,
        offset: u64,
        buf: &[u8],
    ) -> Result<usize, DeviceError>;

    // ========== 内存管理 ==========

    /// 在设备上分配内存
    fn alloc(
        &self,
        size: usize,
        flags: AllocFlags,
    ) -> Result<DeviceMemoryRegion, DeviceError>;

    /// 释放设备上的内存
    fn free(&self, region: &DeviceMemoryRegion) -> Result<(), DeviceError>;

    // ========== 控制操作 (可选) ==========

    /// 设备控制命令
    ///
    /// 用于设备特定的操作，如：
    /// - 同步设备缓存
    /// - 查询设备状态
    /// - 设置设备参数
    ///
    /// # 默认实现
    /// 默认返回 `Unsupported` 错误，设备可选择性实现。
    fn control(&self, cmd: ControlCommand) -> Result<ControlResult, DeviceError> {
        Err(DeviceError::UnsupportedControl)
    }

    // ========== 设备状态 ==========

    /// 检查设备是否就绪
    fn is_ready(&self) -> bool {
        matches!(self.info().status, DeviceStatus::Ready)
    }

    /// 获取设备可用内存
    fn available_memory(&self) -> u64 {
        // 默认实现，具体设备可覆盖
        self.info().total_memory
    }
}