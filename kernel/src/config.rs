#![allow(unused)]

pub const USTACK_SZ: usize = 4096 * 2;

pub const KSTACK_SZ: usize = 4096 * 2;

pub const PAGE_SZ: usize = 0x1000;
pub const PAGE_SZ_BITS: usize = 0xc;

pub const TRAMPOLINE: usize = usize::MAX - PAGE_SZ + 1;
pub const TRAP_CONTEXT_BASE: usize = TRAMPOLINE - PAGE_SZ;
