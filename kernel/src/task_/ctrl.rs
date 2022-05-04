use super::task::TaskCtrlBlock;
use crate::sync::UPSafeCell;
use alloc::{collections::VecDeque, sync::Arc};

lazy_static! {
    static ref TASK_CTRLLER: UPSafeCell<TaskCtrller> =
        unsafe { UPSafeCell::new(TaskCtrller::default()) };
}

pub struct TaskCtrller {
    ready_queue: VecDeque<Arc<TaskCtrlBlock>>,
}

// a simple FIFO scheduler
impl TaskCtrller {
    pub fn new() -> Self {
        Self {
            ready_queue: VecDeque::new(),
        }
    }

    pub fn add(&mut self, task: Arc<TaskCtrlBlock>) {
        self.ready_queue.push_back(task);
    }

    pub fn fetch(&mut self) -> Option<Arc<TaskCtrlBlock>> {
        self.ready_queue.pop_front()
    }

    pub fn add_task(task: Arc<TaskCtrlBlock>) {
        TASK_CTRLLER.exclusive_access().add(task);
    }

    pub fn fetch_task() -> Option<Arc<TaskCtrlBlock>> {
        TASK_CTRLLER.exclusive_access().fetch()
    }
}

impl Default for TaskCtrller {
    fn default() -> Self {
        Self::new()
    }
}
