use bitflags::*;
use alloc::vec;
use alloc::vec::Vec;
use super::{ PhysPageNr, FrameTracker, alloc_frame };

bitflags! {
    pub struct PTEFlags: u8 {
        const V = 1 << 0;   // Valid, 1 = valid
        const R = 1 << 1;   // Read
        const W = 1 << 2;   // Write
        const X = 1 << 3;   // eXecute
        const U = 1 << 4;   // User
        const G = 1 << 5;   // (dont know)
        const A = 1 << 6;   // Accessed
        const D = 1 << 7;   // Dirty
    }
}

pub struct PageTable {
    root_ppn: PhysPageNr,
    frames: Vec<FrameTracker>,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PageTableEntry {
    pub bits: usize,
}

pub struct UserBuffer {
    pub buffers: Vec<&'static mut [u8]>,
}

impl PageTable {
    pub fn new() -> Self {
        let frame = alloc_frame().unwrap();
        PageTable {
            root_ppn: frame.ppn,
            frames: vec![frame],
        }
    }
}

impl PageTableEntry {
    pub fn new(ppn: PhysPageNr, flags: PTEFlags) -> Self {
        PageTableEntry {
            bits: ppn.as_usize() << 10 | flags.bits as usize,
        }
    }
    pub fn empty() -> Self {
        PageTableEntry {
            bits: 0,
        }
    }
    pub fn pnn(&self) -> PhysPageNr {
        (self.bits >> 10 & ((1_usize << 44) - 1)).into()
    }
    pub fn flags(&self) -> PTEFlags {
        PTEFlags::from_bits(self.bits as u8).unwrap()
    }
    pub fn is_valid(&self) -> bool {
        (self.flags() & PTEFlags::V) != PTEFlags::empty()
    }
}

impl UserBuffer {
    pub fn new(buffers: Vec<&'static mut [u8]>) -> Self {
        Self { buffers }
    }
    pub fn len(&self) -> usize {
        let mut total: usize = 0;
        for b in self.buffers.iter() {
            total += b.len();
        }
        total
    }
}