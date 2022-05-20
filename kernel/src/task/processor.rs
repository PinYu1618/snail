use crate::{task::{Thread, ThreadContext}, trap::TrapContext};
use alloc::sync::Arc;
use core::arch::asm;
#[cfg(feature = "up")]
use crate::sync::UPSafeCell;
use super::Process;

pub fn schedule(_switched_thread_cx_ptr: *mut ThreadContext) {
    todo!()
}

pub struct Processor {
    pub current: Option<Arc<Thread>>,
    pub idle_thread_cx: ThreadContext,
}

impl Processor {
    pub fn new() -> Self {
        Self {
            current: None,
            idle_thread_cx: ThreadContext::zero_init(),
        }
    }

    pub fn current(&self) -> Option<Arc<Thread>> {
        self.current.clone()
    }

    pub fn take_current(&mut self) -> Option<Arc<Thread>> {
        self.current.take()
    }

    pub fn idle_thread_cx_mut(&mut self) -> *mut ThreadContext {
        &mut self.idle_thread_cx as *mut _
    }
}

#[cfg(feature = "up")]
pub fn current() -> Option<Arc<Thread>> {
    PROCESSOR.exclusive_access().current()
}

#[cfg(feature = "up")]
pub fn take_current_thread() -> Option<Arc<Thread>> {
    PROCESSOR.exclusive_access().take_current()
}

#[cfg(feature = "up")]
pub fn current_process() -> Arc<Process> {
    current().unwrap().process.upgrade().unwrap()
}

#[cfg(feature = "up")]
pub fn current_user_token() -> usize {
    current().unwrap().user_token()
}

#[cfg(feature = "up")]
pub fn get_current_trap_cx_mut() -> &'static mut TrapContext {
    current().unwrap().trap_cx_mut()
}

#[cfg(feature = "up")]
pub fn current_kstack_top() -> usize {
    current().unwrap().kstack_top()
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

#[cfg(feature = "up")]
lazy_static! {
    pub static ref PROCESSOR: UPSafeCell<Processor> = unsafe { UPSafeCell::new(Processor::new()) };
}

#[naked]
#[link_section = ".text"]
pub unsafe extern "C" fn switch(_current_task_cx_ptr: *mut ThreadContext, _next_task_cx_ptr: *const ThreadContext) {
    asm!(
        "sd sp, 8(a0)",
        "sd ra, 0(a0)",
        "
        sd s0, 16(a0)
        sd s1, 24(a0)
        sd s2, 32(a0)
        sd s3, 40(a0)
        sd s4, 48(a0)
        sd s5, 56(a0)
        sd s6, 64(a0)
        sd s7, 72(a0)
        sd s8, 80(a0)
        sd s9, 88(a0)
        sd s10, 96(a0)
        sd s11, 104(a0)
        ",
        "
        ld ra, 0(a1)
        ld s0, 16(a1)
        ld s1, 24(a1)
        ld s2, 32(a1)
        ld s3, 40(a1)
        ld s4, 48(a1)
        ld s5, 56(a1)
        ld s6, 64(a1)
        ld s7, 72(a1)
        ld s8, 80(a1)
        ld s9, 88(a1)
        ld s10, 96(a1)
        ld s11, 104(a1)
        ",
        "ld sp, 8(a1)",
        "ret",
        options(noreturn)
    )
}
