#![feature(panic_info_message)]
#![feature(alloc_error_handler)]
#![no_std]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(unused)]

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

#[cfg(test)]
mod test {
    #[test]
    #[should_panic]
    fn panics_ok() {
        assert!(false);
    }
}
