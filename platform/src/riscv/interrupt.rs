use riscv_crate::register::sstatus;
use crate::Interrupt;
use super::RiscV;

impl Interrupt for RiscV {
    fn intr_enable() {
        unsafe { sstatus::set_sie() };
    }

    fn intr_disable() {
        unsafe { sstatus::clear_sie() };
    }
}