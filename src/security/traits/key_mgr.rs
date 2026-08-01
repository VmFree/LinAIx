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

//! # 密钥管理接口
//!
//! ## Version
//! 0.1.0
//!
//! ## Author
//! VmFree <vmfree@example.com>
//!
//! ## Date
//! 2026-07-31

use crate::security::types::*;
use crate::security::error::SecurityError;

/// 密钥管理接口
///
/// 职责：密钥的注册、查询、轮换、删除
pub trait KeyManagement: Send + Sync {
    /// 注册新密钥
    fn register(&mut self, key: Key) -> Result<KeyId, SecurityError>;

    /// 获取密钥信息 (不返回密钥数据)
    fn get_info(&self, key_id: &KeyId) -> Result<Key, SecurityError>;

    /// 轮换密钥 (生成新密钥并替换旧密钥)
    fn rotate(&mut self, key_id: &KeyId) -> Result<KeyId, SecurityError>;

    /// 删除密钥
    fn delete(&mut self, key_id: &KeyId) -> Result<(), SecurityError>;

    /// 列出所有密钥 ID
    fn list(&self) -> alloc::vec::Vec<KeyId>;

    /// 检查密钥是否存在
    fn exists(&self, key_id: &KeyId) -> bool {
        self.get_info(key_id).is_ok()
    }
}