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

//! # 加密接口
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

/// 加密接口
///
/// 职责：加密/解密/哈希/HMAC
///
/// # 注意
/// 密钥管理由 [`KeyManagement`] trait 负责
pub trait Crypto: Send + Sync {
    /// 加密数据
    fn encrypt(&self, data: &[u8], key_id: &KeyId) -> Result<EncryptResult, SecurityError>;

    /// 解密数据
    fn decrypt(&self, ciphertext: &[u8], key_id: &KeyId) -> Result<alloc::vec::Vec<u8>, SecurityError>;

    /// 计算哈希
    fn hash(&self, data: &[u8], algorithm: CryptoAlgorithm) -> Result<HashResult, SecurityError>;

    /// 验证哈希
    fn verify_hash(&self, data: &[u8], hash: &[u8]) -> Result<bool, SecurityError>;

    /// 计算 HMAC
    fn hmac(&self, data: &[u8], key_id: &KeyId) -> Result<HashResult, SecurityError>;

    /// 验证 HMAC
    fn verify_hmac(&self, data: &[u8], hmac: &[u8], key_id: &KeyId) -> Result<bool, SecurityError>;
}