use bitflags::*;
use lazy_static::*;
use log::info;
use alloc::sync::Arc;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use riscv::register::satp;
use super::{ VirtAddr, VirtPageNr, FrameTracker, PageTable, VPNRange, PhysPageNr, alloc_frame, page::PTEFlags };
use crate::{sync::UPSafeCell, config::{PAGE_SZ, MEM_END, MMIO}, mm::addr::Step};
use core::arch::asm;

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

    // data: start-aligned but maybe with shorter length
    // assume that all frames were cleared before
    pub fn copy_data(&mut self, pt: &mut PageTable, data: &[u8]) {
        assert_eq!(self.map_type, MapType::Framed);
        let mut start: usize = 0;
        let mut current_vpn = self.vpn_range.start();
        let len = data.len();
        loop {
            let src = &data[start..len.min(start + PAGE_SZ)];
            let dst = &mut pt
                .translate(current_vpn)
                .unwrap()
                .ppn()
                .bytes_arr()[..src.len()];
            dst.copy_from_slice(src);
            start += PAGE_SZ;
            if start >= len {
                break;
            }
            current_vpn.step();
        }
    }

    pub fn map(&mut self, pt: &mut PageTable) {
        for vpn in self.vpn_range {
            self.map_one(pt, vpn);
        }
    }

    pub fn map_one(&mut self, pt: &mut PageTable, vpn: VirtPageNr) {
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
        let pte_flags = PTEFlags::from_bits(self.map_perm.bits()).unwrap();
        pt.map(vpn, ppn, pte_flags);
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

        info!("Mapping .text section...");
        memset.push(
            MapArea::new(
                (stext as usize).into(),
                (etext as usize).into(),
                MapType::Identical,
                MapPermission::R | MapPermission::X,
            ),
            None,
        );

        info!("Mapping .rodata section...");
        memset.push(
            MapArea::new(
                (srodata as usize).into(),
                (erodata as usize).into(),
                MapType::Identical,
                MapPermission::R,
            ),
            None,
        );

        info!("Mapping .data section...");
        memset.push(
            MapArea::new(
                (sdata as usize).into(),
                (edata as usize).into(),
                MapType::Identical,
                MapPermission::R | MapPermission::W,
            ),
            None,
        );

        info!("Mapping .bss section...");
        memset.push(
            MapArea::new(
                (sbss as usize).into(),
                (ebss as usize).into(),
                MapType::Identical,
                MapPermission::R | MapPermission::W,
            ),
            None,
        );        

        info!("Mapping physical memory...");
        memset.push(
            MapArea::new(
                (ekernel as usize).into(),
                MEM_END.into(),
                MapType::Identical,
                MapPermission::R | MapPermission::W,
            ),
            None,
        );

        info!("Mapping mem mapped IO...");
        for pair in MMIO {
            memset.push(
                MapArea::new(
                    (*pair).0.into(),
                    ((*pair).0 + (*pair).1).into(),
                    MapType::Identical,
                    MapPermission::R | MapPermission::W,
                ),
                None,
            );
        }
        
        memset
    }

    pub fn init(&self) {
        let token = self.page_table.token();
        unsafe {
            satp::write(token);
            asm!("sfence.vma");
        }
    }

    fn push(&mut self, mut map_area: MapArea, data: Option<&[u8]>) {
        map_area.map(&mut self.page_table);
        if data.is_some() {
            map_area.copy_data(&mut self.page_table, data.unwrap());
        }
        self.areas.push(map_area);
    }
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