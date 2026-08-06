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

//! # Skill ID 管理
//!
//! ## Skill ID 格式
//! `{framework}/{namespace}/{name}@{version}`
//!
//! 示例:
//! - `linAIx/system/kv_cache_read@v1`
//! - `langchain/search/web@v2.1.0`
//!
//! ## 版本
//! 0.1.0

pub mod types;
pub mod traits;

pub use types::*;
pub use traits::*;