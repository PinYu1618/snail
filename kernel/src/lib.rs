#![feature(panic_info_message)]
#![feature(alloc_error_handler)]
//#![feature(custom_test_frameworks)]
#![no_std]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(unused)]
//#![test_runner(crate::test_runner)]

#[macro_use]
extern crate bitflags;
extern crate alloc;

#[macro_use]
pub mod logging;
pub mod config;
pub mod drivers;
pub mod fs;
pub mod mm;
pub mod sbi;
pub mod sync;
pub mod syscall;
pub mod task;
pub mod timer;
pub mod trap;
//pub mod shell;
mod lang;

//#[cfg(test)]
//fn test_runner(tests: &[&dyn Fn()]) {
//    println!("Running {} tests", tests.len());
//    for test in tests {
//        test();
//    }
//}
