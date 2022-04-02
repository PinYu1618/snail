use lazy_static::lazy_static;
use alloc::vec::Vec;

use crate::{config::{TRAMPOLINE, KSTACK_SZ, PAGE_SZ}, sync::UPSafeCell, mm::memset::KERNEL_SPACE};

pub struct PidHandle(pub usize);

pub struct KStack {
    pid: usize,
}

struct PidAllocator {
    current: usize,
    recycled: Vec<usize>,
}

impl Drop for PidHandle {
    fn drop(&mut self) {
        PID_ALLOCATOR.exclusive_access().dealloc(self.0);
    }
}

impl KStack {
    pub fn new(pid_handle: &PidHandle) -> Self {
        unimplemented!()
    }
}

impl Drop for KStack {
    fn drop(&mut self) {
        unimplemented!()
    }
}

impl PidAllocator {
    pub fn new() -> Self {
        Self {
            current: 0,
            recycled: Vec::new(),
        }
    }

    pub fn alloc(&mut self) -> PidHandle {
        if let Some(pid) = self.recycled.pop() {
            PidHandle(pid)
        } else {
            self.current += 1;
            PidHandle(self.current - 1)
        }
    }

    pub fn dealloc(&mut self, pid: usize) {
        assert!(pid < self.current);
        assert!(
            self.recycled.iter().find(|ppid| **ppid == pid).is_none(),
            "pid {} has been deallocated!", pid
        );
        self.recycled.push(pid);
    }
}

lazy_static! {
    static ref PID_ALLOCATOR: UPSafeCell<PidAllocator> = unsafe {
        UPSafeCell::new(PidAllocator::new())
    };
}

// return (bottom, top) of a kstack in kspace
pub fn kstack_pos(app_id: usize) -> (usize, usize) {
    let top = TRAMPOLINE - app_id * (KSTACK_SZ + PAGE_SZ);
    let bottom = top - KSTACK_SZ;
    (bottom, top)
}

pub fn alloc_pid() -> PidHandle {
    PID_ALLOCATOR.exclusive_access().alloc()
}