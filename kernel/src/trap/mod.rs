pub mod context;

use log::{error, trace};
use riscv::register::mtvec::TrapMode;
use riscv::register::scause::{Exception, Interrupt, Trap};
use riscv::register::{scause, sie, stval, stvec};

use core::arch::global_asm;

use crate::syscall::syscall;
use crate::task::suspend_current_and_run_next;
use crate::timer::set_next_trigger;

use context::TrapContext;

pub fn init() {
    extern "C" {
        fn __alltraps();
    }
    unsafe {
        stvec::write(__alltraps as usize, TrapMode::Direct);
    }
}

pub fn enable_timer_interrupt() {
    unsafe {
        sie::set_stimer();
    }
}

#[no_mangle]
pub fn trap_handler(cx: &mut TrapContext) -> &mut TrapContext {
    let scause = scause::read();
    let stval = stval::read();
    match scause.cause() {
        Trap::Interrupt(Interrupt::SupervisorTimer) => {
            set_next_trigger();
            suspend_current_and_run_next();
        }
        Trap::Exception(Exception::UserEnvCall) => {
            cx.sepc += 4;
            cx.x[10] = syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]) as usize;
        }
        Trap::Exception(Exception::StoreFault) | Trap::Exception(Exception::StorePageFault) => {
            error!("PageFault :(");
            panic!("Bobo was panicked due to page fault!");
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            error!(
                "Illegal Instruction in application. Don't forget to improve this handler later"
            );
            panic!("Bobo was panicked due to illegal instruction in application.");
        }
        _ => {
            panic!(
                "Bobo was panicked due to unsupported trap {:?}, stval = {:#x}!",
                scause.cause(),
                stval
            );
        }
    }
    cx
}

global_asm!(include_str!("trap.s"));
