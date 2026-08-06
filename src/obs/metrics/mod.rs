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

//! # Metrics (指标采集)
//!
//! 通用指标采集框架，从 resource/stats 和 model/metrics 等模块拉取数据。
//!
//! ## 版本
//! 0.1.0

pub mod traits;
pub mod types;

pub use traits::*;
pub use types::*;