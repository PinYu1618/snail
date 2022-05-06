use crate::{task::Process, sync::UPSafeCell, mm::PhysPageNr, task::{ThreadContext, ThreadUserRes, KernelStack}, trap::TrapContext};
use alloc::sync::{Weak, Arc};
use core::cell::RefMut;

pub struct Thread {
    // immutable
    pub process: Weak<Process>,
    pub kstack: KernelStack,
    // mutable
    inner: UPSafeCell<ThreadInner>,
}

pub struct ThreadInner {
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

impl Thread {
    pub fn kstack_top(&self) -> usize {
        self.kstack.top()
    }

    pub fn inner_exclusive_access(&self) -> RefMut<'_, ThreadInner> {
        self.inner.exclusive_access()
    }

    pub fn trap_cx_mut(&self) -> &'static mut TrapContext {
        let inner = self.inner_exclusive_access();
        inner.trap_cx_mut()
    }

    pub fn user_token(&self) -> usize {
        let process = self.process.upgrade().unwrap();
        let inner = process.inner_exclusive_access();
        inner.user_token()
    }

    pub fn new(process: Arc<Process>, ustack_base: usize, alloc_user_res: bool) -> Self {
        use crate::task::alloc_kernel_stack;
        let res = ThreadUserRes::new(Arc::clone(&process), ustack_base, alloc_user_res);
        let _trap_cx_ppn = res.trap_cx_ppn();
        let kstack = alloc_kernel_stack();
        let _ksp = kstack.top();
        todo!()
    }
}

impl ThreadInner {
    pub fn status(&self) -> &ThreadStatus {
        &self.thread_status
    }

    pub fn trap_cx_mut(&self) -> &'static mut TrapContext {
        self.trap_cx_ppn.get_mut()
    }
}
