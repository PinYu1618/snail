#![feature(linkage)]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]
#![cfg_attr(not(test), no_std)]

#[macro_use]
extern crate bitflags;
extern crate alloc;

#[macro_use]
pub mod console;
mod lang;
mod sys;
pub mod syscall;

use alloc::vec::Vec;
use buddy_system_allocator::LockedHeap;
use syscall::*;

const UHEAP_SZ: usize = 32768;

static mut HEAP_SPACE: [u8; UHEAP_SZ] = [0; UHEAP_SZ];

#[global_allocator]
static HEAP: LockedHeap = LockedHeap::empty();

#[alloc_error_handler]
pub fn handle_alloc_error(layout: core::alloc::Layout) -> ! {
    panic!("Heap allocation error, layout = {:?}", layout);
}

#[no_mangle]
#[link_section = ".text.entry"]
pub extern "C" fn _start(argc: usize, argv: usize) -> ! {
    unsafe {
        HEAP.lock().init(HEAP_SPACE.as_ptr() as usize, UHEAP_SZ);
    }
    let mut v: Vec<&'static str> = Vec::new();
    for i in 0..argc {
        let str_start =
            unsafe { ((argv + i * core::mem::size_of::<usize>()) as *const usize).read_volatile() };
        let len = (0usize..)
            .find(|i| unsafe { ((str_start + *i) as *const u8).read_volatile() == 0 })
            .unwrap();
        v.push(
            core::str::from_utf8(unsafe {
                core::slice::from_raw_parts(str_start as *const u8, len)
            })
            .unwrap(),
        );
    }
    exit(main(argc, v.as_slice()));
}

#[linkage = "weak"]
#[no_mangle]
fn main(_argc: usize, _argv: &[&str]) -> i32 {
    panic!("Cannot find main!");
}
