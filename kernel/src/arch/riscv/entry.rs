use core::arch::asm;

#[naked]
#[no_mangle]
#[link_section = ".text.entry"]
#[export_name = "_start"]
unsafe extern "C" fn entry(hartid: usize, dt_paddr: usize) -> ! {
    asm!(
        "csrw sie, zero",
        "call {select_stack}",
        "j {main}",
        select_stack = sym select_stack,
        main = sym primary_rust_main,
        options(noreturn)
    )
}

extern "C" fn primary_rust_main(hartid: usize, dt_paddr: usize) -> ! {
    clear_bss();
    todo!()
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}

#[naked]
unsafe extern "C" fn select_stack(hartid: usize) {
    const PER_HART_STACK_SIZE: usize = 4 * 4096; // 16KiB
    const SBI_STACK_SIZE: usize = 8 * PER_HART_STACK_SIZE; // assume 8 cores in QEMU
    
    #[link_section = ".bss.uninit"]
    static mut SBI_STACK: [u8; SBI_STACK_SIZE] = [0; SBI_STACK_SIZE];
    
    asm!(
        "   addi t0, a0, 1",
        "   la sp, {stack}",
        "   li t1, {len_per_hart}",
        "1: add sp, sp, t1",
        "   addi t0, t0, -1",
        "   bnez t0, 1b",
        "   ret",
        stack = sym SBI_STACK,
        len_per_hart = const PER_HART_STACK_SIZE,
        options(noreturn)
    )
}

#[naked]
unsafe extern "C" fn secondary_hart_start(hartid: usize) -> ! {
    asm!(
        "csrw sie, zero",
        "call {select_stack}",
        "j {main}",
        select_stack = sym select_stack,
        main = sym secondary_rust_main,
        options(noreturn)
    )
}

extern "C" fn secondary_rust_main(hartid: usize) -> ! {
    todo!()
}