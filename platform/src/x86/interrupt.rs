use x86_64::instructions::interrupts;
use crate::Interrupt;
use super::X86;

impl Interrupt for X86 {
    fn enable() {
        interrupts::enable();
    }

    fn disable() {
        interrupts::disable();
    }
}