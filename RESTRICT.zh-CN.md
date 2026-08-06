## LinAIx 项目编码规则与约束

### 版本
0.1.0

### 一、目录与模块结构

#### 1.1 子系统目录结构

每个子系统遵循统一的目录结构：

```
src/{subsystem}/
├── mod.rs                           # 模块入口，统一导出
├── error.rs                         # 错误类型定义 (如有)
├── {submodule}/                     # 子模块目录
│   ├── mod.rs                       # 子模块入口
│   ├── traits.rs                    # 子模块接口定义
│   └── types.rs                     # 子模块类型定义
└── default.rs                       # 默认实现 (可选)
```

#### 1.2 目录命名规范

- 使用**小写字母**，多个单词用下划线分隔 (snake_case)
- 目录名应简洁且语义明确
- 缩写仅限广为人知的缩写 (如 `ipc`, `obs`)


### 二、子系统清单与职责边界

| # | 子系统 | 目录 | 职责 |
|---|--------|------|------|
| 1 | Device HAL | `device/` | 硬件资源读写抽象 |
| 2 | Model Runtime | `model/` | 模型信息 + 推理执行接口 |
| 3 | Security | `security/` | 认证/授权/审计/加密/注入检测 |
| 4 | Resource Manager | `res_mgr/` | 配额管理 / KV Cache / 资源统计 |
| 5 | Task | `task/` | Task/SubTask 定义和管理 |
| 6 | Scheduler | `scheduler/` | SubTask 调度 |
| 7 | Skill API | `skill/` | Skill 注册/调用/适配 |
| 8 | Agent Runtime | `runtime/` | Agent 执行环境 |
| 9 | Framework | `framework/` | Agent 框架管理 |
| 10 | IPC | `ipc/` | Agent 间通信 |
| 11 | Observability | `obs/` | 指标/追踪/日志/转储/事件 |


### 三、功能唯一性原则

#### 3.1 核心原则

> **一个功能只在一个地方定义和实现，其他地方通过引用使用。**

#### 3.2 类型归属原则

常见类型定义位置如下 (举例)：

| 类型 | 定义位置 |
|------|----------|
| `AgentId`, `UserId`, `TenantId` | `security/auth/types.rs` |
| `TaskId`, `SubTaskId` | `task/{task,subtask}/types.rs` |
| `FrameworkId` | `framework/framework/types.rs` |
| `ResourceType`, `ResourceQuota` | `res_mgr/quota/types.rs` |
| `SkillId` | `skill/id/types.rs` |
| `TraceId`, `SpanId` | `obs/tracing/types.rs` |
| `ChannelId`, `MessageId` | `ipc/{channel,message}/types.rs` |

#### 3.3 禁止行为

- ❌ 在不同模块中重复定义相同或相似的类型
- ❌ 在不同模块中实现相同或相似的功能


### 四、依赖规则

#### 4.1 依赖方向 (自上而下)

```
上层 (scheduler / skill / runtime / framework)
                    │
                    ▼
中层 (res_mgr / task / ipc / obs)
                    │
                    ▼
底层 (device / model / security)
```

#### 4.2 具体依赖规则

| 子系统 | 可依赖 | 不可依赖 |
|--------|--------|----------|
| `device/` | 无 | 任何其他子系统 |
| `model/` | `device`, `security`, `task` | `scheduler`, `res_mgr` |
| `security/` | `task` (仅类型引用) | 其他业务模块 |
| `res_mgr/` | `device`, `model`, `security`, `task` | `scheduler`, `skill`, `runtime` |
| `task/` | `framework` (仅类型引用) | 其他业务模块 |
| `scheduler/` | `task`, `res_mgr`, `model`, `security`, `ipc` | `skill`, `runtime` |
| `skill/` | `task`, `security`, `res_mgr`, `model` | `scheduler`, `runtime` |
| `runtime/` | 所有下层 | 无 |
| `framework/` | `task`, `security`, `scheduler`, `runtime` | 无 |
| `ipc/` | `task`, `security`, `res_mgr`, `scheduler` | `skill`, `runtime` |
| `obs/` | 所有模块 (作为数据源) | 无 |

#### 4.3 循环依赖禁止

- **严格禁止**任何形式的循环依赖
- 如果 A 依赖 B，B 不能再依赖 A
- 可通过引入中间层或接口抽象来打破循环


### 五、编码风格

#### 5.1 文件头

每个 `.rs` 文件必须包含标准的 GPL 许可证头：

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

//! # 模块文档
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

#### 5.2 命名规范

| 类型 | 规范 | 示例 |
|------|------|------|
| 模块名/目录名 | snake_case | `res_mgr`, `kv_cache` |
| 文件名 | snake_case | `traits.rs`, `types.rs` |
| 类型名 (struct/enum) | PascalCase | `ResourceQuota`, `TaskStatus` |
| Trait 名 | PascalCase | `QuotaResource`, `TaskManager` |
| 函数/方法 | snake_case | `check_quota()`, `create_task()` |
| 常量 | SCREAMING_SNAKE_CASE | `MAX_TOKENS`, `DEFAULT_TIMEOUT` |
| 类型别名 | PascalCase | `AgentId`, `TaskId` |

#### 5.3 Trait 命名约定

- 管理类 trait：`{名词}Manager` (如 `TaskManager`)
- 资源类 trait：`{名词}Resource` (如 `QuotaResource`)
- 提供者类 trait：`{名词}Provider` (如 `ModelInfoProvider`)
- 执行器类 trait：`{名词}Executor` (如 `SkillExecutor`)
- 检查器类 trait：`{名词}Checker` (如 `SkillPermissionChecker`)

#### 5.4 模块导出规范

`mod.rs` 中统一导出：

```rust
pub mod traits;
pub mod types;

pub use traits::*;
pub use types::*;
```


### 六、错误处理

#### 6.1 各子系统错误类型

每个子系统定义自己的错误类型，命名为 `{Subsystem}Error`：

| 子系统 | 错误类型 |
|--------|----------|
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

#### 6.2 错误转换

- 使用 `thiserror::Error` 定义错误类型
- 错误转换使用 `#[from]` 宏或 `map_err()`
- 跨模块错误应转换为调用方模块的错误类型


### 七、测试规范

#### 7.1 单元测试

- 单元测试放在每个 `types.rs` 和 `traits.rs` 文件末尾
- 使用 `#[cfg(test)]` 隔离测试代码

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

#### 7.2 集成测试

- 集成测试放在 `tests/` 目录下
- 文件命名：`test_{subsystem}.rs`


### 八、文档规范

#### 8.1 模块文档

每个 `mod.rs` 必须包含：
- 模块功能描述
- 核心概念说明
- 版本、作者、日期

#### 8.2 类型文档

- 每个 `struct` 和 `enum` 必须有文档注释 (`///`)
- 重要成员应有说明

#### 8.3 Trait 文档

- 每个 `trait` 必须有文档注释
- 说明职责和设计意图


### 九、代码生成规范

#### 9.1 创建新模块时

1. 创建目录 `src/{subsystem}/`
2. 创建 `mod.rs`
3. 创建子模块目录 (如 `types/`, `traits/`)
4. 在 `mod.rs` 中统一导出
5. 更新 `src/lib.rs` 添加模块声明

#### 9.2 添加新类型时

1. 确定类型归属
2. 放在该子系统的 `types.rs` 中
3. 添加文档注释
4. 跨模块使用需在 `mod.rs` 中 re-export

#### 9.3 添加新接口 (Trait) 时

1. 确定接口归属
2. 放在该子系统的 `traits.rs` 中
3. 添加文档注释，说明职责
4. 遵循最小接口原则


### 十、快速参考卡片

| 项目 | 规范 |
|------|------|
| 目录/文件命名 | `snake_case` |
| 类型/Trait 命名 | `PascalCase` |
| 函数命名 | `snake_case` |
| 常量命名 | `SCREAMING_SNAKE_CASE` |
| 错误类型 | `{Subsystem}Error` |
| 模块入口 | `mod.rs` |
| 类型定义 | `types.rs` |
| 接口定义 | `traits.rs` |
| 默认实现 | `default.rs` |
| 许可证头 | GPL-2.0-only |
| 文档 | 所有公开 API 必须有文档 |
| 功能唯一性 | 一个功能只定义一次 |
| 循环依赖 | 严格禁止 |

