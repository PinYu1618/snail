use lazy_static::lazy_static;

use alloc::sync::Arc;

use crate::sync::up::UPSafeCell;

use super::{context::ProcessContext, ctrl::fetch_task, process::ProcessCtrlBlock};

pub struct Processor {
    current: Option<Arc<ProcessCtrlBlock>>,
    idle_task_cx: ProcessContext,
}

impl Processor {
    pub fn new() -> Self {
        Self {
            current: None,
            idle_task_cx: ProcessContext::zero_init(),
        }
    }

    pub fn idle_task_cx_ptr(&mut self) -> *mut ProcessContext {
        &mut self.idle_task_cx as *mut _
    }

    pub fn take_current(&mut self) -> Option<Arc<ProcessCtrlBlock>> {
        self.current.take()
    }

    pub fn current(&self) -> Option<Arc<ProcessCtrlBlock>> {
        self.current.as_ref().map(|task| Arc::clone(task))
    }
}

lazy_static! {
    pub static ref PROCESSOR: UPSafeCell<Processor> = unsafe { UPSafeCell::new(Processor::new()) };
}

pub fn take_current_process() -> Option<Arc<ProcessCtrlBlock>> {
    PROCESSOR.exclusive_access().take_current()
}

pub fn current_process() -> Option<Arc<ProcessCtrlBlock>> {
    PROCESSOR.exclusive_access().current()
}

pub fn current_user_token() -> Option<usize> {
    if let Some(pcb) = current_process() {
        let token = pcb.inner_exclusive_access().user_token();
        Some(token)
    } else {
        None
    }
}

pub fn run_tasks() {
    loop {
        let mut processor = PROCESSOR.exclusive_access();
        if let Some(task) = fetch_task() {
            let idle_task_cx_ptr = processor.idle_task_cx_ptr();

            // access coming task pcb exclusively
            let mut pcb_inner = task.inner_exclusive_access();
        }
    }
    unimplemented!()
}

pub fn schedule(process_cx_ptr: *mut ProcessContext) {
    unimplemented!()
}
