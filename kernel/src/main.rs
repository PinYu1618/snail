#![no_std]
#![no_main]
#![feature(asm_sym, asm_const)]
#![feature(alloc_error_handler)]
#![feature(custom_test_frameworks)]
#![feature(naked_functions)]
#![test_runner(snail::test_runner)]
#![reexport_test_harness_main = "test_main"]

#[macro_use]
extern crate log;
extern crate alloc;

use arch_hal as hal;
use core::panic::PanicInfo;
use core::sync::atomic::AtomicBool;
use core::sync::atomic::Ordering;
pub use snail::println;

static STARTED: AtomicBool = AtomicBool::new(false);

#[no_mangle]
extern "C" fn kprimary_main() -> ! {
    snail::logging::init().unwrap();
    snail::memory::init_heap();

    hal::primary_init();
    STARTED.store(true, Ordering::SeqCst);

    #[cfg(test)]
    test_main();

    todo!()
}

#[no_mangle]
extern "C" fn ksecondary_main() -> ! {
    while !STARTED.load(Ordering::SeqCst) {
        core::hint::spin_loop();
    }

    hal::secondary_init();
    info!("[Hart {}] init.", hal::cpu::id());

    todo!()
}

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
