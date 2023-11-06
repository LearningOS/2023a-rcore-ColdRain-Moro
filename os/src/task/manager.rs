//!Implementation of [`TaskManager`]

use super::TaskControlBlock;
use crate::sync::UPSafeCell;
use alloc::{sync::Arc, collections::VecDeque};
use lazy_static::*;
///A array of `TaskControlBlock` that is thread-safe
pub struct TaskManager {
    ready_queue: VecDeque<Arc<TaskControlBlock>>,
}

/// A simple FIFO scheduler.
impl TaskManager {
    ///Creat an empty TaskManager
    pub fn new() -> Self {
        Self {
            ready_queue: VecDeque::new(),
        }
    }
    /// Add process back to ready queue
    pub fn add(&mut self, task: Arc<TaskControlBlock>) {
        for (i, other_task) in self.ready_queue.iter().enumerate() {
            let other_task_stride = other_task.inner_exclusive_access().stride;
            let current_task_stride = task.inner_exclusive_access().stride;
            // 从小到大排序
            if other_task_stride > current_task_stride {
                self.ready_queue.insert(i, task);
                return;
            }
        }
        self.ready_queue.push_back(task);
    }
    /// Take a process out of the ready queue
    pub fn fetch(&mut self) -> Option<Arc<TaskControlBlock>> {
        // let mut min_idx = 0;
        // let mut min_stride = self.ready_queue[0].inner_exclusive_access().stride;
        // for (idx ,task) in self.ready_queue.iter().enumerate() {
        //     let inner = task.inner_exclusive_access();
        //     if inner.task_status == TaskStatus::Ready {
        //         let v = inner.stride;
        //         min_stride = cmp::min(min_stride, v);
        //         if v == min_stride {
        //             min_idx = idx;
        //         }
        //     }
        // }
        // let mut task = self.ready_queue.remove(min_idx);
        // if let Some(task) = &mut task {
        //     let stride = task.inner_exclusive_access().stride;
        //     let pass = task.inner_exclusive_access().pass;
        //     task.inner_exclusive_access().stride = stride + pass;
        // }
        // task
        self.ready_queue.pop_front()
    }
}

lazy_static! {
    /// TASK_MANAGER instance through lazy_static!
    pub static ref TASK_MANAGER: UPSafeCell<TaskManager> =
        unsafe { UPSafeCell::new(TaskManager::new()) };
}

/// Add process to ready queue
pub fn add_task(task: Arc<TaskControlBlock>) {
    //trace!("kernel: TaskManager::add_task");
    TASK_MANAGER.exclusive_access().add(task);
}

/// Take a process out of the ready queue
pub fn fetch_task() -> Option<Arc<TaskControlBlock>> {
    //trace!("kernel: TaskManager::fetch_task");
    TASK_MANAGER.exclusive_access().fetch()
}
