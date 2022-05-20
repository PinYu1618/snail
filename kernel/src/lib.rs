#![no_std]
#![cfg_attr(test, no_main)]
#![feature(asm_sym, asm_const)]
#![feature(alloc_error_handler)]
#![feature(custom_test_frameworks)]
#![feature(naked_functions)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

#[macro_use]
extern crate log;
#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate lazy_static;
extern crate alloc;

#[macro_use]
pub mod logging;
pub mod config;
pub mod drivers;
mod errno;
pub mod fs;
pub mod mm;
pub mod sbi;
pub mod signal;
pub mod sync;
pub mod syscall;
pub mod task;
pub mod task_;
pub mod timer;
pub mod trap;

use core::panic::PanicInfo;

pub use errno::Error;
pub type Result<T> = core::result::Result<T, Error>;

pub trait Testable {
    fn run(&self);
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        print!("{}...\t", core::any::type_name::<T>());
        self();
        println!("[ok]");
    }
}

pub fn test_runner(tests: &[&dyn Testable]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    //exit_qemu(QemuExitCode::Success);
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    println!("[failed]\n");
    println!("Error: {}\n", info);
    //exit_qemu(QemuExitCode::Failed);
    loop {}
}

/// Entry point for `cargo test`
#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}
