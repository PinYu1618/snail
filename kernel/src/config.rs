#![allow(unused)]

pub const USTACK_SZ: usize = 4096 * 2;

pub const KSTACK_SZ: usize = 4096 * 2;
pub const KHEAP_SZ: usize = 0x20_0000;

pub const MEM_END: usize = 0x80800000;
pub const PAGE_SZ: usize = 0x1000;
pub const PAGE_SZ_BITS: usize = 0xc;

pub const TRAMPOLINE: usize = usize::MAX - PAGE_SZ + 1;
pub const TRAP_CONTEXT_BASE: usize = TRAMPOLINE - PAGE_SZ;

pub const CLOCK_FREQ: usize = 12500000;

pub const MMIO: &[(usize, usize)] = &[
    //    (0x1000_0000, 0x1000),
    (0x1000_1000, 0x1000),
    //    (0xC00_0000, 0x40_0000),
];
