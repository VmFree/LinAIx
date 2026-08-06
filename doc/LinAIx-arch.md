## LinAIx Project Documentation


### I. Overall Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────────────────────────────────────┐
│                                          Application Layer                                      │
│                              User Tasks / Agent Applications / Business Logic                   │
└─────────────────────────────────────────────────────────────────────────────────────────────────┘
                                              │
                                              ▼
┌─────────────────────────────────────────────────────────────────────────────────────────────────┐
│                                       Agent Framework Layer                                     │
│                                                                                                 │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────────────┐    │
│  │   LangChain     │  │    CrewAI       │  │    AutoGen      │  │   Other Frameworks       │    │
│  │   (Chain/Graph) │  │   (Agent/Task)  │  │   (Conversation)│  │   (Custom Adapter)       │    │
│  └────────┬────────┘  └────────┬────────┘  └────────┬────────┘  └────────────┬────────────┘    │
│           │                    │                    │                        │                 │
│           └────────────────────┴────────────────────┴────────────────────────┘                 │
│                                              │                                                  │
│                                    Framework Manager                                           │
│                                   (Install/Select/Runtime)                                     │
└─────────────────────────────────────────────────────────────────────────────────────────────────┘
                                              │
                                              ▼
┌─────────────────────────────────────────────────────────────────────────────────────────────────┐
│                                        LinAIx Kernel Layer                                      │
│                                                                                                 │
│  ┌─────────────────────────────────────────────────────────────────────────────────────────┐   │
│  │                                    User Space (L5-L4)                                    │   │
│  │                                                                                         │   │
│  │  ┌─────────────────────────────────────────────────────────────────────────────────┐   │   │
│  │  │  L5 Agent Runtime                                                               │   │   │
│  │  │  Execution Environment / Sandbox / Lifecycle / System Call Bridge               │   │   │
│  │  └─────────────────────────────────────────────────────────────────────────────────┘   │   │
│  │                                         │                                               │   │
│  │  ┌─────────────────────────────────────────────────────────────────────────────────┐   │   │
│  │  │  L4 Skill API                                                                  │   │   │
│  │  │  Skill Registration/Invocation/Adapter / Permission Check / Audit               │   │   │
│  │  └─────────────────────────────────────────────────────────────────────────────────┘   │   │
│  └─────────────────────────────────────────────────────────────────────────────────────────┘   │
│                                              │                                                  │
│  ┌─────────────────────────────────────────────────────────────────────────────────────────┐   │
│  │                                    Kernel Space (L3-L1)                                  │   │
│  │                                                                                         │   │
│  │  ┌─────────────────────────────────────────────────────────────────────────────────┐   │   │
│  │  │  L3 Scheduler                                                                   │   │   │
│  │  │  Runqueue / Waitqueue / Priority / Preemption / SchedClass                      │   │   │
│  │  └─────────────────────────────────────────────────────────────────────────────────┘   │   │
│  │                                         │                                               │   │
│  │  ┌─────────────────────────────────────────────────────────────────────────────────┐   │   │
│  │  │  L2 Resource Manager                                                            │   │   │
│  │  │  Model Resources / KV Cache / Quota Management / Resource Stats                │   │   │
│  │  └─────────────────────────────────────────────────────────────────────────────────┘   │   │
│  │                                         │                                               │   │
│  │  ┌─────────────────────────────────────────────────────────────────────────────────┐   │   │
│  │  │  L1 Hardware Abstraction Layer (HAL)                                            │   │   │
│  │  │  Device Read/Write / Memory Allocation / Inference Execution                   │   │   │
│  │  └─────────────────────────────────────────────────────────────────────────────────┘   │   │
│  └─────────────────────────────────────────────────────────────────────────────────────────┘   │
│                                                                                                 │
│  ┌─────────────────────────────────────────────────────────────────────────────────────────┐   │
│  │                                     Cross-Cutting Services                               │   │
│  │                                                                                         │   │
│  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐                 │   │
│  │  │   Security   │  │     IPC      │  │      Obs     │  │     Task     │                 │   │
│  │  │   Auth/      │  │   Agent-to-  │  │   Metrics/   │  │  Task/       │                 │   │
│  │  │   Authz/     │  │   Agent      │  │   Tracing/   │  │  SubTask     │                 │   │
│  │  │   Audit/     │  │   Comm       │  │   Logging/   │  │  Management  │                 │   │
│  │  │   Crypto     │  │              │  │   Events     │  │              │                 │   │
│  │  └──────────────┘  └──────────────┘  └──────────────┘  └──────────────┘                 │   │
│  └─────────────────────────────────────────────────────────────────────────────────────────┘   │
│                                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────────────────────┘
                                              │
                                              ▼
┌─────────────────────────────────────────────────────────────────────────────────────────────────┐
│                                       Infrastructure Layer                                      │
│                                                                                                 │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────────────┐    │
│  │   GPU (CUDA/    │  │    CPU/Memory   │  │      Disk       │  │      Network            │    │
│  │    ROCm)        │  │                 │  │                 │  │   (Remote Inference)    │    │
│  └─────────────────┘  └─────────────────┘  └─────────────────┘  └─────────────────────────┘    │
│                                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────────────────────┘
```


### II. Subsystem Dependency Diagram

```
┌─────────────────────────────────────────────────────────────────────────────────────────────────┐
│                                   framework (L5)                                               │
│                              Agent Framework Management                                        │
└─────────────────────────────────────────────────────────────────────────────────────────────────┘
                                              │
                                              ▼
┌─────────────────────────────────────────────────────────────────────────────────────────────────┐
│                                   runtime (L5)                                                 │
│                              Agent Execution Environment                                       │
└─────────────────────────────────────────────────────────────────────────────────────────────────┘
                                              │
                                              ▼
┌─────────────────────────────────────────────────────────────────────────────────────────────────┐
│                                   skill (L4)                                                   │
│                              Skill Registration/Invocation/Adapter                             │
└─────────────────────────────────────────────────────────────────────────────────────────────────┘
                                              │
                                              ▼
┌─────────────────────────────────────────────────────────────────────────────────────────────────┐
│                                   scheduler (L3)                                               │
│                              SubTask Scheduling                                                │
└─────────────────────────────────────────────────────────────────────────────────────────────────┘
                                              │
                                              ▼
┌─────────────────────────────────────────────────────────────────────────────────────────────────┐
│                                   resource (L2)                                                │
│                              Quota / KV Cache / Resource Stats                                 │
└─────────────────────────────────────────────────────────────────────────────────────────────────┘
                                              │
                                              ▼
┌─────────────────────────────────────────────────────────────────────────────────────────────────┐
│                                   model (L1)                                                   │
│                              Model Info + Inference Execution                                  │
└─────────────────────────────────────────────────────────────────────────────────────────────────┘
                                              │
                                              ▼
┌─────────────────────────────────────────────────────────────────────────────────────────────────┐
│                                   device (L1)                                                  │
│                              Hardware Resource Read/Write Abstraction                          │
└─────────────────────────────────────────────────────────────────────────────────────────────────┘
```

### Cross-Cutting Services (Dependent by All Layers)

```
┌─────────────────────────────────────────────────────────────────────────────────────────────────┐
│                                    Cross-Cutting Services                                       │
│                                                                                                 │
│  ┌─────────────────────────────────────────────────────────────────────────────────────────┐   │
│  │                                   security                                              │   │
│  │                         Auth / Authz / Audit / Crypto                                   │   │
│  └─────────────────────────────────────────────────────────────────────────────────────────┘   │
│                                                                                                 │
│  ┌─────────────────────────────────────────────────────────────────────────────────────────┐   │
│  │                                   task                                                   │   │
│  │                              Task / SubTask Management                                   │   │
│  └─────────────────────────────────────────────────────────────────────────────────────────┘   │
│                                                                                                 │
│  ┌─────────────────────────────────────────────────────────────────────────────────────────┐   │
│  │                                    ipc                                                   │   │
│  │                               Agent-to-Agent Communication                               │   │
│  └─────────────────────────────────────────────────────────────────────────────────────────┘   │
│                                                                                                 │
│  ┌─────────────────────────────────────────────────────────────────────────────────────────┐   │
│  │                                    obs                                                   │   │
│  │                          Metrics / Tracing / Logging / Events                            │   │
│  └─────────────────────────────────────────────────────────────────────────────────────────┘   │
│                                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────────────────────┘
```


### III. Integration with Agent Frameworks

#### III.1 Integration Architecture

```
┌─────────────────────────────────────────────────────────────────────────────────────────────────┐
│                                    Agent Framework Integration Architecture                      │
│                                                                                                 │
│  ┌─────────────────────────────────────────────────────────────────────────────────────────┐   │
│  │                              Agent Framework Process                                     │   │
│  │                                                                                         │   │
│  │  ┌─────────────────────────────────────────────────────────────────────────────────┐   │   │
│  │  │  Framework Native Capabilities                                                   │   │   │
│  │  │  • Task Decomposition                                                             │   │   │
│  │  │  • Agent Role Definition (Role/Persona)                                         │   │   │
│  │  │  • Workflow Orchestration (DAG/Graph)                                           │   │   │
│  │  │  • Collaboration Patterns (Master-Worker/Pipeline/Voting)                       │   │   │
│  │  └─────────────────────────────────────────────────────────────────────────────────┘   │   │
│  │                                         │                                               │   │
│  │                                         ▼                                               │   │
│  │  ┌─────────────────────────────────────────────────────────────────────────────────┐   │   │
│  │  │                              LinAIx Adapter Layer                                │   │   │
│  │  │                                                                                 │   │   │
│  │  │  1. SubTask Detection  →  Detect framework subtask creation                     │   │   │
│  │  │                           →  Call TaskManager.create_subtask()                 │   │   │
│  │  │  2. Skill Interception  →  Intercept framework tool calls                      │   │   │
│  │  │                           →  Call SkillAPI.invoke()                            │   │   │
│  │  │  3. Model Interception  →  Intercept framework model calls                     │   │   │
│  │  │                           →  Attach TaskId/SubTaskId → Call HAL               │   │   │
│  │  │  4. Result Adaptation   →  Convert LinAIx results to framework native format   │   │   │
│  │  │  5. Error Handling      →  Convert LinAIx errors to framework native errors   │   │   │
│  │  └─────────────────────────────────────────────────────────────────────────────────┘   │   │
│  │                                         │                                               │   │
│  └─────────────────────────────────────────────────────────────────────────────────────────┘   │
│                                              │                                                  │
│                                              ▼                                                  │
│                              LinAIx Kernel (via Framework Module)                              │
│                                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────────────────────┘
```

#### III.2 Integration Points Reference

| Framework Behavior | LinAIx Interception Point | LinAIx Module Used | Description |
|--------------------|---------------------------|-------------------|-------------|
| Framework creates subtask | Adapter detection | `task` | Adapter calls `SubTaskManager::create_subtask()` |
| Framework calls tool (Skill) | L4 Skill API | `skill`, `security`, `resource` | Intercepted through permission/quota/audit pipeline |
| Framework calls model (internal inference) | L1 HAL | `hal`, `resource` | Attach TaskId/SubTaskId, enforce quota check |
| Framework requires permission check | Security module | `security/auth`, `security/authorization` | Unified permission verification |
| Framework requires resource quota | Resource module | `resource/quota` | Token/VRAM/Memory quota checking |
| Framework requires audit log | Security/Audit | `security/audit` | Unified audit logging |
| Framework requires observability | Obs module | `obs/metrics`, `obs/tracing` | Metrics collection and distributed tracing |
| Framework-to-Framework communication | IPC module | `ipc/channel`, `ipc/message` | Agent-to-Agent message passing |

#### III.3 Adapter Interface List

| Interface | Responsibility |
|-----------|----------------|
| `FrameworkAdapter::adapt_to_framework()` | Wrap LinAIx Skill as framework native tool (Reverse Adaptation) |
| `FrameworkAdapter::adapt_to_linAIx_skill()` | Convert framework tool call to LinAIx Skill call (Forward Adaptation) |
| `FrameworkAdapter::intercept_model_call()` | Intercept framework model calls, attach quota tracking info |
| `FrameworkAdapter::detect_and_report_subtask()` | Detect framework subtask creation, report to TaskManager |
| `FrameworkAdapter::handle_quota_exceeded()` | Handle quota exceeded errors |


### IV. Subsystem Details

#### IV.1 Device HAL (`device/`)

| Item | Content |
|------|---------|
| **Core Positioning** | Hardware resource read/write abstraction, shielding differences across hardware (GPU/NPU/CPU) |
| **Design Principles** | Minimal interface (read/write/alloc/free/control only) |
| **Core Responsibilities** | Device info query / Memory read/write / Memory alloc/free / Optional control operations |

**Core Concepts**:

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                               Device                                       │
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │  DeviceInfo                                                          │   │
│  │  • device_id  • device_type  • total_memory  • vendor              │   │
│  │  • capabilities (streaming, batch_inference, kv_cache_persistence) │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                    │                                       │
│                                    ▼                                       │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │  Device Operations (Device trait)                                   │   │
│  │  • read()    • write()    • alloc()    • free()    • control()     │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                    │                                       │
│                                    ▼                                       │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │  Concrete Device Implementations                                     │   │
│  │  • CudaDevice  • RocmDevice  • CpuDevice  • CustomDevice           │   │
│  │  │                                                                   │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

| Item | Content |
|------|---------|
| **Core Interfaces** | `Device` (unified device interface) |
| **Type Definitions** | `DeviceId`, `DeviceType`, `DeviceInfo`, `DeviceCapabilities`, `DeviceStatus`, `DeviceMemoryRegion`, `AllocFlags`, `ControlCommand` |
| **Dependencies** | None |


#### IV.2 Model Runtime (`model/`)

| Item | Content |
|------|---------|
| **Core Positioning** | Model info query + unified inference execution interface |
| **Design Principles** | Does not handle model load/unload/lifecycle (handled by upper layers) |
| **Core Responsibilities** | Model metadata query / Token estimation / Inference execution (sync/async/stream/batch) |

**Core Concepts**:

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                             Model Runtime                                   │
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │  ModelInfo                                                           │   │
│  │  • model_id  • name  • context_window  • parameter_count           │   │
│  │  • capabilities  • format  • quantization                          │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                    │                                       │
│                                    ▼                                       │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │  ModelInfoProvider                                                  │   │
│  │  • get_model_info()  • list_models()  • estimate_tokens()         │   │
│  │  • get_tokenizer_info()  • find_by_capability()                   │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                    │                                       │
│                                    ▼                                       │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │  ModelExecutor                                                      │   │
│  │  • infer()  • infer_async()  • infer_stream()  • infer_batch()    │   │
│  │  • get_metrics()  • is_ready()  • list_backends()                 │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

**Concept Relationships**:

```
┌──────────────────┐                    ┌──────────────────┐
│   ModelInfo      │                    │   Backend        │
│   (Static Meta)  │                    │   (Inference     │
└────────┬─────────┘                    │    Engine)       │
         │                              └────────┬─────────┘
         ▼                                       │
┌──────────────────┐                             │
│ ModelInfoProvider│── Query ────────────────────│
│ (Info Query)     │                             │
└──────────────────┘                             │
                                                 │
┌──────────────────┐                             │
│  InferenceRequest│── Execute ──────────────────│
│  (Request)       │                             │
└────────┬─────────┘                             │
         │                                       │
         ▼                                       ▼
┌──────────────────┐                    ┌──────────────────┐
│  InferenceResult │                    │   BackendInfo    │
│  (Result)        │                    │   (Backend Info) │
└──────────────────┘                    └──────────────────┘
```

| Item | Content |
|------|---------|
| **Core Interfaces** | `ModelInfoProvider` (info query), `ModelExecutor` (inference execution) |
| **Type Definitions** | `ModelId`, `ModelInfo`, `ModelCapability`, `Precision`, `InferenceRequest`, `InferenceResult`, `BackendType`, `ModelMetrics` |
| **Dependencies** | `device`, `security`, `task` (type references only) |


#### IV.3 Security (`security/`)

| Item | Content |
|------|---------|
| **Core Positioning** | Foundation of LinAIx security system, independent of other subsystems |
| **Design Principles** | Zero Trust / Default Deny / Defense in Depth |
| **Core Responsibilities** | Identity Authentication / Authorization / Audit Logging / Encryption / Key Management / Prompt Injection Detection |

**Core Concepts**:

```
┌─────────────────────────────────────────────────────────────────────────────────────────────────┐
│                                          Security                                              │
│                                                                                                 │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────────────┐    │
│  │    Account      │  │      Auth       │  │  Authorization  │  │         Audit           │    │
│  │   Account       │  │   Authentication│  │   Authorization │  │   Audit                 │    │
│  │   Management    │  │   AuthToken     │  │   Permission    │  │   AuditEvent            │    │
│  │   Account       │  │   AuthContext   │  │   Policy        │  │   AuditFilter           │    │
│  │   Group         │  │                 │  │                 │  │                         │    │
│  └─────────────────┘  └─────────────────┘  └─────────────────┘  └─────────────────────────┘    │
│                                                                                                 │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────────────────────────────────────┐ │
│  │     Crypto      │  │  KeyManagement  │  │                 Injection                       │ │
│  │   Crypto        │  │   KeyManagement │  │               Injection Detection               │ │
│  │   EncryptResult │  │   Key           │  │   InjectionDetectionResult                     │ │
│  │   HashResult    │  │   KeyId         │  │   RiskLevel                                    │ │
│  └─────────────────┘  └─────────────────┘  └─────────────────────────────────────────────────┘ │
│                                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────────────────────┘
```

**Concept Relationships**:

```
┌──────────────────┐         ┌──────────────────┐
│   Credentials    │         │    Account       │
│   (Credential)   │────────▶│    (Account)     │
└──────────────────┘         └────────┬─────────┘
         │                            │
         ▼                            ▼
┌──────────────────┐         ┌──────────────────┐
│   AuthToken      │         │    Group         │
│   (Auth Token)   │         │    (Group)       │
└────────┬─────────┘         └──────────────────┘
         │
         ▼
┌──────────────────┐
│   AuthContext    │◀────────┐
│   (Auth Context) │         │
└────────┬─────────┘         │
         │                   │
         ▼                   │
┌──────────────────┐         │
│   Permission     │─────────┘
│   (Permission)   │
└────────┬─────────┘
         │
         ▼
┌──────────────────┐
│   AuditEvent     │
│   (Audit Event)  │
└──────────────────┘
```

**Core Interfaces**:

| Submodule | Interface | Responsibility |
|-----------|-----------|----------------|
| Account | `AccountProvider` | Account data source abstraction |
| Auth | `Authentication` | Identity authentication and Token management |
| Authorization | `Authorization` | Permission checking and policy management |
| Audit | `Audit` | Audit log recording and querying |
| Crypto | `Crypto` | Encryption/Decryption/Hashing |
| KeyManagement | `KeyManagement` | Key lifecycle management |
| Injection | `InjectionDetection` | Prompt injection detection |

**Type Definitions**:

| Submodule | Types |
|-----------|-------|
| Account | `UserId`, `Account`, `Group`, `AccountType`, `AccountStatus` |
| Auth | `Credentials`, `AuthToken`, `AuthContext`, `CredentialType` |
| Authorization | `Permission`, `ResourceType`, `Action`, `Policy`, `PolicyCondition`, `PermissionResult`, `DenyReason`, `DataClassification`, `ComplianceTag` |
| Audit | `AuditEvent`, `AuditEventType`, `AuditResult`, `AuditFilter`, `AuditStats` |
| Crypto | `CryptoAlgorithm`, `EncryptResult`, `HashResult` |
| KeyManagement | `KeyId`, `Key`, `KeyType` |
| Injection | `InjectionDetectionResult`, `RiskLevel`, `InjectionPattern` |

**Dependencies**: None (independent security foundation)


#### IV.4 Resource Manager (`resource/`)

| Item | Content |
|------|---------|
| **Core Positioning** | Central resource governance hub of LinAIx |
| **Design Principles** | Unified resource abstraction / Quota detection only, no decision making / Resources belong to Task |
| **Core Responsibilities** | Model instance management / KV Cache management / Quota detection and reporting / Resource statistics and events |

**Core Concepts**:

```
┌─────────────────────────────────────────────────────────────────────────────────────────────────┐
│                                      Resource Manager                                          │
│                                                                                                 │
│  ┌─────────────────────────────────────────────────────────────────────────────────────────┐   │
│  │                                    Unified Resource Abstraction                         │   │
│  │                                                                                         │   │
│  │  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐     │   │
│  │  │   ModelResource │  │  KvCacheResource│  │   QuotaResource │  │   StatsResource │     │   │
│  │  │   Model         │  │   KV Cache      │  │   Quota         │  │   Resource      │     │   │
│  │  │   Management    │  │   Management    │  │   Management    │  │   Statistics    │     │   │
│  │  └─────────────────┘  └─────────────────┘  └─────────────────┘  └─────────────────┘     │   │
│  └─────────────────────────────────────────────────────────────────────────────────────────┘   │
│                                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────────────────────┘
```

**Concept Relationships**:

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                              Task (Resource Container)                     │
│                                                                             │
│  Attributes:                                                                │
│  - TaskId                            ← Resource ownership identifier        │
│  - TaskQuota (GPU/Memory/Token)      ← Resource quota                      │
│  - shared_kv_cache                   ← Shared resource                     │
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                      SubTask (Scheduling Unit)                      │   │
│  │                                                                     │   │
│  │  Attributes:                                                         │   │
│  │  - SubTaskId                       ← Scheduling identifier          │   │
│  │  - status (Pending/Ready/Running...) ← Scheduling state             │   │
│  │                                                                     │   │
│  │  ┌─────────────────────────────────────────────────────────────┐   │   │
│  │  │                    Model Instance                           │   │   │
│  │  │                                                             │   │   │
│  │  │  Owner: Task                                                │   │   │
│  │  │  User: SubTask                                              │   │   │
│  │  │  Shared: SubTasks under same Task share                     │   │   │
│  │  └─────────────────────────────────────────────────────────────┘   │   │
│  │                                                                     │   │
│  │  ┌─────────────────────────────────────────────────────────────┐   │   │
│  │  │                    KV Cache                                  │   │   │
│  │  │                                                             │   │   │
│  │  │  Owner: Task                                                │   │   │
│  │  │  State: Active → SwappedOut → Persisted → Freeing          │   │   │
│  │  │  Shared: SubTasks under same Task share                     │   │   │
│  │  └─────────────────────────────────────────────────────────────┘   │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

**KV Cache Lifecycle**:

```
┌──────────┐  Allocate  ┌──────────┐  Swap Out  ┌──────────┐  Persist   ┌──────────┐
│ Unalloc  │ ─────────> │  Active  │ ─────────> │ Swapped  │ ─────────> │Persisted │
└──────────┘            └──────────┘            └──────────┘            └──────────┘
                             │                      │                      │
                             │ Swap In              │ Free                 │ Restore
                             ▼                      ▼                      ▼
                        ┌──────────┐          ┌──────────┐          ┌──────────┐
                        │  Active  │          │  Freed   │<─────────│  Active  │
                        └──────────┘          └──────────┘          └──────────┘
```

**Model Instance Lifecycle**:

```
┌──────────┐  Load     ┌──────────┐  Use      ┌──────────┐  Free     ┌──────────┐
│ Unloaded │ ────────> │ Loading  │ ────────> │  Ready   │ ────────> │  Freed   │
└──────────┘           └──────────┘           └──────────┘           └──────────┘
                            │                      │
                            │ Error                │ Ref Count == 0
                            ▼                      ▼
                       ┌──────────┐          ┌──────────┐
                       │  Error   │<─────────│  Cached  │
                       └──────────┘          └──────────┘
```

**Core Interfaces**:

| Submodule | Interface | Responsibility |
|-----------|-----------|----------------|
| Model | `ModelResource` | Model instance acquire/release/preload/unload |
| KvCache | `KvCacheResource` | KV Cache allocate/free/swap_in/swap_out/persist/restore |
| Quota | `QuotaResource` | Quota set/get/check/report/update |
| Stats | `StatsResource` | Resource usage query / pressure metrics / event subscription |

**Type Definitions**:

| Submodule | Types |
|-----------|-------|
| Model | `ModelId`, `ModelInstanceHandle`, `ModelInstanceState`, `LoadConfig`, `InstancePoolStatus` |
| KvCache | `KvCacheHandle`, `StorageLocation`, `KvCacheState`, `KvCacheStatus`, `KvCacheStats` |
| Quota | `ResourceType`, `ResourceQuota`, `ResourceRequest`, `ResourceUsage`, `ExceededDetail`, `QuotaStatus` |
| Stats | `AgentResourceUsage`, `GlobalResourceUsage`, `ResourcePressure`, `PressureLevel`, `HistoricalDataPoint` |

**Dependencies**: `device`, `model`, `security`, `task`


#### IV.5 Task (`task/`)

| Item | Content |
|------|---------|
| **Core Positioning** | Task and SubTask definition and management |
| **Design Principles** | Task is resource container, SubTask is execution unit (analogous to Linux Process/Thread) |
| **Core Responsibilities** | Task lifecycle management / SubTask lifecycle management / Dependency management (DAG) / Quota association |

**Core Concepts**:

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                              Task                                           │
│                           Resource Allocation Unit                         │
│                                                                             │
│  Attributes:                                                                │
│  - TaskId                            ← Resource ownership identifier        │
│  - TaskQuota (GPU/Memory/Token)      ← Resource quota                      │
│  - subtasks: Vec<SubTaskId>          ← SubTask list                        │
│  - shared_kv_cache                   ← Shared resource                     │
│  - framework_id                      ← Associated framework                │
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                      SubTask                                         │   │
│  │                         Execution Scheduling Unit                   │   │
│  │                                                                     │   │
│  │  Attributes:                                                         │   │
│  │  - SubTaskId                     ← Scheduling identifier            │   │
│  │  - task_id                       ← Parent Task                      │   │
│  │  - dependencies: Vec<SubTaskId>  ← DAG dependencies (predecessors)  │   │
│  │  - dependents: Vec<SubTaskId>    ← DAG dependencies (successors)   │   │
│  │  - status: SubTaskStatus         ← Scheduling state                 │   │
│  │  - priority: SubTaskPriority     ← Subtask-level priority           │   │
│  │  - result: SubTaskResult         ← Execution result                 │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

**Task Lifecycle**:

```
┌──────────┐  Create   ┌──────────┐  Start    ┌──────────┐  Complete ┌──────────┐
│ Created  │ ────────> │  Active  │ ────────> │Suspended │ ────────> │Terminated│
└──────────┘           └──────────┘           └──────────┘           └──────────┘
                            │                      │
                            │ All SubTasks Done    │ Resume
                            ▼                      ▼
                       ┌──────────┐          ┌──────────┐
                       │Completed │<─────────│  Active  │
                       └──────────┘          └──────────┘
```

**SubTask State Machine**:

```
┌──────────┐  Deps     ┌──────────┐  Schedule ┌──────────┐
│ Pending  │ ────────> │  Ready   │ ────────> │ Running  │
└──────────┘           └──────────┘           └────┬─────┘
      │                                            │
      │                                            ├── Complete ─> ┌──────────┐
      │                                            │               │Completed │
      │                                            │               └──────────┘
      │                                            │
      │                                            ├── Fail ──> ┌──────────┐
      │                                            │            │  Failed  │
      │                                            │            └──────────┘
      │                                            │
      │                                            ├── Cancel ─> ┌──────────┐
      │                                            │             │ Canceled │
      │                                            │             └──────────┘
      │                                            │
      │                                            ├── Wait Event ─> ┌──────────┐
      │                                            │                 │ Blocked  │
      │                                            │                 └────┬─────┘
      │                                            │                      │ Event Wake
      │                                            │                      ▼
      │                                            │                 ┌──────────┐
      │                                            │                 │ Running  │
      │                                            │                 └──────────┘
      │
      └── Deps Not Satisfied ───────────────────────────────────────────────┘
```

**Concept Relationships**:

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                            Task (1)                                        │
│                              │                                             │
│                              │ Contains (1:N)                             │
│                              ▼                                             │
│                            SubTask (N)                                    │
│                              │                                             │
│                              │ Depends On (DAG)                           │
│                              ▼                                             │
│                            SubTask (Dependency)                           │
│                                                                             │
│  Relationship Notes:                                                       │
│  • 1 Task : N SubTask                                                      │
│  • SubTask dependencies form a DAG                                         │
│  • SubTask states change independently                                    │
│  • Task state aggregates from all SubTask states                          │
└─────────────────────────────────────────────────────────────────────────────┘
```

**Core Interfaces**:

| Submodule | Interface | Responsibility |
|-----------|-----------|----------------|
| Task | `TaskManager` | Task create/query/state update/terminate/quota check |
| SubTask | `SubTaskManager` | SubTask create/state update/dependency check/topological sort/complete mark |

**Type Definitions**:

| Submodule | Types |
|-----------|-------|
| Task | `TaskId`, `Task`, `TaskQuota`, `TaskUsage`, `TaskPriority`, `TaskStatus` |
| SubTask | `SubTaskId`, `SubTask`, `SubTaskPriority`, `SubTaskStatus`, `SubTaskResult`, `SubTaskMetrics` |

**Dependencies**: `framework` (type references only)


#### IV.6 Scheduler (`scheduler/`)

| Item | Content |
|------|---------|
| **Core Positioning** | SubTask scheduling (analogous to Linux Process Scheduler) |
| **Design Principles** | Mechanism and Policy Separation / Reference Linux Scheduler Design |
| **Core Responsibilities** | Runqueue management / Waitqueue management / Priority scheduling / Preemption control / SchedClass management |

**Core Concepts**:

```
┌─────────────────────────────────────────────────────────────────────────────────────────────────┐
│                                          Scheduler                                              │
│                                                                                                 │
│  ┌─────────────────────────────────────────────────────────────────────────────────────────┐   │
│  │                                  RunQueue                                                │   │
│  │                         SubTasks in Ready state                                         │   │
│  │                                                                                         │   │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐                    │   │
│  │  │  Realtime   │  │    High     │  │   Normal    │  │     Low     │                    │   │
│  │  │  (Prio 0)   │  │  (Prio 1)   │  │  (Prio 2)   │  │  (Prio 3)   │                    │   │
│  │  └─────────────┘  └─────────────┘  └─────────────┘  └─────────────┘                    │   │
│  └─────────────────────────────────────────────────────────────────────────────────────────┘   │
│                                                                                                 │
│  ┌─────────────────────────────────────────────────────────────────────────────────────────┐   │
│  │                                  WaitQueue                                               │   │
│  │                         SubTasks waiting for events                                     │   │
│  │                                                                                         │   │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐                    │   │
│  │  │  Wait IPC   │  │  Wait Res   │  │  Wait Dep   │  │  Wait Timer │                    │   │
│  │  └─────────────┘  └─────────────┘  └─────────────┘  └─────────────┘                    │   │
│  └─────────────────────────────────────────────────────────────────────────────────────────┘   │
│                                                                                                 │
│  ┌─────────────────────────────────────────────────────────────────────────────────────────┐   │
│  │                                    SchedClass                                            │   │
│  │                                                                                         │   │
│  │  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐                          │   │
│  │  │   Realtime      │  │     Fair        │  │     Idle        │                          │   │
│  │  │   (Real-time)   │  │   (Fair)        │  │   (Idle)        │                          │   │
│  │  │   SCHED_FIFO    │  │     CFS         │  │   SCHED_IDLE    │                          │   │
│  │  └─────────────────┘  └─────────────────┘  └─────────────────┘                          │   │
│  └─────────────────────────────────────────────────────────────────────────────────────────┘   │
│                                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────────────────────┘
```

**Scheduler Lifecycle**:

```
┌──────────┐  Init     ┌──────────┐ Schedule  ┌──────────┐  Stop    ┌──────────┐
│ Created  │ ────────> │ Running  │ ────────> │  Paused  │ ────────> │ Stopped  │
└──────────┘           └──────────┘           └──────────┘           └──────────┘
                            │                      │
                            │ Pause                │ Resume
                            ▼                      ▼
                       ┌──────────┐          ┌──────────┐
                       │ Pausing  │          │ Resuming │
                       └──────────┘          └──────────┘
```

**SubTask Scheduling Flow**:

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                          SubTask Scheduling Flow                           │
│                                                                             │
│  ┌──────────┐  Deps     ┌──────────┐ enqueue()  ┌──────────┐              │
│  │ Pending  │ ────────> │  Ready   │ ─────────> │ RunQueue │              │
│  └──────────┘           └──────────┘            └────┬─────┘              │
│                                                        │                    │
│                                               pick_next_task()             │
│                                                        │                    │
│                                                        ▼                    │
│  ┌──────────┐  Wake     ┌──────────┐  Block   ┌──────────┐               │
│  │ WaitQueue│ <──────── │ Blocked  │ <─────── │ Running  │               │
│  └────┬─────┘           └──────────┘          └────┬─────┘               │
│       │                                             │                     │
│       │                                             ├── Complete ─> Completed
│       │                                             │                     │
│       │                                             ├── Fail ──> Failed
│       │                                             │                     │
│       │                                             ├── Timeout ──> Timeout
│       │                                             │                     │
│       └── Wake All Matching ────────────────────────┘                     │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

**Core Interfaces**:

| Submodule | Interface | Responsibility |
|-----------|-----------|----------------|
| Main | `Scheduler` | Main scheduler interface (queue/schedule/preempt/stats) |
| SchedClass | `SchedClass` | SchedClass interface (similar to Linux sched_class) |

**Type Definitions**:

| Submodule | Types |
|-----------|-------|
| RunQueue | `RunQueueEntry`, `RunQueueStats` |
| WaitQueue | `WaitQueueEntry`, `WaitReason`, `WaitQueueStats` |
| Priority | `SchedulingPriority`, `DynamicPriority` |
| SchedClass | `SchedClassType`, `SchedClassPriority` |
| Timeslice | `TimeSliceConfig` |
| Config | `SchedulerConfig`, `SchedulingPolicy` |
| Stats | `SchedulerStats`, `SchedulerStatus` |

**Dependencies**: `task`, `resource`, `model`, `security`, `ipc`


#### IV.7 Skill API (`skill/`)

| Item | Content |
|------|---------|
| **Core Positioning** | LinAIx's "System Call Table" (analogous to Linux syscall) |
| **Design Principles** | Standardization / Default Deny / End-to-End Audit |
| **Core Responsibilities** | Skill registration and discovery / Skill invocation (sync/async/stream) / Skill permission check / Skill version management / Framework adapter |

**Core Concepts**:

```
┌─────────────────────────────────────────────────────────────────────────────────────────────────┐
│                                          Skill API                                             │
│                                                                                                 │
│  ┌─────────────────────────────────────────────────────────────────────────────────────────┐   │
│  │                                    Skill Registry                                        │   │
│  │                                                                                         │   │
│  │  ┌─────────────────────────────────────────────────────────────────────────────────┐   │   │
│  │  │  Skill ID: {framework}/{namespace}/{name}@{version}                             │   │   │
│  │  │                                                                                 │   │   │
│  │  │  Examples:                                                                      │   │   │
│  │  │  • linAIx/system/kv_cache_read@v1                                              │   │   │
│  │  │  • langchain/search/web@v2.1.0                                                 │   │   │
│  │  │  • crewai/database/query@v1.0.0                                                │   │   │
│  │  └─────────────────────────────────────────────────────────────────────────────────┘   │   │
│  └─────────────────────────────────────────────────────────────────────────────────────────┘   │
│                                    │                                                           │
│                                    ▼                                                           │
│  ┌─────────────────────────────────────────────────────────────────────────────────────────┐   │
│  │                                    Skill Invocation Pipeline                            │   │
│  │                                                                                         │   │
│  │  1. SkillRegistry.lookup()  →  Find Skill metadata                                     │   │
│  │  2. Security.Auth           →  Authentication                                          │   │
│  │  3. Security.Authorization  →  Permission check (Default Deny)                         │   │
│  │  4. Resource.Quota          →  Quota check                                             │   │
│  │  5. Input Validation        →  Validate against input schema                          │   │
│  │  6. Security.Injection      →  Injection detection                                     │   │
│  │  7. SkillExecutor.execute() →  Execute Skill                                           │   │
│  │  8. Security.Audit          →  Audit logging                                           │   │
│  │  9. Resource.Quota          →  Quota update                                            │   │
│  └─────────────────────────────────────────────────────────────────────────────────────────┘   │
│                                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────────────────────┘
```

**Skill Lifecycle**:

```
┌──────────┐ Register ┌──────────┐ Version   ┌──────────┐ Deprecate ┌──────────┐
│ Unreg'd  │ ────────> │  Active  │ ────────> │ Updating │ ────────> │Deprecated│
└──────────┘           └──────────┘           └──────────┘           └──────────┘
                            │                                           │
                            │ Version Conflict                          │ Migrate
                            ▼                                           ▼
                       ┌──────────┐                               ┌──────────┐
                       │ Conflict │                               │Migrating │
                       └──────────┘                               └──────────┘
```

**Core Interfaces**:

| Submodule | Interface | Responsibility |
|-----------|-----------|----------------|
| Registry | `SkillRegistry` | Skill register/unregister/discover/version management |
| Invocation | `SkillInvoker` | Skill invocation (sync/async/stream/cancel) |
| Executor | `SkillExecutor` | Skill execution logic |
| Permission | `SkillPermissionChecker` | Skill permission check (Default Deny) |
| Adapter | `SkillAdapter` | Framework adapter interface |

**Type Definitions**:

| Submodule | Types |
|-----------|-------|
| ID | `SkillId`, `SkillVersion` |
| Metadata | `SkillMetadata`, `RetryConfig`, `SkillStatus` |
| Invocation | `SkillInvocation`, `SkillInvocationContext` |
| Result | `SkillResult`, `SkillExecutionError`, `SkillErrorCode` |
| Permission | `SkillPermission`, `SkillIdPattern`, `SkillAction` |
| Adapter | `AdapterDirection`, `AdapterContext`, `AdapterResult` |

**Dependencies**: `task`, `security`, `resource`, `model`


#### IV.8 Agent Runtime (`runtime/`)

| Item | Content |
|------|---------|
| **Core Positioning** | Agent code execution environment (analogous to Linux user-space process) |
| **Design Principles** | Agent binds to SubTask (1:1) / Sandbox isolation |
| **Core Responsibilities** | Agent execution environment / Agent lifecycle / System call bridge / Sandbox management |

**Core Concepts**:

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                          Agent Runtime                                     │
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                    Agent Code (Python/Go/JS)                        │   │
│  │                    • Business logic                                  │   │
│  │                    • Call Skill (via System Call Bridge)           │   │
│  │                    • Call Model (via Model Runtime)                │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                    │                                       │
│                                    ▼                                       │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                    System Call Bridge                               │   │
│  │                    • Skill call translation                         │   │
│  │                    • Parameter serialization/deserialization       │   │
│  │                    • Return value conversion                        │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                    │                                       │
│                                    ▼                                       │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                    Agent Sandbox                                    │   │
│  │                    • Memory isolation (bound to Task)              │   │
│  │                    • Filesystem isolation                          │   │
│  │                    • Network isolation                             │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

**Agent Lifecycle**:

```
┌──────────┐  start()   ┌──────────┐   Init Done  ┌──────────┐
│ Created  │ ─────────> │ Starting │ ───────────> │ Running  │
└──────────┘            └──────────┘              └────┬─────┘
                                                        │
                                   ┌────────────────────┼────────────────────┐
                                   │                    │                    │
                              pause()              complete()            error()
                                   │                    │                    │
                                   ▼                    ▼                    ▼
                              ┌──────────┐       ┌──────────┐       ┌──────────┐
                              │ Pausing  │       │Completed │       │  Error   │
                              └────┬─────┘       └──────────┘       └──────────┘
                                   │
                                   ▼
                              ┌──────────┐  resume()  ┌──────────┐
                              │  Paused  │ ─────────> │ Resuming │
                              └──────────┘            └────┬─────┘
                                   │                        │
                                   │ terminate()            ▼
                                   ▼                  ┌──────────┐
                              ┌──────────┐           │ Running  │
                              │Stopping  │           └──────────┘
                              └────┬─────┘
                                   │
                                   ▼
                              ┌──────────┐
                              │Terminated│
                              └──────────┘
```

**Concept Relationships**:

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                            Task (1)                                        │
│                              │                                             │
│                              │ Contains (1:N)                             │
│                              ▼                                             │
│                            SubTask (N)                                    │
│                              │                                             │
│                              │ Binds (1:1)                                │
│                              ▼                                             │
│                         Agent Runtime (1)                                 │
│                                                                             │
│  Relationship Notes:                                                       │
│  • 1 Task : N SubTask                                                      │
│  • 1 SubTask : 1 Agent Runtime                                             │
│  • Agent Runtime is created when SubTask executes                         │
│  • Agent Runtime is destroyed when SubTask completes                      │
│  • Agent Runtime shares Task's resources and quota                        │
└─────────────────────────────────────────────────────────────────────────────┘
```

**Core Interfaces**:

| Submodule | Interface | Responsibility |
|-----------|-----------|----------------|
| Agent | `Agent` | Agent runtime/status query/metrics |
| Lifecycle | `AgentLifecycle` | Agent start/pause/resume/terminate |
| Sandbox | `AgentSandbox` | Sandbox create/configure/execute |
| Bridge | `SystemCallBridge` | System call execute/permission check |

**Type Definitions**:

| Submodule | Types |
|-----------|-------|
| Agent | `AgentId`, `Agent`, `AgentStatus`, `AgentMetrics` |
| Lifecycle | `LifecycleState`, `LifecycleEvent`, `LifecycleEventReason` |
| Sandbox | `SandboxConfig`, `SandboxLimits`, `MountPoint`, `NetworkConfig` |
| Bridge | `SystemCall`, `SystemCallType`, `SystemCallResult` |
| Config | `RuntimeConfig`, `AgentConfig`, `AgentLanguage`, `LogLevel` |

**Dependencies**: All lower layers (no restrictions)


#### IV.9 Framework (`framework/`)

| Item | Content |
|------|---------|
| **Core Positioning** | Agent framework management (analogous to Linux Shell management) |
| **Design Principles** | Pluggable / Extensible / Runtime management |
| **Core Responsibilities** | Framework install/uninstall / Framework runtime management / Framework adapter management / Default framework selection |

**Core Concepts**:

```
┌─────────────────────────────────────────────────────────────────────────────────────────────────┐
│                                          Framework                                              │
│                                                                                                 │
│  ┌─────────────────────────────────────────────────────────────────────────────────────────┐   │
│  │                                      Framework Manager                                   │   │
│  │                                                                                         │   │
│  │  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐                          │   │
│  │  │   LangChain     │  │    CrewAI       │  │    AutoGen      │                          │   │
│  │  │   Installed v0.3│  │   Installed v0.2│  │   Installed v0.4│                          │   │
│  │  │   Default       │  │                 │  │                 │                          │   │
│  │  └─────────────────┘  └─────────────────┘  └─────────────────┘                          │   │
│  └─────────────────────────────────────────────────────────────────────────────────────────┘   │
│                                    │                                                           │
│                                    ▼                                                           │
│  ┌─────────────────────────────────────────────────────────────────────────────────────────┐   │
│  │                                      Runtime Manager                                    │   │
│  │                                                                                         │   │
│  │  ┌─────────────────────────────────────────────────────────────────────────────────┐   │   │
│  │  │  Task-123 → Framework Runtime (LangChain) → PID 12345 → Status: Running          │   │   │
│  │  │  Task-456 → Framework Runtime (CrewAI)    → PID 12346 → Status: Running          │   │   │
│  │  └─────────────────────────────────────────────────────────────────────────────────┘   │   │
│  └─────────────────────────────────────────────────────────────────────────────────────────┘   │
│                                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────────────────────┘
```

**Framework Adapter Architecture**:

```
┌─────────────────────────────────────────────────────────────────────────────────────────────────┐
│                                    Framework Adapter Layer                                     │
│                                                                                                 │
│  ┌─────────────────────────────────────────────────────────────────────────────────────────┐   │
│  │                              FrameworkAdapter trait                                      │   │
│  │                                                                                         │   │
│  │  • framework_name()               → Framework name                                     │   │
│  │  • direction()                    → Adapter direction                                  │   │
│  │  • adapt_to_framework()           → LinAIx Skill → Framework native tool              │   │
│  │  • adapt_to_linAIx_skill()        → Framework tool → LinAIx Skill call                │   │
│  │  • intercept_model_call()         → Intercept framework model call → attach quota      │   │
│  │  • detect_and_report_subtask()    → Detect framework subtask → report TaskManager     │   │
│  │  • handle_quota_exceeded()        → Handle quota exceeded                             │   │
│  └─────────────────────────────────────────────────────────────────────────────────────────┘   │
│                                    │                                                           │
│                                    ▼                                                           │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐                               │
│  │  LangChain      │  │   CrewAI        │  │   AutoGen       │                               │
│  │  Adapter        │  │   Adapter       │  │   Adapter       │                               │
│  └─────────────────┘  └─────────────────┘  └─────────────────┘                               │
│                                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────────────────────┘
```

**Core Interfaces**:

| Submodule | Interface | Responsibility |
|-----------|-----------|----------------|
| Framework | `FrameworkManager` | Framework install/uninstall/query/default framework |
| Framework | `FrameworkRegistry` | Framework register/discover/capability matching |
| Adapter | `FrameworkAdapter` | Framework adaptation (Skill/Model/SubTask/Quota) |
| Runtime | `FrameworkRuntimeManager` | Framework runtime start/stop/pause/resume |

**Type Definitions**:

| Submodule | Types |
|-----------|-------|
| Framework | `FrameworkId`, `Framework`, `FrameworkType`, `FrameworkCapability`, `FrameworkConfig` |
| Adapter | `AdapterDirection`, `ModelCallRequest`, `ModelCallResponse`, `SubTaskInfo` |
| Runtime | `RuntimeHandle`, `FrameworkRuntime`, `RuntimeStatus` |

**Dependencies**: `task`, `security`, `scheduler`, `runtime`


#### IV.10 IPC (`ipc/`)

| Item | Content |
|------|---------|
| **Core Positioning** | Agent-to-Agent communication infrastructure (analogous to Linux IPC) |
| **Design Principles** | Mechanism and Policy Separation / Bound to Task/SubTask / Zero-copy shared memory |
| **Core Responsibilities** | Channel management / Message passing / Queue management / Semaphore synchronization / Shared memory |

**Core Concepts**:

```
┌─────────────────────────────────────────────────────────────────────────────────────────────────┐
│                                          IPC Subsystem                                          │
│                                                                                                 │
│  ┌─────────────────────────────────────────────────────────────────────────────────────────┐   │
│  │                                      IPC Channel                                         │   │
│  │                                                                                         │   │
│  │  ┌─────────────────────────────────────────────────────────────────────────────────┐   │   │
│  │  │  Communication Modes                                                             │   │   │
│  │  │                                                                                 │   │   │
│  │  │  • P2P (Point-to-Point)   ┌──────┐      ┌──────┐                                │   │   │
│  │  │                           │ SubA │──────│ SubB │                                │   │   │
│  │  │                           └──────┘      └──────┘                                │   │   │
│  │  │                                                                                 │   │   │
│  │  │  • Broadcast             ┌──────┐      ┌──────┐                                │   │   │
│  │  │                           │ SubA │─────▶│ SubB │                                │   │   │
│  │  │                           └──────┘      ├──────┤                                │   │   │
│  │  │                                         │ SubC │                                │   │   │
│  │  │                                         └──────┘                                │   │   │
│  │  │                                                                                 │   │   │
│  │  │  • Stream (Streaming)    ┌──────┐      ┌──────┐                                │   │   │
│  │  │                           │ SubA │─────▶│ SubB │  (Continuous data flow)        │   │   │
│  │  │                           └──────┘      └──────┘                                │   │   │
│  │  └─────────────────────────────────────────────────────────────────────────────────┘   │   │
│  └─────────────────────────────────────────────────────────────────────────────────────────┘   │
│                                                                                                 │
│  ┌─────────────────────────────────────────────────────────────────────────────────────────┐   │
│  │                                      Synchronization Primitives                         │   │
│  │                                                                                         │   │
│  │  • Semaphore              ┌──────┐  P Op   ┌──────┐                                   │   │
│  │                           │ SubA │ ──────▶ │ Sem  │  V Op   ┌──────┐                  │   │
│  │                           └──────┘         └──────┘ ──────▶ │ SubB │                  │   │
│  │                                                                └──────┘                  │   │
│  │                                                                                         │   │
│  │  • Shared Memory          ┌──────────────────────────────────────────────────┐          │   │
│  │                           │  ┌────────────────────────────────────────────┐  │          │   │
│  │                           │  │           Shared Memory Region             │  │          │   │
│  │                           │  │  SubA RW  │  SubB RW  │  SubC RO          │  │          │   │
│  │                           │  └────────────────────────────────────────────┘  │          │   │
│  │                           └──────────────────────────────────────────────────┘          │   │
│  └─────────────────────────────────────────────────────────────────────────────────────────┘   │
│                                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────────────────────┘
```

**Channel Lifecycle**:

```
┌──────────┐ create()  ┌──────────┐ bind()    ┌──────────┐ send/recv ┌──────────┐
│ Uncreated│ ────────> │ Created  │ ────────> │  Active  │ ────────> │   In Use │
└──────────┘           └──────────┘           └──────────┘           └────┬─────┘
                                                                           │
                                                                           │ close()
                                                                           ▼
                                                                      ┌──────────┐
                                                                      │  Closed  │
                                                                      └──────────┘
```

**Message Lifecycle**:

```
┌──────────┐ send()   ┌──────────┐ Enqueue  ┌──────────┐ Dequeue  ┌──────────┐
│ Created  │ ───────> │  Pending │ ───────> │ In Queue │ ───────> │  Ready   │
└──────────┘          └──────────┘          └──────────┘          └────┬─────┘
                                                                        │
                                                                        │ recv()
                                                                        ▼
                                                                   ┌──────────┐
                                                                   │Received  │
                                                                   └──────────┘
```

**Core Interfaces**:

| Submodule | Interface | Responsibility |
|-----------|-----------|----------------|
| Channel | `IpcChannelManager`, `IpcChannel` | Channel create/delete/pause/resume/send/recv |
| Message | `IpcMessageFactory` | Message creation (request/response/event) |
| Queue | `IpcQueue`, `IpcPriorityQueue` | Enqueue/dequeue/priority support |
| Semaphore | `IpcSemaphoreManager`, `IpcSemaphore` | Semaphore create/wait/signal |
| SHM | `IpcSharedMemory` | Shared memory alloc/free/map/unmap |
| Endpoint | `IpcEndpointManager`, `IpcEndpoint` | Endpoint create/bind/unbind/query |

**Type Definitions**:

| Submodule | Types |
|-----------|-------|
| Channel | `ChannelId`, `ChannelMode`, `ChannelStatus`, `ChannelConfig`, `ChannelStats` |
| Message | `MessageId`, `MessageType`, `MessagePriority`, `Message`, `MessageAck`, `AckStatus` |
| Queue | `QueueId`, `QueueConfig`, `QueueStats` |
| Semaphore | `SemaphoreId`, `SemaphoreType`, `SemaphoreStatus` |
| SHM | `SharedMemoryId`, `SharedMemoryRegion`, `ShmFlags`, `ShmStats` |
| Endpoint | `EndpointId`, `EndpointRole`, `EndpointStatus`, `Endpoint`, `EndpointStats` |

**Dependencies**: `task`, `security`, `resource`, `scheduler`


#### IV.11 Observability (`obs/`)

| Item | Content |
|------|---------|
| **Core Positioning** | LinAIx's "System Eye" (analogous to Linux tracepoint + perf) |
| **Design Principles** | Loose coupling / High performance / Structured / Configurable |
| **Core Responsibilities** | Metrics collection / Distributed tracing / System logging / State dump / Performance profiling / Event bus |

**Core Concepts**:

```
┌─────────────────────────────────────────────────────────────────────────────────────────────────┐
│                                       Observability                                            │
│                                                                                                 │
│  ┌─────────────────────────────────────────────────────────────────────────────────────────┐   │
│  │                                      Metrics                                              │   │
│  │  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐                          │   │
│  │  │    Counter      │  │     Gauge       │  │   Histogram     │                          │   │
│  │  │   (Monotonic)   │  │   (Instant)     │  │  (Distribution) │                          │   │
│  │  └─────────────────┘  └─────────────────┘  └─────────────────┘                          │   │
│  └─────────────────────────────────────────────────────────────────────────────────────────┘   │
│                                                                                                 │
│  ┌─────────────────────────────────────────────────────────────────────────────────────────┐   │
│  │                                      Tracing                                              │   │
│  │                                                                                         │   │
│  │  ┌─────────────────────────────────────────────────────────────────────────────────┐   │   │
│  │  │  Trace: User Request → Agent A → SubTask-1 → Skill → Model → Response           │   │   │
│  │  │         ├── Span-1: Agent A Processing  (100ms)                                 │   │   │
│  │  │         ├── Span-2: SubTask-1    (80ms)                                         │   │   │
│  │  │         ├── Span-3: Skill Invoke (50ms)                                         │   │   │
│  │  │         └── Span-4: Model Infer  (30ms)                                         │   │   │
│  │  └─────────────────────────────────────────────────────────────────────────────────┘   │   │
│  └─────────────────────────────────────────────────────────────────────────────────────────┘   │
│                                                                                                 │
│  ┌─────────────────────────────────────────────────────────────────────────────────────────┐   │
│  │                                      Event Bus                                          │   │
│  │                                                                                         │   │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐                    │   │
│  │  │   Agent     │  │   Task      │  │   Skill     │  │   Resource  │                    │   │
│  │  │   Event     │  │   Event     │  │   Event     │  │   Event     │                    │   │
│  │  └─────────────┘  └─────────────┘  └─────────────┘  └─────────────┘                    │   │
│  └─────────────────────────────────────────────────────────────────────────────────────────┘   │
│                                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────────────────────┘
```

**Core Interfaces**:

| Submodule | Interface | Responsibility |
|-----------|-----------|----------------|
| Metrics | `MetricsCollector` | Metric register/update/snapshot/export |
| Tracing | `TracingProvider` | Span management/sampling/export |
| Logging | `Logger` | Log write/query/level management |
| Dump | `DumpManager` | Dump create/restore/cleanup |
| Profiling | `Profiler` | Profile start/stop/data query |
| Event | `EventBus` | Event publish/subscribe/query |

**Type Definitions**:

| Submodule | Types |
|-----------|-------|
| Metrics | `MetricKey`, `MetricKind`, `MetricValue`, `MetricLabels`, `MetricSnapshot`, `MetricsConfig` |
| Tracing | `TraceId`, `SpanId`, `SpanKind`, `SpanStatus`, `SpanContext`, `Span`, `SamplingDecision` |
| Logging | `LogLevel`, `LogEntry`, `LogFilter`, `LogConfig` |
| Dump | `DumpType`, `DumpMetadata`, `DumpContent`, `DumpConfig` |
| Profiling | `ProfileType`, `ProfileSample`, `ProfileData`, `ProfileConfig` |
| Event | `EventType`, `EventSeverity`, `Event`, `EventFilter`, `EventStats` |

**Dependencies**: All modules (as data sources / event producers)