#![no_std]

pub mod cpu;
pub mod interrupt;
pub mod pm;
pub mod sbi;
pub mod timer;
pub mod vm;

pub const KERNEL_OFFSET: usize = 0xffff_ffff_8000_0000;
// for qemu -machine virt
pub const MAX_CPUS: usize = 8;
pub const MODE_SV39: usize = 8 << 60;
// qemu puts us here, this doesn't change.
pub const PHYS_MEMORY_START: usize = 0x8000_0000;
// temporarilly used, later we should change to parse it from device tree.
pub const PHYS_MEMORY_END: usize = 0x80f00000;

pub fn primary_init() {
    //unsafe { };
    todo!()
}

pub fn secondary_init() {
    todo!()
}

#[repr(align(4096))]
pub struct BootPageTable([usize; 512]);

impl BootPageTable {
    pub const ZERO: Self = Self([0; 512]);
}