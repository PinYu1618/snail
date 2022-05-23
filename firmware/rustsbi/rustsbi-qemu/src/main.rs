#![feature(asm_sym, asm_const)]
#![feature(default_alloc_error_handler)]
#![feature(generator_trait)]
#![feature(naked_functions)]
#![no_main]
#![no_std]

extern crate alloc;
#[macro_use]
extern crate rustsbi;

mod clint;
mod executer;
mod feature;
mod hart_count;
mod hsm;
mod mem;
mod ns16550a;
mod runtime;
mod sifive;

use buddy_system_allocator::LockedHeap;
use rustsbi::SbiRet;
use riscv::register::mstatus::Mstatus;
use riscv::register::mtvec;
use riscv::register::mtvec::TrapMode;
use core::arch::asm;

extern "C" fn rust_main(hartid: usize, opaque: usize) -> ! {
    runtime::init();
    if hartid == 0 {
        init_heap();
        ns16550a::early_init();
        clint::init();
        sifive::test();
        unsafe { hart_count::init(opaque) };
        rustsbi::init_hsm(HSM.clone());
    } else {
        hsm::pause();
    }
    delegate_interrupt_exception();
    mem::set_pmp();
    unsafe {
        // enable wake by ipi
        riscv::register::mstatus::set_mie();
    }
    if hartid == 0 {
        // start other harts
        let clint = clint::Clint::new(0x2000000 as *mut u8);
        let num_harts = *{ hart_count::NUM_HARTS.lock() };
        for target_hart_id in 0..num_harts {
            if target_hart_id != 0 {
                clint.send_soft(target_hart_id);
            }
        }
        println!("[rustsbi] enter supervisor 0x80200000");
    }
    executer::execute_supervisor(0x80200000, hartid, opaque, HSM.clone())
}

lazy_static::lazy_static! {
    static ref HSM: hsm::Hsm = hsm::Hsm::new();
}

const PER_HART_STACK_SIZE: usize = 4 * 4096; // 16KiB
const SBI_STACK_SIZE: usize = 8 * PER_HART_STACK_SIZE; // assume 8 cores in QEMU
#[link_section = ".bss.uninit"]
static mut SBI_STACK: [u8; SBI_STACK_SIZE] = [0; SBI_STACK_SIZE];

const SBI_HEAP_SIZE: usize = 64 * 1024; // 64KiB
#[link_section = ".bss.uninit"]
static mut HEAP_SPACE: [u8; SBI_HEAP_SIZE] = [0; SBI_HEAP_SIZE];
#[global_allocator]
static SBI_HEAP: LockedHeap<32> = LockedHeap::empty();

fn init_heap() {
    unsafe {
        SBI_HEAP
            .lock()
            .init(HEAP_SPACE.as_ptr() as usize, SBI_HEAP_SIZE)
    }
}

fn delegate_interrupt_exception() {
    use riscv::register::{medeleg, mideleg, mie};
    unsafe {
        mideleg::set_sext();
        mideleg::set_stimer();
        mideleg::set_ssoft();
        mideleg::set_uext();
        mideleg::set_utimer();
        mideleg::set_usoft();
        medeleg::set_instruction_misaligned();
        medeleg::set_breakpoint();
        medeleg::set_user_env_call();
        medeleg::set_instruction_page_fault();
        medeleg::set_load_page_fault();
        medeleg::set_store_page_fault();
        medeleg::set_instruction_fault();
        medeleg::set_load_fault();
        medeleg::set_store_fault();
        mie::set_mext();
        // 不打开mie::set_mtimer
        mie::set_msoft();
    }
}

#[naked]
#[link_section = ".text.entry"]
#[export_name = "_start"]
unsafe extern "C" fn entry(a0: usize, a1: usize) -> ! {
    asm!(
    // 1. set sp
    // sp = bootstack + (hartid + 1) * HART_STACK_SIZE
    "
    la      sp, {stack}
    li      t0, {per_hart_stack_size}
    addi    t1, a0, 1
1:  add     sp, sp, t0
    addi    t1, t1, -1
    bnez    t1, 1b
    ",
    // 2. jump to rust_main (absolute address)
    "j      {rust_main}",
    per_hart_stack_size = const PER_HART_STACK_SIZE,
    stack = sym SBI_STACK,
    rust_main = sym rust_main,
    options(noreturn))
}