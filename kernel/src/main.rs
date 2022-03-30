#![feature(alloc_error_handler)]
#![no_std]
#![no_main]
#![allow(dead_code, unused_imports, unused_macros)]
#![allow(unused)]

#[macro_use]
extern crate bitflags;
extern crate alloc;

use log::info;
use core::arch::global_asm;

#[macro_use]
mod logging;
mod lang;
mod sbi;
mod trap;
mod timer;
mod config;
mod syscall;
mod sync;
mod task;
mod mm;
mod fs;

#[no_mangle]
pub extern "C" fn kmain() -> ! {
    clear_bss();
    logging::init();
    mm::init();
    trap::init();
    trap::enable_timer_interrupt();
    timer::set_next_trigger();
    info!("Hyy, there.");
    loop {}
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe {
            (a as *mut u8).write_volatile(0)
        }
    });
}

global_asm!(include_str!("entry.s"));