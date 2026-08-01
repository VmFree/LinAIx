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

//! # 设备相关类型定义
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

/// 设备唯一标识
pub type DeviceId = u32;

/// 设备句柄
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DeviceHandle {
    pub device_id: DeviceId,
    pub name: alloc::string::String,
}

/// 设备信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    /// 设备 ID
    pub device_id: DeviceId,

    /// 设备名称 (如 "NVIDIA A100", "AMD MI300X")
    pub name: alloc::string::String,

    /// 设备类型
    pub device_type: DeviceType,

    /// 总内存大小 (字节)
    pub total_memory: u64,

    /// 厂商
    pub vendor: alloc::string::String,

    /// 设备驱动版本
    pub driver_version: alloc::string::String,

    /// 设备能力标志
    pub capabilities: DeviceCapabilities,

    /// 设备状态
    pub status: DeviceStatus,
}

/// 设备类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeviceType {
    Gpu,      // GPU
    Npu,      // NPU (神经网络处理器)
    Cpu,      // CPU
    Accelerator, // 通用加速器
}

/// 设备能力
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeviceCapabilities {
    /// 是否支持流式输出
    pub streaming: bool,

    /// 是否支持批量推理
    pub batch_inference: bool,

    /// 是否支持 KV Cache 持久化
    pub kv_cache_persistence: bool,

    /// 是否支持数据压缩
    pub compression: bool,

    /// 最大上下文窗口大小
    pub max_context_window: usize,

    /// 支持的精度
    pub supported_precisions: alloc::vec::Vec<Precision>,
}

/// 精度类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Precision {
    F32,
    F16,
    BF16,
    F8,
    INT8,
    INT4,
}

/// 设备状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeviceStatus {
    /// 就绪，可用
    Ready,

    /// 繁忙
    Busy,

    /// 出错
    Error,

    /// 离线/不可用
    Offline,
}

/// 设备内存区域
#[derive(Debug, Clone)]
pub struct DeviceMemoryRegion {
    /// 内存区域偏移
    pub offset: u64,

    /// 内存区域大小
    pub size: usize,

    /// 设备句柄
    pub handle: DeviceHandle,
}

/// 分配标志
#[derive(Debug, Clone, Default)]
pub struct AllocFlags {
    /// 是否固定内存 (不可换出)
    pub pinned: bool,

    /// 是否清零
    pub zeroed: bool,
}

/// 控制命令参数
#[derive(Debug, Clone)]
pub struct ControlCommand {
    /// 命令 ID
    pub cmd_id: u32,

    /// 命令参数
    pub args: alloc::vec::Vec<u8>,
}

/// 控制命令结果
#[derive(Debug, Clone)]
pub struct ControlResult {
    /// 命令执行状态
    pub success: bool,

    /// 返回数据
    pub data: alloc::vec::Vec<u8>,

    /// 错误信息 (如果有)
    pub error_message: Option<alloc::string::String>,
}