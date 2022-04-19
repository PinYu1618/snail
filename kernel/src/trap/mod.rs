pub mod context;

use crate::config::{TRAMPOLINE, TRAP_CONTEXT_BASE};
use crate::syscall::syscall;
use crate::task::Processor;
use crate::timer::Timer;
use core::arch::{asm, global_asm};
use riscv::register::mtvec::TrapMode;
use riscv::register::scause::{Exception, Interrupt, Trap};
use riscv::register::{scause, sie, stval, stvec};

pub use context::TrapContext;

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
    set_user_trap_entry();
    let trap_cx_ptr = TRAP_CONTEXT_BASE;
    let user_satp = Processor::current_user_token();
    extern "C" {
        fn __alltraps();
        fn __restore();
    }
    let restore_va = __restore as usize - __alltraps as usize + TRAMPOLINE;
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
fn trap_from_kernel() {
    panic!("a trap from kernel!");
}

fn set_kernel_trap_entry() {
    unsafe {
        stvec::write(trap_from_kernel as usize, TrapMode::Direct);
    }
}

fn set_user_trap_entry() {
    unsafe {
        stvec::write(TRAMPOLINE as usize, TrapMode::Direct);
    }
}

global_asm!(include_str!("trap.s"));
