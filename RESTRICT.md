## LinAIx Coding Rules & Constraints

### Version
0.1.0

### I. Directory & Module Structure

#### 1.1 Subsystem Directory Structure

Each subsystem follows a consistent directory structure:

```
src/{subsystem}/
├── mod.rs                           # Module entry, unified exports
├── error.rs                         # Error type definitions (if needed)
├── {submodule}/                     # Submodule directory
│   ├── mod.rs                       # Submodule entry
│   ├── traits.rs                    # Submodule interface definitions
│   └── types.rs                     # Submodule type definitions
└── default.rs                       # Default implementation (optional)
```

#### 1.2 Directory Naming Convention

- Use **lowercase** letters, separate multiple words with underscores (`snake_case`)
- Directory names should be concise and semantically clear
- Abbreviations allowed only for widely recognized ones (e.g., `ipc`, `obs`)


### II. Subsystem List & Responsibilities

| # | Subsystem | Directory | Responsibility |
|---|-----------|-----------|----------------|
| 1 | Device HAL | `device/` | Hardware resource read/write abstraction |
| 2 | Model Runtime | `model/` | Model info + inference execution interface |
| 3 | Security | `security/` | Authentication/Authorization/Audit/Encryption/Injection Detection |
| 4 | Resource Manager | `res_mgr/` | Quota management / KV Cache / Resource statistics |
| 5 | Task | `task/` | Task/SubTask definition and management |
| 6 | Scheduler | `scheduler/` | SubTask scheduling |
| 7 | Skill API | `skill/` | Skill registration/invocation/adapter |
| 8 | Agent Runtime | `runtime/` | Agent execution environment |
| 9 | Framework | `framework/` | Agent framework management |
| 10 | IPC | `ipc/` | Inter-Agent communication |
| 11 | Observability | `obs/` | Metrics/Tracing/Logging/Dump/Events |


### III. Single Responsibility Principle

#### 3.1 Core Principle

> **One feature is defined and implemented in exactly one place. Other places reference it by import.**

#### 3.2 Type Ownership (Examples)

| Type | Defined In |
|------|------------|
| `AgentId`, `UserId`, `TenantId` | `security/auth/types.rs` |
| `TaskId`, `SubTaskId` | `task/{task,subtask}/types.rs` |
| `FrameworkId` | `framework/framework/types.rs` |
| `ResourceType`, `ResourceQuota` | `res_mgr/quota/types.rs` |
| `SkillId` | `skill/id/types.rs` |
| `TraceId`, `SpanId` | `obs/tracing/types.rs` |
| `ChannelId`, `MessageId` | `ipc/{channel,message}/types.rs` |

#### 3.3 Prohibited Actions

- ❌ Duplicating identical or similar types across different modules
- ❌ Implementing identical or similar functionality across different modules


### IV. Dependency Rules

#### 4.1 Dependency Direction (Top-Down)

```
Upper Layer (scheduler / skill / runtime / framework)
                    │
                    ▼
Middle Layer (res_mgr / task / ipc / obs)
                    │
                    ▼
Lower Layer (device / model / security)
```

#### 4.2 Specific Dependency Rules

| Subsystem | May Depend On | Must NOT Depend On |
|-----------|---------------|-------------------|
| `device/` | None | Any other subsystem |
| `model/` | `device`, `security`, `task` | `scheduler`, `res_mgr` |
| `security/` | `task` (types only) | Other business modules |
| `res_mgr/` | `device`, `model`, `security`, `task` | `scheduler`, `skill`, `runtime` |
| `task/` | `framework` (types only) | Other business modules |
| `scheduler/` | `task`, `res_mgr`, `model`, `security`, `ipc` | `skill`, `runtime` |
| `skill/` | `task`, `security`, `res_mgr`, `model` | `scheduler`, `runtime` |
| `runtime/` | All lower layers | None |
| `framework/` | `task`, `security`, `scheduler`, `runtime` | None |
| `ipc/` | `task`, `security`, `res_mgr`, `scheduler` | `skill`, `runtime` |
| `obs/` | All modules (as data source) | None |

#### 4.3 Circular Dependency Prohibition

- **Strictly prohibited** in any form
- If A depends on B, B MUST NOT depend on A
- Break circular dependencies by introducing intermediate layers or interface abstractions


### V. Code Style

#### 5.1 File Header

Every `.rs` file MUST include the standard GPL license header:

```rust
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

//! # Module Documentation
//!
//! ## Version
//! 0.1.0
//!
//! ## Author
//! author <email>
//!
//! ## Date
//! yyyy-MM-dd
```

#### 5.2 Naming Conventions

| Category | Convention | Example |
|----------|------------|---------|
| Module/Directory name | `snake_case` | `res_mgr`, `kv_cache` |
| File name | `snake_case` | `traits.rs`, `types.rs` |
| Type name (struct/enum) | `PascalCase` | `ResourceQuota`, `TaskStatus` |
| Trait name | `PascalCase` | `QuotaResource`, `TaskManager` |
| Function/Method | `snake_case` | `check_quota()`, `create_task()` |
| Constant | `SCREAMING_SNAKE_CASE` | `MAX_TOKENS`, `DEFAULT_TIMEOUT` |
| Type alias | `PascalCase` | `AgentId`, `TaskId` |

#### 5.3 Trait Naming Conventions

- Manager trait: `{Noun}Manager` (e.g., `TaskManager`)
- Resource trait: `{Noun}Resource` (e.g., `QuotaResource`)
- Provider trait: `{Noun}Provider` (e.g., `ModelInfoProvider`)
- Executor trait: `{Noun}Executor` (e.g., `SkillExecutor`)
- Checker trait: `{Noun}Checker` (e.g., `SkillPermissionChecker`)

#### 5.4 Module Export Convention

In `mod.rs`, export submodules uniformly:

```rust
pub mod traits;
pub mod types;

pub use traits::*;
pub use types::*;
```


### VI. Error Handling

#### 6.1 Subsystem Error Types

Each subsystem defines its own error type named `{Subsystem}Error`:

| Subsystem | Error Type |
|-----------|------------|
| `device/` | `DeviceError` |
| `model/` | `ModelError` |
| `security/` | `SecurityError` |
| `res_mgr/` | `ResourceError` |
| `task/` | `TaskError` |
| `scheduler/` | `SchedulerError` |
| `skill/` | `SkillError` |
| `runtime/` | `RuntimeError` |
| `framework/` | `FrameworkError` |
| `ipc/` | `IpcError` |
| `obs/` | `ObsError` |

#### 6.2 Error Conversion

- Use `thiserror::Error` for error type definitions
- Use `#[from]` macro or `map_err()` for error conversion
- Cross-module errors should be converted to the caller module's error type


### VII. Testing Standards

#### 7.1 Unit Tests

- Place unit tests at the end of each `types.rs` and `traits.rs` file
- Use `#[cfg(test)]` to isolate test code

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_something() {
        // ...
    }
}
```

#### 7.2 Integration Tests

- Place integration tests in the `tests/` directory
- File naming: `test_{subsystem}.rs`


### VIII. Documentation Standards

#### 8.1 Module Documentation

Every `mod.rs` MUST include:
- Module functionality description
- Core concept explanation
- Version, author, date

#### 8.2 Type Documentation

- Every `struct` and `enum` MUST have doc comments (`///`)
- Important fields SHOULD have explanations

#### 8.3 Trait Documentation

- Every `trait` MUST have doc comments
- Explain responsibilities and design intent


### IX. Code Generation Guidelines

#### 9.1 Creating a New Module

1. Create directory `src/{subsystem}/`
2. Create `mod.rs`
3. Create submodule directories (e.g., `types/`, `traits/`)
4. Export in `mod.rs`
5. Update `src/lib.rs` to declare the module

#### 9.2 Adding a New Type

1. Determine type ownership
2. Place in the subsystem's `types.rs`
3. Add doc comments
4. Re-export in `mod.rs` if used across modules

#### 9.3 Adding a New Trait

1. Determine trait ownership
2. Place in the subsystem's `traits.rs`
3. Add doc comments explaining responsibility
4. Follow the minimal interface principle


### X. Quick Reference Card

| Item | Convention |
|------|------------|
| Directory/File naming | `snake_case` |
| Type/Trait naming | `PascalCase` |
| Function naming | `snake_case` |
| Constant naming | `SCREAMING_SNAKE_CASE` |
| Error type | `{Subsystem}Error` |
| Module entry | `mod.rs` |
| Type definitions | `types.rs` |
| Trait definitions | `traits.rs` |
| Default implementation | `default.rs` |
| License header | GPL-2.0-only |
| Documentation | All public APIs MUST have docs |
| Single responsibility | One feature defined once |
| Circular dependencies | Strictly prohibited |