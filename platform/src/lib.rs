#![no_std]

#[macro_use]
extern crate cfg_if;
extern crate alloc;

//mod arch;
mod boot;
mod cpu;
mod interrupt;
mod mem;
pub mod timer;

cfg_if! {
    if #[cfg(target_arch = "x86_64")] {
        #[path ="x86/mod.rs"]
        pub mod arch;
    } else if #[cfg(any(target_arch = "riscv64", target_arch = "riscv32"))] {
        mod riscv;
        pub use riscv::RiscV;
    }
}

//pub use arch::Arch;
pub use boot::*;
pub(crate) use boot::Boot;
pub(crate) use cpu::Cpu;
pub use interrupt::Interrupt;
pub use mem::Memory;
pub use timer::Timer;