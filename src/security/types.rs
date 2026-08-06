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

//! # 跨模块共享类型
//!
//! 定义被多个 security 子模块引用的共享类型。
//!
//! ## Version
//! 0.1.0
//!
//! ## Author
//! VmFree <vmfree@example.com>
//!
//! ## Date
//! 2026-07-31

/// 密钥标识 (跨模块共享)
///
/// 被以下模块引用：
/// - `crypto`: 加密/解密操作
/// - `key_management`: 密钥生命周期管理
/// - `resource`: 资源管理 (加密存储)
pub type KeyId = alloc::string::String;

/// 租户标识 (跨模块共享)
pub type TenantId = alloc::string::String;
