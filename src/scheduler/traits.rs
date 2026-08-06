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

//! # 调度器接口定义
//!
//! 参考 Linux 调度器接口设计。
//!
//! ## 核心接口
//! - [`Scheduler`]：调度器主接口
//! - [`SchedClass`]：调度类接口 (类似 Linux 的 sched_class)
//!
//! ## Version
//! 0.1.0
//!
//! ## Author
//! VmFree <vmfree@example.com>
//!
//! ## Date
//! 2026-08-01

use crate::task::{TaskId, SubTaskId, SubTask, SubTaskStatus};
use crate::scheduler::types::*;
use crate::scheduler::error::SchedulerError;

/// 调度器主接口
///
/// 参考 Linux 的调度器设计，核心职责：
/// 1. 管理运行队列 (runqueue) — 存放就绪的 SubTask
/// 2. 管理等待队列 (waitqueue) — 存放等待事件的 SubTask
/// 3. 选择下一个要运行的 SubTask (pick_next)
/// 4. 处理抢占 (preemption)
/// 5. 更新调度统计
pub trait Scheduler: Send + Sync {
    // ===== 运行队列 (Runqueue) =====

    /// 将 SubTask 加入运行队列
    ///
    /// # 前置条件
    /// - SubTask 状态为 Ready
    /// - SubTask 所属 Task 配额充足
    ///
    /// # 参考 Linux
    /// activate_task() / enqueue_task()
    fn enqueue_task(&mut self, subtask_id: SubTaskId) -> Result<(), SchedulerError>;

    /// 从运行队列中移除 SubTask
    ///
    /// # 参考 Linux
    /// deactivate_task() / dequeue_task()
    fn dequeue_task(&mut self, subtask_id: &SubTaskId) -> Result<(), SchedulerError>;

    /// 选择下一个要执行的 SubTask
    ///
    /// # 参考 Linux
    /// pick_next_task()
    fn pick_next_task(&mut self) -> Result<Option<SubTask>, SchedulerError>;

    /// 获取运行队列统计信息
    fn get_runqueue_stats(&self) -> RunQueueStats;

    /// 获取运行队列中的所有 SubTask
    fn get_runqueue_tasks(&self) -> alloc::vec::Vec<SubTaskId>;

    /// 检查 SubTask 是否在运行队列中
    fn is_in_runqueue(&self, subtask_id: &SubTaskId) -> bool;

    // ===== 等待队列 (Waitqueue) =====

    /// 将 SubTask 加入等待队列
    ///
    /// # 用途
    /// SubTask 因等待 IPC、资源、依赖而阻塞时调用
    ///
    /// # 参考 Linux
    /// wait_event() / add_wait_queue()
    fn wait_event(
        &mut self,
        subtask_id: SubTaskId,
        reason: WaitReason,
        timeout: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<(), SchedulerError>;

    /// 唤醒等待队列中的 SubTask
    ///
    /// # 参考 Linux
    /// wake_up() / wake_up_process()
    fn wake_up(&mut self, subtask_id: &SubTaskId) -> Result<(), SchedulerError>;

    /// 唤醒所有等待特定条件的 SubTask
    ///
    /// # 参考 Linux
    /// wake_up_all() / __wake_up()
    fn wake_up_all(&mut self, reason: WaitReason) -> Result<usize, SchedulerError>;

    /// 获取等待队列统计信息
    fn get_waitqueue_stats(&self) -> WaitQueueStats;

    /// 获取等待队列中的所有 SubTask
    fn get_waitqueue_tasks(&self) -> alloc::vec::Vec<WaitQueueEntry>;

    /// 检查 SubTask 是否在等待队列中
    fn is_in_waitqueue(&self, subtask_id: &SubTaskId) -> bool;

    // ===== 调度控制 =====

    /// 执行一次调度循环
    ///
    /// 1. 从运行队列中 pick_next_task
    /// 2. 分配执行资源
    /// 3. 执行 SubTask (调用 ModelExecutor)
    /// 4. 更新 SubTask 状态
    ///
    /// # 参考 Linux
    /// schedule()
    fn schedule_once(&mut self) -> Result<Option<SubTask>, SchedulerError>;

    /// 启动调度循环 (阻塞)
    ///
    /// # 参考 Linux
    /// schedule() loop
    fn run(&mut self) -> Result<(), SchedulerError>;

    /// 暂停调度
    fn pause(&mut self) -> Result<(), SchedulerError>;

    /// 恢复调度
    fn resume(&mut self) -> Result<(), SchedulerError>;

    // ===== 抢占控制 =====

    /// 抢占当前运行的 SubTask
    ///
    /// # 参考 Linux
    /// preempt()
    fn preempt(&mut self) -> Result<(), SchedulerError>;

    /// 检查是否可以抢占
    ///
    /// # 参考 Linux
    /// preemptible()
    fn can_preempt(&self) -> bool;

    /// 获取当前运行的 SubTask
    fn current_task(&self) -> Option<SubTask>;

    // ===== 调度类管理 =====

    /// 注册调度类
    ///
    /// # 参考 Linux
    /// sched_class 注册
    fn register_sched_class(&mut self, class: &'static dyn SchedClass) -> Result<(), SchedulerError>;

    /// 获取 SubTask 对应的调度类
    fn get_sched_class(&self, subtask_id: &SubTaskId) -> Option<&'static dyn SchedClass>;

    // ===== 资源检查 =====

    /// 检查 Task 配额是否允许执行 SubTask
    fn check_task_quota(&self, task_id: &TaskId) -> Result<(), SchedulerError>;

    /// 检查资源是否充足
    fn check_resources(&self, subtask_id: &SubTaskId) -> Result<(), SchedulerError>;

    // ===== 统计与状态 =====

    /// 获取调度器状态
    fn get_status(&self) -> SchedulerStatus;

    /// 获取调度器统计信息
    fn get_stats(&self) -> SchedulerStats;

    /// 获取所有就绪的 SubTask
    fn get_ready_tasks(&self) -> alloc::vec::Vec<SubTaskId>;

    /// 获取所有阻塞的 SubTask (等待队列)
    fn get_blocked_tasks(&self) -> alloc::vec::Vec<WaitQueueEntry>;
}

/// 调度类接口
///
/// 参考 Linux 的 sched_class 结构
///
/// 每个调度类实现不同的调度策略：
/// - Realtime: FIFO/RR 实时调度
/// - Fair: CFS 完全公平调度
/// - Idle: 空闲调度
pub trait SchedClass: Send + Sync {
    /// 调度类类型
    fn class_type(&self) -> SchedClassType;

    /// 调度类优先级
    fn priority(&self) -> SchedClassPriority;

    /// 将 SubTask 加入调度类的队列
    fn enqueue(&mut self, subtask: &SubTask) -> Result<(), SchedulerError>;

    /// 从调度类的队列中移除 SubTask
    fn dequeue(&mut self, subtask_id: &SubTaskId) -> Result<(), SchedulerError>;

    /// 选择下一个要执行的 SubTask
    fn pick_next(&mut self) -> Result<Option<SubTask>, SchedulerError>;

    /// 抢占当前运行的 SubTask
    fn preempt(&mut self) -> Result<Option<SubTask>, SchedulerError>;

    /// 获取调度类中的所有 SubTask
    fn get_all_tasks(&self) -> alloc::vec::Vec<SubTaskId>;

    /// 获取调度类名称
    fn name(&self) -> &'static str;

    /// 检查 SubTask 是否属于此调度类
    fn belongs_to(&self, subtask: &SubTask) -> bool;
}