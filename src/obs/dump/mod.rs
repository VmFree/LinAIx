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

//! # Dump (状态转储)
//!
//! 提供状态转储能力，与 KV Cache 持久化区分：
//! - KV Cache 持久化: 正常流程，用于恢复
//! - Dump: 异常情况，用于诊断
//!
//! ## 版本
//! 0.1.0

pub mod traits;
pub mod types;

pub use traits::*;
pub use types::*;