use crate::{task::{Thread, ThreadContext}, sync::UPSafeCell, trap::TrapContext};
use alloc::sync::Arc;

use super::Process;

pub struct Processor {
    pub current: Option<Arc<Thread>>,
    pub idle_thread_cx: ThreadContext,
}

pub fn current_thread() -> Option<Arc<Thread>> {
    PROCESSOR.exclusive_access().current()
}

pub fn take_current_thread() -> Option<Arc<Thread>> {
    PROCESSOR.exclusive_access().take_current()
}

pub fn current_process() -> Arc<Process> {
    current_thread().unwrap().process.upgrade().unwrap()
}

pub fn current_user_token() -> usize {
    current_thread().unwrap().user_token()
}

pub fn get_current_trap_cx_mut() -> &'static mut TrapContext {
    current_thread().unwrap().trap_cx_mut()
}

pub fn current_kstack_top() -> usize {
    current_thread().unwrap().kstack_top()
}

pub fn schedule(_switched_thread_cx_ptr: *mut ThreadContext) {
    todo!()
}

pub fn suspend_current_and_run_next() {
    todo!()
}

pub fn block_current_and_run_next() {
    todo!()
}

pub fn exit_current_and_run_next(_exit_code: i32) {
    todo!()
}

pub fn add_initproc() {
    todo!()
}

impl Processor {
    pub fn idle_thread_cx_mut(&mut self) -> *mut ThreadContext {
        &mut self.idle_thread_cx as *mut _
    }

    pub fn new() -> Self {
        Self {
            current: None,
            idle_thread_cx: ThreadContext::zero_init(),
        }
    }

    pub fn current(&self) -> Option<Arc<Thread>> {
        self.current.as_ref().map(Arc::clone)
    }

    pub fn take_current(&mut self) -> Option<Arc<Thread>> {
        self.current.take()
    }
}

lazy_static! {
    pub static ref PROCESSOR: UPSafeCell<Processor> = unsafe { UPSafeCell::new(Processor::new()) };
}
