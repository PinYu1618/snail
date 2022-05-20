//! Code Ref: zcore/kernel-hal/src/x86_64/cpu.rs

use raw_cpuid::CpuId;

use crate::cpu::Cpu;
pub struct X86Cpu;

impl crate::Cpu for X86Cpu {
    fn id() -> u8 {
        CpuId::new()
            .get_feature_info()
            .unwrap()
            .initial_local_apic_id() as u8
    }

    fn frequency() -> u16 {
        static CPU_FREQ_MHZ: spin::Once<u16> = spin::Once::new();
        *CPU_FREQ_MHZ.call_once(|| {
            const DEFAULT: u16 = 4000;
            CpuId::new()
                .get_processor_frequency_info()
                .map(|info| info.processor_base_frequency())
                .unwrap_or(DEFAULT)
                .max(DEFAULT)
        })
    }
}

pub fn id() -> u8 {
    X86Cpu::id()
}