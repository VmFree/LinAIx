# LinAIx

<p align="center">
  <em>Where Linux Meets Intelligence — The Kernel for the AI Era</em>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/rust-1.85+-orange.svg" alt="Rust Version"/>
  <img src="https://img.shields.io/badge/license-GPLv2-blue.svg" alt="License"/>
  <img src="https://img.shields.io/badge/status-early--stage-red.svg" alt="Status"/>
</p>

---

## 📖 English | [中文](./README.zh-CN.md)

---

## 🎯 Core Mission

LinAIx is an operating system kernel built from the ground up for the AI-native era. Its core mission can be summarized in one sentence:

> **Provide operating system-level abstraction and governance for AI Agents, transforming them from uncontrollable black-box scripts into observable, schedulable, correctable, and securely isolated first-class system citizens.**

This mission is broken down into five dimensions:

### 1. Standardized System Calls — Skill as API

Every Agent framework today has its own incompatible tool invocation methods. LinAIx abstracts external capabilities as standard **Skills (system calls)**. Agents invoke unified interfaces while underlying implementations can be swapped seamlessly.

### 2. Kernel-Level Governance — Scheduling, Isolation, Quotas

Agents are treated as first-class operating system processes, with full lifecycle management, fair scheduling, resource isolation, and quota controls — preventing any single Agent from crashing the entire system.

### 3. Observability and Mandatory Intervention — A "Correction Toolbox" for Experts

Drawing from Linux's `ptrace` and `coredump`, LinAIx provides comprehensive kernel-level diagnostic capabilities. When Agents spiral out of control, experts can forcibly preempt, reclaim resources, and revoke permissions from the kernel layer — without Agent cooperation.

### 4. Compute Neutrality — Transparent Local/Remote Switching

Through the Hardware Abstraction Layer (HAL), Agents seamlessly switch between local GPUs, cloud APIs, or private model clusters. The scheduler automatically routes based on cost, latency, or privacy policies.

### 5. Security and Isolation — Zero-Trust Agent Sandbox

| Security Capability | Description | Analogy |
| :--- | :--- | :--- |
| **Identity & Authentication** | Every Agent has a unique identity verified at startup | Linux UID/GID |
| **Least Privilege** | Agents granted only explicitly required Skill permissions | Linux Capabilities |
| **Resource Isolation** | Agents run in independent resource sandboxes (memory/GPU/network isolated) | Container/VM isolation |
| **Data Sandbox** | Agents cannot directly access each other's memory or context data | Process address space isolation |
| **Prompt Injection Defense** | System-level input filtering detects and blocks malicious injection attacks | Web Application WAF |
| **Operation Auditing** | All Skill invocations, permission changes, and system operations are audited end-to-end | Linux auditd |

---

## 🤝 Multi-Agent Collaboration

LinAIx provides system-level inter-agent communication and coordination mechanisms, but **does not prescribe specific collaboration patterns**. This follows the classic operating system design principle — **"Mechanism and Policy Separation"**.

### Design Principle: Mechanism vs. Policy Separation

| Layer | Responsibility | Analogy |
| :--- | :--- | :--- |
| **Upper Layer (Application/Framework)** | Determines "how to collaborate" (master-worker, pipeline, voting, debate, etc.) | Application-level orchestration |
| **LinAIx Kernel** | Provides "primitive capabilities for collaboration" (communication, synchronization, sharing, scheduling, etc.) | Linux provides `pipe`/`socket`/`futex` |

### Primitive Capabilities Provided by LinAIx

| Capability | Description | Analogy |
| :--- | :--- | :--- |
| **IPC Channel** | Standardized message passing between Agents, supporting point-to-point, broadcast, and streaming communication | Linux `pipe()` / `socket()` |
| **Shared Memory** | Agents share KV Cache and context data with zero-copy for high-performance collaboration | Linux `shm` |
| **Synchronization Primitives** | Agents can wait, notify, and acquire locks for simple coordination | Linux `futex` / `semaphore` |
| **Collaboration Group** | Multiple Agents form a group for resource quota and scheduling at the group level | Linux `cgroup` |
| **Group-level Scheduling** | L3 Scheduler allocates GPU time slices at the group level to ensure coordinated execution | Linux CFS group scheduling |
| **End-to-End Tracing** | Cross-Agent requests are traced with a unified Trace ID for full observability | Linux `tracepoint` |
| **Inter-Agent Authorization** | Agent A requires explicit permissions to send messages to Agent B or invoke its Skills | Linux file permissions |


## 🔴 The Agent Ecosystem Pain Points

LinAIx addresses **7 categories with 29 specific pain points** prevalent in current AI Agent development and operations.

### Pain Points Overview

| Category | Pain Point | Manifestation | LinAIx Solution |
| :---: | :--- | :--- | :--- |
| **① Tools & Interop** | Tool invocation incompatible | LangChain uses `@tool`, CrewAI inherits `BaseTool`, AutoGen uses `register_function` — switching frameworks requires rewriting all tool code | L4 Skill API unifies system calls |
| | Tool versioning chaos | Tool updates cannot be smoothly migrated; Agent code hardcodes tool names and parameters | Skill registry supports versioned routing |
| **② Resources & Scheduling** | No resource governance | A single Agent can exhaust all GPU memory without quota limits, impacting other Agents | L2 Resource Manager + quota controls |
| | Token explosion & cost runaway | Agent enters a loop generating millions of Tokens, API costs explode | Scheduler Token quota caps + mandatory preemption |
| | Deadlock & mutual wait | Agent A holds resources waiting for B, B holds resources waiting for A — system freezes | Built-in deadlock detection and breaking |
| | Resource starvation | High-priority Agents cannot get inference slots, tasks time out | Weighted fair queuing prevents starvation |
| | No fair scheduling | High-priority tasks preempted by low-priority tasks, critical Agent response quality suffers | L3 Scheduler priority + preemption |
| | Inference engine fragmentation | vLLM/TensorRT-LLM/llama.cpp each have different APIs; Agents need adapters for each engine | L1 HAL unified inference interface |
| **③ Observability & Ops** | Ops black box — not observable | Agent reasoning process completely invisible — cannot see what it's thinking or why it's stuck | Full observability interface |
| | Cannot debug or reproduce | Temperature sampling introduces randomness — same Prompt yields different results at different times | Deterministic mode + reasoning chain recording |
| | No step-through debugging | No `gdb`-like tool for stepping through Agent reasoning chains | Kernel-level debug interface (breakpoints/step) |
| | State not persistent | Agent crash loses KV Cache and conversation history — long tasks restart from scratch | KV Cache persistence to disk |
| | State cannot migrate | Agent cannot migrate across GPUs/nodes — high failure recovery cost | State serialization + migration |
| **④ Compute & Models** | Compute lock-in | Agent hardcodes `gpt-4-turbo` bound to OpenAI, or hardcodes local Llama bound to specific hardware | L1 HAL compute abstraction |
| | Model version chaos | Model deprecation/update/performance regression requires manually modifying all Agent code and redeploying | Model routing table with dynamic switching/canary rollback |
| | Cold start disaster | 70B model loading takes 30-60 seconds; KV Cache recomputed every time — high-frequency tasks extremely inefficient | Model preloading + KV reuse + Fork |
| | No dynamic routing | Cannot automatically select optimal inference backend based on cost, latency, or privacy policies | Policy engine auto-routing |
| **⑤ Security & Isolation** | No inter-Agent isolation | One Agent can read/modify another Agent's memory or files — data leak risk | Data sandbox + process address space isolation |
| | Missing permission controls | Agent can invoke any tool without least-privilege principle — malicious Agent can abuse system capabilities | RBAC/ABAC + default deny |
| | Prompt injection attacks | User input can manipulate Agent into executing unexpected system commands or privilege escalation | System-level input filtering + injection detection |
| | No operation auditing | Cannot trace "who invoked what tool, when, and with what result" | End-to-end audit logs |
| | API key security risks | Multi-cloud/multi-model API keys scattered across code and config files | Secure key storage + unified credential management |
| **⑥ Lifecycle** | Agent lifecycle chaos | No unified startup, pause, resume, terminate mechanism | Scheduler unified lifecycle management |
| | No graceful degradation | Remote API unavailable or rate-limited causes Agent to crash with no fallback | HAL automatic failover switching |
| | Missing inter-Agent communication | Multi-Agent collaboration has no standardized messaging mechanism | IPC subsystem standardized messaging |
| | Long-running degradation | Long-context Agent memory bloats, slowing inference | Memory Manager automatic compression/archiving |
| **⑦ Developer Experience** | No standardized SDK | Each framework has its own Agent development paradigm — steep learning curve | Unified SDK (Python/Go) |
| | Local vs production environment mismatch | Local small models behave differently from production large models | HAL ensures behavior consistency |
| | No CI/CD support | Agent code changes lack standardized testing, deployment, rollback processes | Canary deployment + version management |

### Pain Point → Subsystem Mapping

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│                    Pain Point → LinAIx Subsystem Mapping                        │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  ① Tools & Interoperability  →  L4 Tool Manager (Skill API)                   │
│  ② Resources & Scheduling    →  L3 Scheduler + L2 Resource Manager            │
│  ③ Observability & Ops       →  Observability Cross-Cutting Service           │
│  ④ Compute & Models          →  L1 HAL + L3 Scheduler (Policy Engine)         │
│  ⑤ Security & Isolation      →  Security Manager Cross-Cutting Service        │
│  ⑥ Lifecycle                 →  L3 Scheduler + L2 Resource Manager + IPC      │
│  ⑦ Developer Experience      →  SDK (Python/Go) + Tool Manager (Registry)    │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

---

## 📍 Where LinAIx Fits

LinAIx occupies the **operating system layer** in the AI technology stack — filling the gap between Agent frameworks and inference engines.

```
┌─────────────────────────────────────────────────────────────────┐
│  Application    Business Logic, User Interface                │
│  ───────────────────────────────────────────────────────────── │
│  Agent Frameworks  LangChain, CrewAI, AutoGen                │
│                    (Planning, Memory, Tool Orchestration)    │
│  ───────────────────────────────────────────────────────────── │
│  ╔═══════════════════════════════════════════════════════════╗│
│  ║  LinAIx Kernel  ◄── We are here                          ║│
│  ║  Scheduling, Isolation, Observability, Skill Abstraction ║│
│  ║  Compute Routing, Security Sandbox                       ║│
│  ╚═══════════════════════════════════════════════════════════╝│
│  ───────────────────────────────────────────────────────────── │
│  Inference Engines  vLLM, TensorRT-LLM, llama.cpp           │
│                    (Model Loading, Efficient Inference)      │
│  ───────────────────────────────────────────────────────────── │
│  Hardware          GPU / TPU / CPU                          │
└─────────────────────────────────────────────────────────────────┘
```

**LinAIx does NOT solve "intelligence problems" (reasoning quality, hallucinations, alignment). It solves "systems problems" (reliability, governability, scalability, security).**

---

## 🏗️ Architecture

### Five-Layer Architecture Overview

```
┌─────────────────────────────────────────────────────────────────────┐
│                                                                     │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │                   User Space                                 │   │
│  │  ┌──────────────────────────────────────────────────────┐  │   │
│  │  │  L5  Agent Runtime                                  │  │   │
│  │  │  • Agent process execution environment              │  │   │
│  │  │  • Lifecycle management (start/pause/resume/terminate)│ │   │
│  │  │  • Multi-Agent collaboration                         │  │   │
│  │  └──────────────────────────────────────────────────────┘  │   │
│  │  ┌──────────────────────────────────────────────────────┐  │   │
│  │  │  L4  Skill API — "System Call Table"               │  │   │
│  │  │  • Tool registration and discovery                  │  │   │
│  │  │  • Invocation authentication and auditing           │  │   │
│  │  │  • Rate limiting and circuit breaking               │  │   │
│  │  └──────────────────────────────────────────────────────┘  │   │
│  └─────────────────────────────────────────────────────────────┘   │
│                                                                     │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │                   Kernel Space                              │   │
│  │  ┌──────────────────────────────────────────────────────┐  │   │
│  │  │  L3  Agent Scheduler — "Process Scheduler"          │  │   │
│  │  │  • Task queuing and dispatching                     │  │   │
│  │  │  • Priority and preemption                          │  │   │
│  │  │  • Context switching (KV Cache save/restore)        │  │   │
│  │  │  • Deadlock detection and breaking                  │  │   │
│  │  └──────────────────────────────────────────────────────┘  │   │
│  │  ┌──────────────────────────────────────────────────────┐  │   │
│  │  │  L2  Resource Manager — "Memory Management"         │  │   │
│  │  │  • KV Cache pooling                                 │  │   │
│  │  │  • GPU memory / CPU memory allocation               │  │   │
│  │  │  • Quotas and rate limiting                         │  │   │
│  │  │  • State persistence and migration                  │  │   │
│  │  └──────────────────────────────────────────────────────┘  │   │
│  │  ┌──────────────────────────────────────────────────────┐  │   │
│  │  │  L1  Hardware Abstraction Layer — "Device Drivers"  │  │   │
│  │  │  • Local inference (GPU / CPU)                      │  │   │
│  │  │  • Remote inference (Cloud API / Model Clusters)    │  │   │
│  │  │  • Heterogeneous chip adapters (NVIDIA/AMD/Huawei)  │  │   │
│  │  │  • Model routing and version management             │  │   │
│  │  └──────────────────────────────────────────────────────┘  │   │
│  └─────────────────────────────────────────────────────────────┘   │
│                                                                     │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │              Cross-Cutting Services                         │   │
│  │  ┌───────────┐  ┌───────────┐  ┌───────────────────────┐  │   │
│  │  │Observabilty│  │   IPC     │  │  Memory Manager       │  │   │
│  │  │Metrics/    │  │ Agent-to- │  │  Vector Storage/      │  │   │
│  │  │Traces/     │  │ Agent     │  │  Long-term Memory     │  │   │
│  │  │Logs/Dumps  │  │ Messaging │  │  Compression/Archive  │  │   │
│  │  └───────────┘  └───────────┘  └───────────────────────┘  │   │
│  │  ┌───────────────────────────────────────────────────────┐  │   │
│  │  │  Security Manager — Cross-Cuts All Layers            │  │   │
│  │  │  Authentication / Authorization / Data Isolation /   │  │   │
│  │  │  Injection Defense / Auditing                        │  │   │
│  │  └───────────────────────────────────────────────────────┘  │   │
│  └─────────────────────────────────────────────────────────────┘   │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘
```

### Subsystem Dependency Diagram

```
                         ┌─────────────────┐
                         │   Agent App     │
                         └────────┬────────┘
                                  │
                                  ▼
                         ┌─────────────────┐
                         │   Tool Manager  │ ◄── Skill register/invoke/auth
                         │   (L4)          │
                         └────────┬────────┘
                                  │
         ┌────────────────────────┼────────────────────────┐
         │                        │                        │
         ▼                        ▼                        ▼
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│    Scheduler    │    │       IPC       │    │  Memory Manager │
│    (L3)         │    │   Agent-to-     │    │   Long-term     │
│                 │    │   Agent Comm    │    │   Memory        │
└────────┬────────┘    └─────────────────┘    └─────────────────┘
         │
         ▼
┌─────────────────┐
│    Resource     │
│    Manager      │
│    (L2)         │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│    HAL (L1)     │ ◄── Local/Remote Inference
└─────────────────┘

┌─────────────────────────────────────────────────────────────────┐
│                  Observability (Cross-Cuts All)                 │
│          Metrics / Tracing / Logs / Core Dump                   │
├─────────────────────────────────────────────────────────────────┤
│                  Security Manager (Cross-Cuts All)              │
│   Authentication / Authorization / Data Isolation / Injection   │
│   Defense / End-to-End Auditing                                │
└─────────────────────────────────────────────────────────────────┘
```

---

## 💡 Core Concepts

### Agent as Process

An Agent is a first-class system process with:
- **PID**: Unique identity
- **Credentials**: UID, GID, permission sets
- **Context**: KV Cache, conversation history, state
- **Priority**: Determines scheduling order
- **Resource Limits**: Memory, Token, time quotas
- **Sandbox Boundaries**: Independent memory space, filesystem view, network namespace

The Scheduler manages the full lifecycle: `Submitted → Ready → Running → Suspended/Waiting → Terminated`

### Skill as System Call

External tools (search, database, email, API) are registered in the kernel as **Skills**, analogous to Linux's system call table.

When an Agent invokes a Skill, it passes through a complete security pipeline:
1. **Authentication**: Verify Agent identity is valid
2. **Authorization**: Check Agent has permission to invoke this Skill
3. **Resource Quota**: Check Token/frequency quota is sufficient
4. **Input Filtering**: Detect and block malicious inputs like Prompt injection
5. **Execution**: Execute within the sandbox
6. **Audit Log**: Record complete invocation chain (who, when, what, result)

### Kernel Correction Toolbox

Drawing from Linux's `ptrace`, `coredump`, and `/proc` filesystem, LinAIx provides experts with:

| Capability | Analogy | Description |
| :--- | :--- | :--- |
| Forced Preemption | `SIGSTOP` | Deprive runaway Agents of inference time slices |
| Resource Reclamation | `OOM Killer` | Force-reclaim Agent-occupied memory/KV Cache |
| Permission Revocation | `capset` | Instantly cut off Agent Skill invocation capabilities |
| Core Dump | `coredump` | Export complete Agent state for offline analysis |
| Kernel Breakpoints | `ptrace` | Set breakpoints on Agent reasoning paths |
| Security Isolation | `seccomp` / `AppArmor` | Restrict system operations Agents can execute |

### Compute Neutrality

Through HAL abstraction, LinAIx automatically routes inference requests based on policies:

| Policy | Route Target |
| :--- | :--- |
| Latency First | Local GPU |
| Cost First | Local open-source models (zero Token cost) |
| Quality First | GPT-4 / Claude / strongest models |
| Privacy First | Local inference, data never leaves the domain |
| Failover | Local GPU failure → automatic cloud switch |
| Load Balancing | Local queue overflow → spill to remote cluster |

### Defense in Depth

LinAIx's security model follows **Zero Trust** and **Defense in Depth** principles:

```
┌─────────────────────────────────────────────────────────────────┐
│                    LinAIx Defense in Depth                      │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │  L5 App Layer   • Agent authentication                   │ │
│  │                 • Session management / Token refresh     │ │
│  ├───────────────────────────────────────────────────────────┤ │
│  │  L4 Skill Layer • Permission checks (RBAC/ABAC)         │ │
│  │                 • Input sanitization (Prompt injection)  │ │
│  │                 • Operation auditing                     │ │
│  ├───────────────────────────────────────────────────────────┤ │
│  │  L3 Scheduler   • Inter-Agent resource isolation        │ │
│  │                 • Priority anti-starvation               │ │
│  ├───────────────────────────────────────────────────────────┤ │
│  │  L2 Resource    • Memory/GPU memory quota enforcement   │ │
│  │                 • KV Cache access control                │ │
│  ├───────────────────────────────────────────────────────────┤ │
│  │  L1 HAL Layer   • Model invocation authentication       │ │
│  │                 • API key secure storage                │ │
│  ├───────────────────────────────────────────────────────────┤ │
│  │  Cross-Cutting  • End-to-end encryption (TLS)           │ │
│  │                 • Sensitive data masking                │ │
│  │                 • Compliance auditing (GDPR/SOC2 ready) │ │
│  └───────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
```

---

## 🚧 Current Status

> **⚠️ LinAIx is in early-stage design and development.** We are defining core interfaces and building the kernel prototype.

**Current Focus:**
- [ ] Define subsystem interface specifications (in progress)
- [ ] Implement `no_std` Rust kernel core
- [ ] Develop minimal Scheduler (supporting multi-Agent management)
- [ ] Build HAL stub (supporting local inference)
- [ ] Design security module interfaces (authentication/authorization/audit)
- [ ] Write foundational unit and integration tests

---

## 🛠️ Technology Stack

| Layer | Technology | Rationale |
| :--- | :--- | :--- |
| **Kernel & Core** | Rust (no_std) | Memory safety, zero-cost abstractions, no GC pauses — ideal for systems programming |
| **Agent SDK** | Python / Go (planned) | Python: rich AI ecosystem; Go: high concurrency and clean engineering |
| **IPC** | gRPC / Protobuf | Cross-language support, versioned contracts |
| **Observability** | OpenTelemetry | Vendor-neutral tracing, metrics, and logging standards |
| **Security** | RustCrypto, OPA (Policy Engine) | Cryptographic primitives, fine-grained authorization policies |
| **Build** | Cargo | Rust-native build and dependency management |

---

## 🗺️ Roadmap

| Version | Milestone |
| :--- | :--- |
| **v0.1** | • Schedule 2+ Agents concurrently<br>• Basic Skill API (register/invoke)<br>• Minimal observability (logs, metrics)<br>• HAL local inference support<br>• Basic authentication |
| **v0.5** | • Local + remote hybrid inference<br>• Priority scheduling and preemption<br>• CLI management tool<br>• KV Cache pooling<br>• RBAC permission model |
| **v1.0** | • Kernel-level mandatory intervention (preempt/reclaim/revoke)<br>• Multi-Agent IPC<br>• Persistent memory management<br>• Production-grade stability<br>• Full security audit pipeline |
| **v1.5** | • Adopted by 3+ external projects<br>• Initial community ecosystem<br>• Python/Go SDK official release<br>• Prompt injection defense system |

---

## 🔍 Positioning vs. Existing Projects

| Project | Positioning | How LinAIx Differs |
| :--- | :--- | :--- |
| **Kubernetes** | Container orchestration | Kubernetes manages stateless containers; LinAIx manages stateful Agents (KV Cache, context, inference affinity) |
| **LangChain** | Agent application framework | LangChain is Agent "application layer"; LinAIx is Agent "operating system layer" — lower-level and more general |
| **vLLM** | Inference engine | vLLM is the "CPU"; LinAIx is the "full computer" — integrating scheduling, memory, I/O, and security |
| **Dify** | Low-code AI platform | Dify targets end-users; LinAIx is the "foundation" for building such platforms |
| **AIOS (paper)** | AI OS concept proof | AIOS remains theoretical/academic; LinAIx aims for a production-grade open-source implementation |

---

## 🤝 Contributing

We are looking for:

- **Kernel Engineers**: Experience with operating systems, schedulers, memory management
- **Rust Enthusiasts**: Passionate about systems programming, memory safety, and performance
- **AI Systems Thinkers**: Unique insights into Agent architecture and distributed inference
- **Security Engineers**: Familiar with zero-trust architectures, permission models, and security auditing

**How to Contribute:**
1. Read the [Design Documents](./docs/)
2. Check [Issues](https://github.com/LinAIx/LinAIx/issues) for open tasks
3. Join the discussion (Discord/Matrix links coming soon)

**Development Setup:**
```bash
# Clone the repository
git clone https://github.com/LinAIx/LinAIx.git
cd LinAIx

# Build the kernel
cargo build --release

# Run tests
cargo test

# Run the minimal example
cargo run --example minimal
```

---

## 📄 License

LinAIx is licensed under the **GNU General Public License v2.0**, aligning with the foundational philosophy of Linux.

---

# **LinAIx: Where Linux Meets Intelligence.**

---
