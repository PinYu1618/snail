use lazy_static::lazy_static;

use alloc::{collections::VecDeque, sync::Arc};

use crate::sync::up::UPSafeCell;

use super::process::ProcessCtrlBlock;

pub struct TaskCtrller {
    ready_queue: VecDeque<Arc<ProcessCtrlBlock>>,
}

// a simple FIFO scheduler
impl TaskCtrller {
    pub fn new() -> Self {
        Self {
            ready_queue: VecDeque::new(),
        }
    }

    pub fn add(&mut self, task: Arc<ProcessCtrlBlock>) {
        self.ready_queue.push_back(task);
    }

    pub fn fetch(&mut self) -> Option<Arc<ProcessCtrlBlock>> {
        self.ready_queue.pop_front()
    }
}

lazy_static! {
    pub static ref TASK_CTRLLER: UPSafeCell<TaskCtrller> =
        unsafe { UPSafeCell::new(TaskCtrller::new()) };
}

pub fn add_task(task: Arc<ProcessCtrlBlock>) {
    TASK_CTRLLER.exclusive_access().add(task);
}

pub fn fetch_task() -> Option<Arc<ProcessCtrlBlock>> {
    TASK_CTRLLER.exclusive_access().fetch()
}
