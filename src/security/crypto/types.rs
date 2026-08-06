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

//! # 加密相关类型
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

// KeyId 从 security::types 导入，不在本文件定义
use crate::security::types::KeyId;

/// 加密算法
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CryptoAlgorithm {
    Aes256Gcm,
    Aes256Cbc,
    ChaCha20Poly1305,
    Rsa2048,
    Rsa4096,
    EcdsaP256,
    EcdsaP384,
    HmacSha256,
    HmacSha512,
    Sha256,
    Sha512,
}

/// 加密结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptResult {
    pub ciphertext: alloc::vec::Vec<u8>,
    pub algorithm: CryptoAlgorithm,
    pub key_id: KeyId,
    pub iv: Option<alloc::vec::Vec<u8>>,
    pub tag: Option<alloc::vec::Vec<u8>>,
}

/// 哈希结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HashResult {
    pub hash: alloc::vec::Vec<u8>,
    pub algorithm: CryptoAlgorithm,
}