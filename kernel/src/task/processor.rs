use alloc::sync::Arc;
use lazy_static::lazy_static;

use crate::sync::UPSafeCell;

use super::{task::TaskCtrlBlock, context::TaskContext};

pub struct Processor {
    current: Option<Arc<TaskCtrlBlock>>,
    idle_task_cx: TaskContext,
}

impl Processor {
    pub fn new() -> Self {
        Self {
            current: None,
            idle_task_cx: TaskContext::zero_init(),
        }
    }

    pub fn current(&self) -> Option<Arc<TaskCtrlBlock>> {
        self.current.as_ref().map(|task| Arc::clone(task))
    }
}

lazy_static! {
    pub static ref PROCESSOR: UPSafeCell<Processor> = unsafe {
        UPSafeCell::new(Processor::new())
    };
}