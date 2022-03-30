use lazy_static::*;
use alloc::vec::Vec;
use super::{ PhysAddr, PhysPageNr };
use crate::config::MEM_END;
use crate::sync::UPSafeCell;

pub trait FrameAllocator {
    fn new() -> Self;
    fn alloc(&mut self) -> Option<PhysPageNr>;
    fn dealloc(&mut self, ppn: PhysPageNr);
}

pub struct FrameTracker {
    ppn: PhysPageNr,
}

pub struct StackFrameAllocator {
    current: usize,
    end: usize,
    recycled: Vec<usize>,
}

impl FrameTracker {
    pub fn new(ppn: PhysPageNr) -> Self {
        // page cleaning
        let bytes_arr = ppn.bytes_arr();
        for i in bytes_arr {
            *i = 0;
        }
        Self { ppn }
    }

    pub fn ppn(&self) -> PhysPageNr { self.ppn }
}

type FrameAllocatorImpl = StackFrameAllocator;

impl StackFrameAllocator {
    pub fn init(&mut self, l: PhysPageNr, r: PhysPageNr) {
        self.current = l.as_usize();
        self.end = r.as_usize();
    }
}
impl FrameAllocator for StackFrameAllocator {
    fn new() -> Self {
        Self {
            current: 0,
            end: 0,
            recycled: Vec::new(),
        }
    }
    fn alloc(&mut self) -> Option<PhysPageNr> {
        if let Some(ppn) = self.recycled.pop() {
            Some(ppn.into())
        } else {
            if self.current == self.end {
                None
            } else {
                self.current += 1;
                Some((self.current - 1).into())
            }
        }
    }
    fn dealloc(&mut self, ppn: PhysPageNr) {
        let ppn = ppn.as_usize();
        // validity check
        if ppn >= self.current || self.recycled
            .iter()
            .find(|&v| {*v == ppn})
            .is_some() {
            panic!("Frame ppn={:#x} has not been allocated!", ppn);
        }
        // recycle
        self.recycled.push(ppn);
    }
}

lazy_static! {
    pub static ref FRAME_ALLOCATOR: UPSafeCell<FrameAllocatorImpl> = unsafe {
        UPSafeCell::new(FrameAllocatorImpl::new())
    };
}

pub fn init() {
    extern "C" {
        fn ekernel();
    }
    FRAME_ALLOCATOR
        .exclusive_access()
        .init(PhysAddr::from(ekernel as usize).ceil(), PhysAddr::from(MEM_END).floor());
}

pub fn alloc_frame() -> Option<FrameTracker> {
    FRAME_ALLOCATOR
        .exclusive_access()
        .alloc()
        .map(FrameTracker::new)
}