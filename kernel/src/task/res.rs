use crate::{task::Process, mm::PhysPageNr};
use alloc::sync::{Weak, Arc};

pub struct ThreadUserRes {
    pub tid: usize,
    pub ustack_base: usize,
    pub process: Weak<Process>,
}

impl ThreadUserRes {
    pub fn tid(&self) -> usize {
        self.tid
    }

    pub fn ustack_base(&self) -> usize {
        self.ustack_base
    }

    pub fn ustack_top(&self) -> usize {
        use crate::config;
        user_stack_bottom(self.ustack_base(), self.tid()) + config::USTACK_SZ
    }

    pub fn alloc_tid(&mut self) {
        self.tid = self.process.upgrade().unwrap().inner_exclusive_access().alloc_tid();
    }

    pub fn dealloc_tid(&self) {
        let process = self.process.upgrade().unwrap();
        let mut process_inner = process.inner_exclusive_access();
        process_inner.dealloc_tid(self.tid());
    }

    pub fn alloc(&self) {
        use crate::mm::MapPermission;
        use crate::config::{USTACK_SZ, PAGE_SZ};
        let process = self.process.upgrade().unwrap();
        let mut inner = process.inner_exclusive_access();
        // alloc user stack
        let ustack_bottom = user_stack_bottom(self.ustack_base(), self.tid());
        let ustack_top = ustack_bottom + USTACK_SZ;
        inner.memory_set.insert_framed(
            ustack_bottom.into(),
            ustack_top.into(),
            MapPermission::R | MapPermission::W | MapPermission::U,
        );
        // alloc trap context
        let trap_cx_bottom = trap_cx_bottom(self.tid());
        let trap_cx_top = trap_cx_bottom + PAGE_SZ;
        inner.memory_set.insert_framed(
            trap_cx_bottom.into(),
            trap_cx_top.into(),
            MapPermission::R | MapPermission::W,
        );
    }

    pub fn new(_process: Arc<Process>, _ustack_base: usize, _alloc_user_res: bool) -> Self {
        todo!()
    }

    pub fn trap_cx_ppn(&self) -> PhysPageNr {
        let _process = self.process.upgrade().unwrap();
        let _inner = _process.inner_exclusive_access();
        todo!()
    }
}

fn user_stack_bottom(ustack_base: usize, tid: usize) -> usize {
    use crate::config::{PAGE_SZ, USTACK_SZ};
    ustack_base + tid * (PAGE_SZ + USTACK_SZ)
}

fn trap_cx_bottom(tid: usize) -> usize {
    use crate::config::{TRAP_CONTEXT_BASE, PAGE_SZ};
    TRAP_CONTEXT_BASE - tid * PAGE_SZ
}
