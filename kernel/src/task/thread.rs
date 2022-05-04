use crate::{task::ProcessCtrlBlock, sync::UPSafeCell, mm::PhysPageNr, task::{ThreadContext, ThreadUserRes, KernelStack}, trap::TrapContext};
use alloc::sync::{Weak, Arc};
use core::cell::RefMut;

pub struct ThreadCtrlBlock {
    // immutable
    pub process: Weak<ProcessCtrlBlock>,
    pub kstack: KernelStack,
    // mutable
    inner: UPSafeCell<ThreadCtrlBlockInner>,
}

pub struct ThreadCtrlBlockInner {
    pub res: Option<ThreadUserRes>,
    trap_cx_ppn: PhysPageNr,
    pub thread_cx: ThreadContext,
    thread_status: ThreadStatus,
    pub exit_code: Option<i32>,
}

#[derive(PartialEq, Eq)]
pub enum ThreadStatus {
    Ready,
    Running,
    Blocking,
}

impl ThreadCtrlBlock {
    pub fn get_kstack_top(&self) -> usize {
        self.kstack.get_top()
    }

    pub fn inner_exclusive_access(&self) -> RefMut<'_, ThreadCtrlBlockInner> {
        self.inner.exclusive_access()
    }

    pub fn get_trap_cx_mut(&self) -> &'static mut TrapContext {
        let inner = self.inner_exclusive_access();
        inner.get_trap_cx_mut()
    }

    pub fn get_user_token(&self) -> usize {
        let process = self.process.upgrade().unwrap();
        let inner = process.inner_exclusive_access();
        inner.get_user_token()
    }

    pub fn new(process: Arc<ProcessCtrlBlock>, ustack_base: usize, alloc_user_res: bool) -> Self {
        use crate::task::alloc_kernel_stack;
        let res = ThreadUserRes::new(Arc::clone(&process), ustack_base, alloc_user_res);
        let _trap_cx_ppn = res.get_trap_cx_ppn();
        let kstack = alloc_kernel_stack();
        let _ksp = kstack.get_top();
        todo!()
    }
}

impl ThreadCtrlBlockInner {
    pub fn status(&self) -> &ThreadStatus {
        &self.thread_status
    }

    pub fn get_trap_cx_mut(&self) -> &'static mut TrapContext {
        self.trap_cx_ppn.get_mut()
    }
}
