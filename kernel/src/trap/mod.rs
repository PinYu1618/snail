pub mod context;

use riscv::register::{ stvec, scause, stval, sie };
use riscv::register::scause::{ Trap, Exception, Interrupt };
use riscv::register::mtvec::TrapMode;
use context::TrapContext;
use crate::syscall::syscall;
use crate::timer::set_next_trigger;
use core::arch::global_asm;

pub fn init() {
    extern "C" { fn __alltraps(); }
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
        },
        Trap::Exception(Exception::UserEnvCall) => {
            cx.sepc += 4;
            cx.x[10] = syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]) as usize;
        }
        Trap::Exception(Exception::StoreFault) |
        Trap::Exception(Exception::StorePageFault) => {
            println!("[kernel] PageFault in application.");
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            println!("[kernel] Illegal Instruction in application.");
        }
        _ => {
            panic!("[kernel] Unsupported trap {:?}, stval = {:#x}!", scause.cause(), stval);
        }
    }
    cx
}

global_asm!(include_str!("trap.s"));