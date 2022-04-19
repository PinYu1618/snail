use riscv::register::sstatus::{self, Sstatus, SPP};

#[repr(C)]
pub struct TrapContext {
    pub x: [usize; 32], // registers x0 ~ x31
    pub sstatus: Sstatus,
    pub sepc: usize, // program counter after trap ended
    pub kernel_satp: usize,
    pub kernel_stack_top: usize,
    pub trap_handler: usize,
}

impl TrapContext {
    pub fn set_sp(&mut self, sp: usize) {
        self.x[2] = sp;
    }

    pub fn init_app_cx(
        entry: usize,
        sp: usize,
        kernel_satp: usize,
        kernel_stack_top: usize,
        trap_handler: usize,
    ) -> Self {
        let mut sstatus = sstatus::read();
        // set cpu privilege to U after trapping back
        sstatus.set_spp(SPP::User);
        let mut cx = Self {
            x: [0; 32],
            sstatus,
            sepc: entry,
            kernel_satp,
            kernel_stack_top,
            trap_handler,
        };
        cx.set_sp(sp);
        cx
    }
}
