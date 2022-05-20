pub mod boot;
pub mod cpu;
mod interrupt;
mod kconfig;

pub struct X86;

impl crate::Boot for X86 {
    fn primary_init() {
        todo!()
    }

    fn secondary_init() {
        todo!()
    }
}

pub use cpu::X86Cpu as Cpu;