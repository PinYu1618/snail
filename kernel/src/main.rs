#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
#![feature(custom_test_frameworks)]
#![test_runner(snail::test_runner)]
#![reexport_test_harness_main = "test_main"]

#[macro_use]
extern crate log;
extern crate alloc;

use core::arch::global_asm;
use core::panic::PanicInfo;
pub use snail::println;

#[no_mangle]
pub extern "C" fn kmain() -> ! {
    clear_bss();
    snail::logging::init().unwrap();
    snail::mm::init();
    snail::trap::init();
    snail::trap::enable_timer_interrupt();
    snail::timer::Timer::set_next_trigger();
    snail::fs::list_all_apps();
    snail::task_::add_initproc();
    info!("Hyy, there.");

    #[cfg(test)]
    test_main();

    loop {}
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}

global_asm!(include_str!("entry.s"));

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    snail::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
