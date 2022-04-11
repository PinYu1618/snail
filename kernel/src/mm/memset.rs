use bitflags::*;
use lazy_static::*;
use log::{debug, info, trace};
use riscv::register::satp;
use xmas_elf;

use alloc::collections::BTreeMap;
use alloc::sync::Arc;
use alloc::vec::Vec;

use core::arch::asm;

use crate::config::{TRAMPOLINE, USTACK_SZ, TRAP_CONTEXT_BASE};
use crate::mm::map::{MapPermission, MapType};
use crate::{
    config::{MEM_END, MMIO, PAGE_SZ},
    mm::addr::Step,
    sync::up::UPSafeCell,
};

use super::addr::{PhysAddr, PhysPageNr, VPNRange, VirtAddr, VirtPageNr};
use super::frame::{alloc_frame, FrameTracker};
use super::map::MapArea;
use super::page::{PTEFlags, PageTable, PageTableEntry};

extern "C" {
    fn stext();
    fn etext();
    fn srodata();
    fn erodata();
    fn sdata();
    fn edata();
    fn sbss_with_stack();
    fn ebss();
    fn ekernel();
    fn strampoline();
}

#[derive(Clone, Debug)]
pub struct MemorySet {
    page_table: PageTable,
    areas: Vec<MapArea>,
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
        memset.map_trampoline();

        // map kernel sections
        info!("Mapping kernel sections:");
        info!(".text [{:#x}, {:#x})", stext as usize, etext as usize);
        info!(".rodata [{:#x}, {:#x})", srodata as usize, erodata as usize);
        info!(".data [{:#x}, {:#x})", sdata as usize, edata as usize);
        info!(
            ".bss [{:#x}, {:#x})",
            sbss_with_stack as usize, ebss as usize
        );

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
                (sbss_with_stack as usize).into(),
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

    pub fn token(&self) -> usize {
        self.page_table.token()
    }

    pub fn insert_framed_area(&mut self, sva: VirtAddr, eva: VirtAddr, perm: MapPermission) {
        self.push(MapArea::new(sva, eva, MapType::Framed, perm), None);
    }

    pub fn remove_area(&mut self, svpn: VirtPageNr) {
        unimplemented!()
    }

    pub fn from_existed_user(uspace: &MemorySet) -> MemorySet {
        let mut memset = Self::new_bare();

        // map trampoline
        memset.map_trampoline();

        for area in uspace.areas.iter() {
            let new_area = MapArea::from_another(area);
            memset.push(new_area, None);
            for vpn in area.vpn_range {
                let src_ppn = uspace.translate(vpn).unwrap().ppn();
                let dst_ppn = memset.translate(vpn).unwrap().ppn();
                dst_ppn.bytes_arr().copy_from_slice(src_ppn.bytes_arr());
            }
        }

        memset
    }

    // Return (memset, ustack base, entry point) from user's elf file
    pub fn from_elf(elf_data: &[u8]) -> (Self, usize, usize) {
        assert!(!elf_data.is_empty());
        let mut memset = Self::new_bare();

        // map trampoline
        memset.map_trampoline();

        let elf = xmas_elf::ElfFile::new(elf_data).unwrap();
        let header = elf.header;

        // check elf magic
        let magic = header.pt1.magic;
        assert_eq!(magic, [0x7f, 0x45, 0x4c, 0x46], "Invalid elf.");

        // map program headers with (at least) U-flag
        let ph_count = header.pt2.ph_count();
        let mut max_evpn = VirtPageNr::from(0);
        for i in 0..ph_count {
            let ph = elf.program_header(i).unwrap();
            if ph.get_type().unwrap() == xmas_elf::program::Type::Load {
                let sva: VirtAddr = (ph.virtual_addr() as usize).into();
                let eva: VirtAddr = ((ph.virtual_addr() + ph.mem_size()) as usize).into();
                let mut perm = MapPermission::U;
                let ph_flags = ph.flags();

                if ph_flags.is_read() {
                    perm |= MapPermission::R;
                }
                if ph_flags.is_write() {
                    perm |= MapPermission::W;
                }
                if ph_flags.is_execute() {
                    perm |= MapPermission::X;
                }

                let map_area = MapArea::new(sva, eva, MapType::Framed, perm);
                max_evpn = map_area.vpn_range.end();
                memset.push(
                    map_area,
                    Some(&elf.input[ph.offset() as usize..(ph.offset() + ph.file_size()) as usize]),
                );
            }
        }

        // map user stack with U flags
        let max_eva: VirtAddr = max_evpn.into();
        let mut ustack_base: usize = max_eva.into();

        // guard page
        ustack_base += PAGE_SZ;
        let ustack_top = ustack_base + USTACK_SZ;
        memset.push(
            MapArea::new(
                ustack_base.into(),
                ustack_top.into(),
                MapType::Framed,
                MapPermission::R | MapPermission::W | MapPermission::U,
            ),
            None,
        );

        // map TrapContext
        memset.push(
            MapArea::new(
                TRAP_CONTEXT_BASE.into(),
                TRAMPOLINE.into(),
                MapType::Framed,
                MapPermission::R | MapPermission::W,
            ),
            None,
        );

        (memset, ustack_top, header.pt2.entry_point() as usize)
    }

    pub fn translate(&self, vpn: VirtPageNr) -> Option<PageTableEntry> {
        self.page_table.translate(vpn)
    }

    fn push(&mut self, mut map_area: MapArea, data: Option<&[u8]>) {
        map_area.map(&mut self.page_table);
        if let Some(data) = data {
            map_area.copy_data(&mut self.page_table, data);
        }
        self.areas.push(map_area);
    }

    fn map_trampoline(&mut self) {
        self.page_table.map(
            VirtAddr::from(TRAMPOLINE).into(),
            PhysAddr::from(strampoline as usize).into(),
            PTEFlags::R | PTEFlags::X,
        );
    }
}

lazy_static! {
    pub static ref KSPACE: Arc<UPSafeCell<MemorySet>> =
        Arc::new(unsafe { UPSafeCell::new(MemorySet::new_kernel()) });
}

pub fn ktoken() -> usize {
    KSPACE.exclusive_access().token()
}

#[cfg(dbg)]
pub fn test_remap() {
    let mut kernel_space = KERNEL_SPACE.exclusive_access();
    let mid_text: VirtAddr = ((stext as usize + etext as usize) / 2).into();
    let mid_rodata: VirtAddr = ((srodata as usize + erodata as usize) / 2).into();
    let mid_data: VirtAddr = ((sdata as usize + edata as usize) / 2).into();
    assert!(!kernel_space
        .page_table
        .translate(mid_text.floor())
        .unwrap()
        .is_writable(),);
    assert!(!kernel_space
        .page_table
        .translate(mid_rodata.floor())
        .unwrap()
        .is_writable(),);
    assert!(!kernel_space
        .page_table
        .translate(mid_data.floor())
        .unwrap()
        .is_executable(),);
    println!("remap_test passed!");
}
