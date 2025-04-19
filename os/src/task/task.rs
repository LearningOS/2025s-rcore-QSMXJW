//! Types related to task management

use super::TaskContext;

/// The task control block (TCB) of a task.
#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    /// The task status in it's lifecycle
    pub task_status: TaskStatus,
    /// The task context
    pub task_cx: TaskContext,
}

/// The status of a task
#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    /// uninitialized
    UnInit,
    /// ready to run
    Ready,
    /// running
    Running,
    /// exited
    Exited,
}


/// The info of task
#[derive(Copy, Clone)]
pub struct  TaskInfo {
    /// sys_call_counter
    pub sys_call_counter: [isize; 1024]
}

impl TaskInfo {
    /// 构造函数
    pub fn new() -> Self {
        TaskInfo { sys_call_counter: [0; 1024] }
    }
}