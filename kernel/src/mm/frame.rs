use lazy_static::*;

use alloc::vec::Vec;

use crate::config::MEM_END;
use crate::sync::up::UPSafeCell;

use super::addr::{PhysAddr, PhysPageNr};

pub trait FrameAlloc {
    fn new() -> Self;
    fn alloc(&mut self) -> Option<PhysPageNr>;
    fn dealloc(&mut self, ppn: PhysPageNr);
}

#[derive(Clone, Debug)]
pub struct FrameTracker {
    pub ppn: PhysPageNr,
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
}

impl Drop for FrameTracker {
    fn drop(&mut self) {
        dealloc_frame(self.ppn);
    }
}

type FrameAllocImpl = StackFrameAllocator;

impl StackFrameAllocator {
    pub fn init(&mut self, l: PhysPageNr, r: PhysPageNr) {
        self.current = l.0;
        self.end = r.0;
    }
}
impl FrameAlloc for StackFrameAllocator {
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
        } else if self.current == self.end {
            None
        } else {
            self.current += 1;
            Some((self.current - 1).into())
        }
    }
    fn dealloc(&mut self, ppn: PhysPageNr) {
        let ppn = ppn.as_usize();
        // validity check
        if ppn >= self.current || self.recycled.iter().find(|&v| *v == ppn).is_some() {
            panic!("Frame ppn={:#x} has not been allocated!", ppn);
        }
        // recycle
        self.recycled.push(ppn);
    }
}

lazy_static! {
    pub static ref FRAME_ALLOCATOR: UPSafeCell<FrameAllocImpl> =
        unsafe { UPSafeCell::new(FrameAllocImpl::new()) };
}

pub fn init() {
    extern "C" {
        fn ekernel();
    }
    FRAME_ALLOCATOR.exclusive_access().init(
        PhysAddr::from(ekernel as usize).ceil(),
        PhysAddr::from(MEM_END).floor(),
    );
}

pub fn alloc_frame() -> Option<FrameTracker> {
    FRAME_ALLOCATOR
        .exclusive_access()
        .alloc()
        .map(FrameTracker::new)
}

pub fn dealloc_frame(ppn: PhysPageNr) {
    FRAME_ALLOCATOR.exclusive_access().dealloc(ppn);
}

// testing
#[cfg(dbg)]
pub fn test_frame_allocator() {
    let mut v: Vec<FrameTracker> = Vec::new();
    for i in 0..5 {
        let frame = alloc_frame().unwrap();
        println!("{:?}", frame);
        v.push(frame);
    }
    v.clear();
    for i in 0..5 {
        let frame = alloc_frame().unwrap();
        println!("{:?}", frame);
        v.push(frame);
    }
    drop(v);
    println!("test_frame_allocator passed!");
}
