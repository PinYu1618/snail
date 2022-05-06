use crate::config::TRAP_CONTEXT_BASE;
use crate::syscall::syscall;
use crate::task_::Processor;
use crate::timer::Timer;
use core::arch::{asm, global_asm};
use riscv::register::mtvec::TrapMode;
use riscv::register::scause::{Exception, Interrupt, Trap};
use riscv::register::{scause, sie, stval, stvec};

pub fn init() {
    set_kernel_trap_entry();
}

pub fn enable_timer_interrupt() {
    unsafe {
        sie::set_stimer();
    }
}

#[no_mangle]
pub fn trap_handler() -> ! {
    set_kernel_trap_entry();
    let scause = scause::read();
    let stval = stval::read();
    match scause.cause() {
        Trap::Exception(Exception::UserEnvCall) => {
            trace!("UserEnvCall, bobo!");
            let mut cx = Processor::current_trap_cx();
            cx.sepc += 4;
            let res = syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]) as usize;
            cx = Processor::current_trap_cx();
            cx.x[10] = res;
        }
        Trap::Exception(Exception::StoreFault) => {
            error!("StoreFault");
            panic!("Bobo was panicked due to store fault!");
        }
        Trap::Exception(Exception::StorePageFault) => {
            error!("PageFault :(");
            panic!("Bobo was panicked due to page fault!");
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            error!(
                "Illegal Instruction in application. Don't forget to improve this handler later"
            );
            panic!("Bobo was panicked due to illegal instruction in application.");
        }
        Trap::Interrupt(Interrupt::SupervisorTimer) => {
            Timer::set_next_trigger();
            Processor::suspend_current_and_run_next();
        }
        _ => {
            panic!(
                "Bobo was panicked due to unsupported trap {:?}, stval = {:#x}!",
                scause.cause(),
                stval
            );
        }
    }
    trap_return()
}

#[no_mangle]
pub fn trap_return() -> ! {
    use crate::config;
    set_user_trap_entry();
    let trap_cx_ptr = TRAP_CONTEXT_BASE;
    let user_satp = Processor::current_user_token();
    extern "C" {
        fn __alltraps();
        fn __restore();
    }
    let restore_va = __restore as usize - __alltraps as usize + config::TRAMPOLINE;
    //println!("before return");
    unsafe {
        asm!(
            "fence.i",
            "jr {restore_va}",
            restore_va = in(reg) restore_va,
            in("a0") trap_cx_ptr,
            in("a1") user_satp,
            options(noreturn)
        );
    }
    //panic!("Unreachable in back_to_user!");
}

#[no_mangle]
pub fn trap_from_kernel() -> ! {
    use riscv::register::sepc;
    println!("stval = {:#x}, sepc = {:#x}", stval::read(), sepc::read());
    panic!("a trap from kernel!");
}

fn set_kernel_trap_entry() {
    unsafe {
        stvec::write(trap_from_kernel as usize, TrapMode::Direct);
    }
}

fn set_user_trap_entry() {
    use crate::config::TRAMPOLINE;
    unsafe {
        stvec::write(TRAMPOLINE as usize, TrapMode::Direct);
    }
}

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

    pub fn init_app_cx(entry: usize, sp: usize, kernel_satp: usize, kernel_stack_top: usize, trap_handler: usize) -> Self {
        // set cpu privilege to U after trapping back
        let sstatus = sstatus::read();
        unsafe { sstatus::set_spp(SPP::User); }
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


global_asm!(include_str!("trap.s"));
