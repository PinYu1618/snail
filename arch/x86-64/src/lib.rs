//! x86-64 arch specific things for snail os.
//! 
//! Note this crate is written for the snail os project only,
//! so it may lose some generality for convenience. :(
#![no_std]
#![cfg(target_arch = "x86_64")]
#![cfg(target_pointer_width = "64")]
#![feature(abi_x86_interrupt)]

#[macro_use]
extern crate lazy_static;

use x86_64::instructions::segmentation::CS;
use x86_64::instructions::tables::load_tss;
//use x86_64::registers::control::Cr3;
use x86_64::registers::segmentation::Segment;
use x86_64::structures::gdt::Descriptor;
use x86_64::structures::gdt::GlobalDescriptorTable;
use x86_64::structures::gdt::SegmentSelector;
use x86_64::structures::idt::InterruptDescriptorTable;
use x86_64::structures::idt::InterruptStackFrame;
use x86_64::structures::tss::TaskStateSegment;
use x86_64::VirtAddr;

pub mod cpu;
pub mod idt;
pub mod gdt;
pub mod interrupt;
pub mod vm;

pub fn primary_early_init() {
    todo!()
}

pub fn primary_init() {
    todo!()
}

pub fn secondary_init() {
    todo!()
}

pub fn init_ram_disk() -> Option<&'static mut [u8]> {
    todo!()
}