use bitflags::*;
use alloc::vec;
use alloc::vec::Vec;
use super::{addr::{ VirtAddr, PhysAddr, VirtPageNr, PhysPageNr }, frame::{FrameTracker, alloc_frame}};

bitflags! {
    pub struct PTEFlags: u8 {
        const V = 1 << 0;    // Valid, 1 = valid
        const R = 1 << 1;    // Read
        const W = 1 << 2;    // Write
        const X = 1 << 3;    // eXecute
        const U = 1 << 4;    // User
        const G = 1 << 5;    // (dont know)
        const A = 1 << 6;    // Accessed
        const D = 1 << 7;    // Dirty
    }
}

#[derive(Clone, Debug)]
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
            root_ppn: frame.ppn(),
            frames: vec![frame],
        }
    }

    pub fn token(&self) -> usize {
        8_usize << 60 | self.root_ppn.as_usize()
    }

    pub fn translate(&self, vpn: VirtPageNr) -> Option<PageTableEntry> {
        self.find_pte(vpn).map(|pte| pte.clone())
    }

    pub fn translate_va(&self, va: VirtAddr) -> Option<PhysAddr> {
        self.find_pte(va.clone().floor()).map(|pte| {
            let aligned_pa: PhysAddr = pte.ppn().into();
            let offset = va.page_offset();
            let aligned_pa_usize: usize = aligned_pa.into();
            (aligned_pa_usize + offset).into()
        })
    }

    pub fn map(&mut self, vpn: VirtPageNr, ppn: PhysPageNr, flags: PTEFlags) {
        let pte = self.find_pte_or_create(vpn).unwrap();
        assert!(!pte.is_valid(), "vpn {:?} is mapped before mapping", vpn);
        *pte = PageTableEntry::new(ppn, flags | PTEFlags::V);
    }

    pub fn unmap(&mut self, vpn: VirtPageNr) {
        let pte = self.find_pte(vpn).unwrap();
        assert!(pte.is_valid(), "vpn {:?} is invalid before unmapping", vpn);
        *pte = PageTableEntry::empty();
    }

    fn find_pte_or_create(&mut self, vpn: VirtPageNr) -> Option<&mut PageTableEntry> {
        let idxs = vpn.indexes();
        let mut ppn = self.root_ppn;
        let mut res: Option<&mut PageTableEntry> = None;
        for i in 0..3 {
            let pte = &mut ppn.pte_arr()[idxs[i]];
            if i == 2 {
                res = Some(pte);
                break;
            }
            if !pte.is_valid() {
                let frame = alloc_frame().unwrap();
                *pte = PageTableEntry::new(frame.ppn(), PTEFlags::V);
                self.frames.push(frame);
            }
            ppn = pte.ppn();
        }
        res
    }

    fn find_pte(&self, vpn: VirtPageNr) -> Option<&mut PageTableEntry> {
        let idxs = vpn.indexes();
        let mut ppn = self.root_ppn;
        let mut res: Option<&mut PageTableEntry> = None;
        for i in 0..3 {
            let pte = &mut ppn.pte_arr()[idxs[i]];
            if i == 2 {
                res = Some(pte);
                break;
            }
            if !pte.is_valid() {
                return None;
            }
            ppn = pte.ppn();
        }
        res
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

    pub fn ppn(&self) -> PhysPageNr {
        (self.bits >> 10 & ((1_usize << 44) - 1)).into()
    }

    pub fn flags(&self) -> PTEFlags {
        PTEFlags::from_bits(self.bits as u8).unwrap()
    }

    pub fn is_valid(&self) -> bool {
        (self.flags() & PTEFlags::V) != PTEFlags::empty()
    }

    pub fn is_readable(&self) -> bool {
        (self.flags() & PTEFlags::R) != PTEFlags::empty()
    }

    pub fn is_writable(&self) -> bool {
        (self.flags() & PTEFlags::W) != PTEFlags::empty()
    }

    pub fn is_executable(&self) -> bool {
        (self.flags() & PTEFlags::X) != PTEFlags::empty()
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