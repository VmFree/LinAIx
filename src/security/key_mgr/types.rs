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

//! # 密钥管理相关类型
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

// KeyId 从 security::types 导入
use crate::security::types::KeyId;

/// 密钥类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum KeyType {
    Symmetric,
    Public,
    Private,
    ApiKey,
    Hmac,
}

/// 密钥信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Key {
    pub id: KeyId,
    pub key_type: KeyType,
    pub key_data: alloc::vec::Vec<u8>,
    pub algorithm: crate::security::crypto::types::CryptoAlgorithm,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    pub metadata: std::collections::HashMap<alloc::string::String, alloc::string::String>,
}