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

//! # 安全错误类型
//!
//! ## Version
//! 0.1.0
//!
//! ## Author
//! VmFree <vmfree@example.com>
//!
//! ## Date
//! 2026-08-01

use thiserror::Error;

#[derive(Debug, Error)]
pub enum SecurityError {
    // ===== 认证错误 =====
    #[error("Authentication failed: {0}")]
    AuthFailed(alloc::string::String),

    #[error("Invalid credentials: {0}")]
    InvalidCredentials(alloc::string::String),

    #[error("Token expired")]
    TokenExpired,

    #[error("Token invalid")]
    TokenInvalid,

    #[error("Token revoked")]
    TokenRevoked,

    // ===== 授权错误 =====
    #[error("Permission denied: {0}")]
    PermissionDenied(alloc::string::String),

    #[error("Policy not found: {0}")]
    PolicyNotFound(alloc::string::String),

    #[error("Policy conflict: {0}")]
    PolicyConflict(alloc::string::String),

    #[error("Resource ownership mismatch: resource owned by {owner}, attempted by {actor}")]
    OwnershipMismatch { owner: alloc::string::String, actor: alloc::string::String },

    // ===== 审计错误 =====
    #[error("Audit log write failed: {0}")]
    AuditWriteFailed(alloc::string::String),

    #[error("Audit query failed: {0}")]
    AuditQueryFailed(alloc::string::String),

    #[error("Audit export failed: {0}")]
    AuditExportFailed(alloc::string::String),

    // ===== 加密错误 =====
    #[error("Encryption failed: {0}")]
    EncryptionFailed(alloc::string::String),

    #[error("Decryption failed: {0}")]
    DecryptionFailed(alloc::string::String),

    #[error("Hashing failed: {0}")]
    HashFailed(alloc::string::String),

    // ===== 密钥管理错误 =====
    #[error("Key not found: {0}")]
    KeyNotFound(KeyId),

    #[error("Key already exists: {0}")]
    KeyAlreadyExists(KeyId),

    #[error("Key rotation failed: {0}")]
    KeyRotationFailed(alloc::string::String),

    #[error("Invalid key: {0}")]
    InvalidKey(alloc::string::String),

    // ===== 注入检测错误 =====
    #[error("Injection detected: {0}")]
    InjectionDetected(alloc::string::String),

    #[error("Input too large: {0}")]
    InputTooLarge(usize),

    // ===== 通用错误 =====
    #[error("Internal security error: {0}")]
    Internal(alloc::string::String),

    #[error("Configuration error: {0}")]
    ConfigError(alloc::string::String),

    #[error("Unsupported operation: {0}")]
    Unsupported(alloc::string::String),
}