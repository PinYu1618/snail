use crate::mm::{memset::KSPACE, MapPermission, VirtAddr};
use crate::{
    config::{KSTACK_SZ, PAGE_SZ, TRAMPOLINE},
    sync::up::UPSafeCell,
};
use alloc::vec::Vec;

lazy_static! {
    static ref PID_ALLOCATOR: UPSafeCell<PidAllocator> =
        unsafe { UPSafeCell::new(PidAllocator::default()) };
}

// wrap pid in an struct so we can automatically recycle it
#[derive(Clone)]
pub struct PidHandle(pub usize);

#[derive(Clone)]
pub struct KStack {
    pid: usize,
}

pub struct PidAllocator {
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
        let pid = pid_handle.0;
        let (kbp, ksp) = kstack_pos(pid);
        KSPACE.exclusive_access().insert_framed_area(
            kbp.into(),
            ksp.into(),
            MapPermission::R | MapPermission::W,
        );
        Self { pid: pid_handle.0 }
    }

    pub fn push<T>(&self, val: T) -> *mut T
    where
        T: Sized,
    {
        let ksp = self.top();
        let ptr_mut = (ksp - core::mem::size_of::<T>()) as *mut T;
        unsafe {
            *ptr_mut = val;
        }
        ptr_mut
    }

    pub fn top(&self) -> usize {
        let (_, ksp) = kstack_pos(self.pid);
        ksp
    }
}

impl Drop for KStack {
    fn drop(&mut self) {
        let (kbp, _) = kstack_pos(self.pid);
        let kbp_va: VirtAddr = kbp.into();
        KSPACE.exclusive_access().remove_area(kbp_va.into());
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
            !self.recycled.iter().any(|ppid| *ppid == pid),
            "pid {} has been deallocated!",
            pid
        );
        self.recycled.push(pid);
    }

    pub fn alloc_pid() -> PidHandle {
        PID_ALLOCATOR.exclusive_access().alloc()
    }
}

impl Default for PidAllocator {
    fn default() -> Self {
        Self::new()
    }
}

// return (bottom, top) of a kstack in kspace
pub fn kstack_pos(app_id: usize) -> (usize, usize) {
    let top = TRAMPOLINE - app_id * (KSTACK_SZ + PAGE_SZ);
    let bottom = top - KSTACK_SZ;
    (bottom, top)
}
