use crate::sync::UPSafeCell;
use alloc::vec::Vec;

pub struct Pid(pub usize);
pub struct KernelStack(pub usize);

pub struct RecycleAllocator {
    current: usize,
    recycled: Vec<usize>,
}

pub fn alloc_pid() -> Pid {
    Pid(PID_ALLOCATOR.exclusive_access().alloc())
}

pub fn alloc_kernel_stack() -> KernelStack {
    use crate::mm::memset::KSPACE;
    use crate::mm::MapPermission;
    let kstack_id = KERNEL_STACK_ALLOCATOR.exclusive_access().alloc();
    let (bottom, top) = kernel_stack_position(kstack_id);
    KSPACE.exclusive_access().insert_framed(
        bottom.into(), 
        top.into(),
        MapPermission::R | MapPermission::W,
    );
    KernelStack(kstack_id)
}

pub fn kernel_stack_position(kstack_id: usize) -> (usize, usize) {
    use crate::config;
    let top = config::TRAMPOLINE - kstack_id * (config::KSTACK_SZ + config::PAGE_SZ);
    let bottom = top - config::KSTACK_SZ;
    (bottom, top)
}

pub fn kernel_stack_top(kstack_id: usize) -> usize {
    kernel_stack_position(kstack_id).1
}

pub fn kernel_stack_bottom(kstack_id: usize) -> usize {
    kernel_stack_position(kstack_id).0
}

impl RecycleAllocator {
    pub fn new() -> Self {
        Self {
            current: 0,
            recycled: Vec::new(),
        }
    }

    pub fn alloc(&mut self) -> usize {
        if let Some(id) = self.recycled.pop() {
            id
        } else {
            self.current += 1;
            self.current - 1
        }
    }

    pub fn dealloc(&mut self, id: usize) {
        assert!(id < self.current);
        assert!(
            !self.recycled.iter().any(|iid| *iid == id),
            "pid/tid {} has been deallocated!",
            id
        );
        self.recycled.push(id);
    }
}

impl Default for RecycleAllocator {
    fn default() -> Self {
        Self::new()
    }
}

impl Pid {
    pub fn get(&self) -> usize {
        self.0
    }
}

impl KernelStack {
    pub fn id(&self) -> usize {
        self.0
    }

    pub fn top(&self) -> usize {
        kernel_stack_top(self.id())
    }
}

impl Drop for KernelStack {
    fn drop(&mut self) {
        use crate::mm::VirtAddr;
        use crate::mm::memset::KSPACE;
        let bottom  = kernel_stack_bottom(self.id());
        let bottom_va: VirtAddr = bottom.into();
        KSPACE.exclusive_access().remove_area(bottom_va.into());
    }
}

lazy_static! {
    static ref PID_ALLOCATOR: UPSafeCell<RecycleAllocator> = unsafe { UPSafeCell::new(RecycleAllocator::default()) };
    static ref KERNEL_STACK_ALLOCATOR: UPSafeCell<RecycleAllocator> = unsafe { UPSafeCell::new(RecycleAllocator::default()) };
}
