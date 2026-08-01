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

pub type KeyId = alloc::string::String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Key {
    pub id: KeyId,
    pub key_type: KeyType,
    pub key_data: alloc::vec::Vec<u8>,
    pub algorithm: CryptoAlgorithm,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    pub metadata: std::collections::HashMap<alloc::string::String, alloc::string::String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum KeyType {
    Symmetric,
    Public,
    Private,
    ApiKey,
    Hmac,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptResult {
    pub ciphertext: alloc::vec::Vec<u8>,
    pub algorithm: CryptoAlgorithm,
    pub key_id: KeyId,
    pub iv: Option<alloc::vec::Vec<u8>>,
    pub tag: Option<alloc::vec::Vec<u8>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HashResult {
    pub hash: alloc::vec::Vec<u8>,
    pub algorithm: CryptoAlgorithm,
}