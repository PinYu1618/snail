#![feature(alloc_error_handler)]
#![no_std]
#![no_main]
#![allow(dead_code, unused_imports, unused_macros)]
#![allow(unused)]

#[macro_use]
extern crate bitflags;
extern crate alloc;

use log::{info, trace};

use core::arch::global_asm;

use snail_user::{fork, wait, yield_, exit, exec};

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
mod shell;

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

fn init() -> i32 {
    if fork() == 0 {
        exec("shell\0");
    } else {
        loop {
            let mut exit_code: i32 = 0;
            let pid = wait(&mut exit_code);
            if pid == -1 {
                yield_();
                continue;
            }
            println!(
                "[init] Released a zombie process, pid={}, exit code={}",
                pid,
                exit_code,
            );
        }
    }
    0
}

global_asm!(include_str!("entry.s"));