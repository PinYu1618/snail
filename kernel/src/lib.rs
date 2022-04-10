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
pub mod sbi;
pub mod config;
pub mod trap;
pub mod timer;
pub mod drivers;
pub mod syscall;
pub mod sync;
pub mod task;
pub mod mm;
pub mod fs;
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