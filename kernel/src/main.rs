#![no_std]
#![no_main]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(unused)]

#[macro_use]
extern crate snail_kernel;

use log::{info, trace};

use core::arch::global_asm;

//use snail_user::{fork, wait, yield_, exit, exec};

#[macro_use]
use snail_kernel::*;

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

//fn init() -> i32 {
//    if fork() == 0 {
//        exec("shell\0");
//    } else {
//        loop {
//            let mut exit_code: i32 = 0;
//            let pid = wait(&mut exit_code);
//            if pid == -1 {
//                yield_();
//                continue;
//            }
//            println!(
//                "[init] Released a zombie process, pid={}, exit code={}",
//                pid,
//                exit_code,
//            );
//        }
//    }
//    0
//}

global_asm!(include_str!("entry.s"));