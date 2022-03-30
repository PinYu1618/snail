use bitflags::*;
use lazy_static::*;
use log::info;
use alloc::sync::Arc;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use super::{ VirtAddr, VirtPageNr, FrameTracker, PageTable, VPNRange, PhysPageNr, alloc_frame };
use crate::sync::UPSafeCell;

extern "C" {
    fn stext();
    fn etext();
    fn srodata();
    fn erodata();
    fn sdata();
    fn edata();
    fn sbss();
    fn ebss();
    fn ekernel();
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum MapType {
    Identical,
    Framed,
}

bitflags! {
    pub struct MapPermission: u8 {
        const R = 1 << 1;
        const W = 1 << 2;
        const X = 1 << 3;
        const U = 1 << 4;
    }
}

pub struct MapArea {
    vpn_range: VPNRange,
    data_frames: BTreeMap<VirtPageNr, FrameTracker>,
    map_type: MapType,
    map_perm: MapPermission,
}

pub struct MemorySet {
    page_table: PageTable,
    areas: Vec<MapArea>,
}

impl MapArea {
    pub fn new(sva: VirtAddr, eva: VirtAddr, map_type: MapType, map_perm: MapPermission) -> Self {
        let svpn: VirtPageNr = sva.floor();
        let evpn: VirtPageNr = eva.ceil();
        Self {
           vpn_range: VPNRange::new(svpn, evpn),
           data_frames: BTreeMap::new(),
           map_type,
           map_perm,
       } 
    }

    pub fn map_one(&mut self, pt: PageTable, vpn: VirtPageNr) {
        let ppn: PhysPageNr;
        match self.map_type {
            MapType::Identical => {
                ppn = PhysPageNr::from(vpn.as_usize());
            },
            MapType::Framed => {
                let frame = alloc_frame().unwrap();
                ppn = frame.ppn();
                self.data_frames.insert(vpn, frame);
            }
        }
    }
}

impl MemorySet {
    pub fn new_bare() -> Self {
        Self {
            page_table: PageTable::new(),
            areas: Vec::new(),
        }
    }

    // without kernel stack
    pub fn new_kernel() -> Self {
        let mut memset = Self::new_bare();
        
        // map kernel sections
        info!("Mapping kernel sections:");
        info!(".text [{:#x}, {:#x})", stext as usize, etext as usize);
        info!(".rodata [{:#x}, {:#x})", srodata as usize, erodata as usize);
        info!(".data [{:#x}, {:#x})", sdata as usize, edata as usize);
        info!(".bss [{:#x}, {:#x})", sbss as usize, ebss as usize);

        info!("Mapping .text section");

        memset
    }

    pub fn init(&self) {}
}

lazy_static!{
    pub static ref KERNEL_SPACE: Arc<UPSafeCell<MemorySet>>
        = Arc::new(
            unsafe {
                UPSafeCell::new(
                    MemorySet::new_kernel()
                )
            }
        );
}