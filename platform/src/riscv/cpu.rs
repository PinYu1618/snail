pub struct Cpu;

impl crate::Cpu for Cpu {
    fn id(&self) -> u8 {
        let mut cpu_id;
        unsafe { core::arch::asm!("mv {0}, tp", out(reg) cpu_id) };
        cpu_id
    }

    fn frequency() -> u32 {
        todo!()
    }
}