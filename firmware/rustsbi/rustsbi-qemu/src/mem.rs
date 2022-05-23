use riscv::register::mcause;
use riscv::register::mcause::Mcause;
use riscv::register::mcause::Trap;
use riscv::register::mcause::Exception;
use riscv::register::mstatus;
use riscv::register::mtvec;
use riscv::register::mtvec::Mtvec;
use riscv::register::mtvec::TrapMode;
use core::arch::asm;
use core::mem::MaybeUninit;

pub fn set_pmp() {
    // todo: 根据QEMU的loader device等等，设置这里的权限配置
    // read fdt tree value, parse, and calculate proper pmp configuration for this device tree (issue #7)
    // integrate with `count_harts`
    //
    // Qemu MMIO config ref: https://github.com/qemu/qemu/blob/master/hw/riscv/virt.c#L46
    //
    // About PMP:
    //
    // CSR: pmpcfg0(0x3A0)~pmpcfg15(0x3AF); pmpaddr0(0x3B0)~pmpaddr63(0x3EF)
    // pmpcfg packs pmp entries each of which is of 8-bit
    // on RV64 only even pmpcfg CSRs(0,2,...,14) are available, each of which contains 8 PMP
    // entries
    // every pmp entry and its corresponding pmpaddr describe a pmp region
    //
    // layout of PMP entries:
    // ------------------------------------------------------
    //  7   |   [5:6]   |   [3:4]   |   2   |   1   |   0   |
    //  L   |   0(WARL) |   A       |   X   |   W   |   R   |
    // ------------------------------------------------------
    // A = OFF(0), disabled;
    // A = TOR(top of range, 1), match address y so that pmpaddr_{i-1}<=y<pmpaddr_i irrespective of
    // the value pmp entry i-1
    // A = NA4(naturally aligned 4-byte region, 2), only support a 4-byte pmp region
    // A = NAPOT(naturally aligned power-of-two region, 3), support a >=8-byte pmp region
    // When using NAPOT to match a address range [S,S+L), then the pmpaddr_i should be set to (S>>2)|((L>>2)-1)
    let calc_pmpaddr = |start_addr: usize, length: usize| (start_addr >> 2) | ((length >> 2) - 1);
    let mut pmpcfg0: usize = 0;
    // pmp region 0: RW, A=NAPOT, address range {0x1000_1000, 0x1000}, VIRT_VIRTIO
    //                            address range {0x1000_0000, 0x100}, VIRT_UART0
    //                            aligned address range {0x1000_0000, 0x2000}
    pmpcfg0 |= 0b11011;
    let pmpaddr0 = calc_pmpaddr(0x1000_0000, 0x2000);
    // pmp region 1: RW, A=NAPOT, address range {0x200_0000, 0x1_0000}, VIRT_CLINT
    pmpcfg0 |= 0b11011 << 8;
    let pmpaddr1 = calc_pmpaddr(0x200_0000, 0x1_0000);
    // pmp region 2: RW, A=NAPOT, address range {0xC00_0000, 0x40_0000}, VIRT_PLIC
    // VIRT_PLIC_SIZE = 0x20_0000 + 0x1000 * harts, thus supports up to 512 harts
    pmpcfg0 |= 0b11011 << 16;
    let pmpaddr2 = calc_pmpaddr(0xC00_0000, 0x40_0000);
    // pmp region 3: RWX, A=NAPOT, address range {0x8000_0000, 0x1000_0000}, VIRT_DRAM
    pmpcfg0 |= 0b11111 << 24;
    let pmpaddr3 = calc_pmpaddr(0x8000_0000, 0x1000_0000);
    unsafe {
        core::arch::asm!("csrw  pmpcfg0, {}",
             "csrw  pmpaddr0, {}",
             "csrw  pmpaddr1, {}",
             "csrw  pmpaddr2, {}",
             "csrw  pmpaddr3, {}",
             "sfence.vma",
             in(reg) pmpcfg0,
             in(reg) pmpaddr0,
             in(reg) pmpaddr1,
             in(reg) pmpaddr2,
             in(reg) pmpaddr3,
        );
    }
}

/// Reads the supervisor memory value, or fail if any exception occurred.
///
/// This function will invoke multiple instructions including reads, write, enabling
/// or disabling `mstatus.MPRV` bit. After they are executed, the value is typically returned
/// on stack or register with type `T`.
pub unsafe fn try_read<T>(src: SupervisorPointer<T>) -> Result<T, mcause::Exception> {
    let mut ans: MaybeUninit<T> = MaybeUninit::uninit();
    if mstatus::read().mprv() {
        panic!("rustsbi-qemu: mprv should be cleared before try_read")
    }
    for idx in (0..core::mem::size_of::<T>()).step_by(core::mem::size_of::<u32>()) {
        let nr = with_detect_trap(0, || {
            asm!(
            "li     {mprv_bit}, (1 << 17)",
            "csrs   mstatus, {mprv_bit}",
            "lw     {word}, 0({in_s_addr})",
            "csrc   mstatus, {mprv_bit}",
            "sw     {word}, 0({out_m_addr})",
            mprv_bit = out(reg) _,
            word = out(reg) _,
            in_s_addr = in(reg) src.inner.cast::<u8>().add(idx),
            out_m_addr = in(reg) ans.as_mut_ptr().cast::<u8>().add(idx),
            options(nostack),
            )
        });
        if nr != 0 {
            return Err(Exception::from(nr));
        }
    }
    Ok(ans.assume_init())
}

/// Pointer at supervisor level
///
/// These pointers cannot dereference directly from machine level. Instead, you may use
/// function `try_read` to get data from them.
#[derive(Debug)]
pub struct SupervisorPointer<T> {
    inner: *const T,
}

// Trap frame for instruction exception detection
#[repr(C)]
struct TrapFrame {
    ra: usize,
    tp: usize,
    a0: usize,
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,
    a6: usize,
    a7: usize,
    t0: usize,
    t1: usize,
    t2: usize,
    t3: usize,
    t4: usize,
    t5: usize,
    t6: usize,
    mstatus: usize,
    mepc: usize,
    mcause: Mcause,
    mtval: usize,
}

impl<T> SupervisorPointer<T> {
    /// Cast a supervisor parameter into a supervisor pointer
    ///
    /// This is a safe function for creation of a raw pointer; deref it will be unsafe.
    pub fn cast(supervisor_param: usize) -> Self {
        SupervisorPointer { inner: supervisor_param as *const _ }
    }
}

// Tries to execute all instructions defined in clojure `f`.
// If resulted in an exception, this function returns its exception id.
//
// This function is useful to detect if an instruction exists on current environment.
#[inline]
fn with_detect_trap(param: usize, f: impl FnOnce()) -> usize {
    // disable interrupts and handle exceptions only
    let (mie, mtvec, tp) = unsafe { init_detect_trap(param) };
    // run detection inner
    f();
    // restore trap handler and enable interrupts
    let ans = unsafe { restore_detect_trap(mie, mtvec, tp) };
    // return the answer
    ans
}

// Initialize environment for trap detection and filter in exception only
#[inline]
unsafe fn init_detect_trap(param: usize) -> (bool, Mtvec, usize) {
    // clear mie to handle exception only
    let stored_mie = mstatus::read().mie();
    mstatus::clear_mie();
    // use detect trap handler to handle exceptions
    let stored_mtvec = mtvec::read();
    let mut trap_addr = on_detect_trap as usize;
    if trap_addr & 0b1 != 0 {
        trap_addr += 0b1;
    }
    mtvec::write(trap_addr, TrapMode::Direct);
    // store tp register. tp will be used to load parameter and store return value
    let stored_tp: usize;
    asm!("mv  {}, tp", "mv  tp, {}", out(reg) stored_tp, in(reg) param, options(nomem, nostack));
    // returns preserved previous hardware states
    (stored_mie, stored_mtvec, stored_tp)
}

// Restore previous hardware states before trap detection
#[inline]
unsafe fn restore_detect_trap(mie: bool, mtvec: Mtvec, tp: usize) -> usize {
    // read the return value from tp register, and restore tp value
    let ans: usize;
    asm!("mv  {}, tp", "mv  tp, {}", out(reg) ans, in(reg) tp, options(nomem, nostack));
    // restore trap vector settings
    asm!("csrw  mtvec, {}", in(reg) mtvec.bits(), options(nomem, nostack));
    // enable interrupts
    if mie {
        mstatus::set_mie();
    };
    ans
}

// rust trap handler for detect exceptions
extern "C" fn rust_detect_trap(trap_frame: &mut TrapFrame) {
    // store returned exception id value into tp register
    // specially: illegal instruction => 2
    trap_frame.tp = trap_frame.mcause.bits();
    // if illegal instruction, skip current instruction
    match trap_frame.mcause.cause() {
        Trap::Exception(_) => {
            let mut insn_bits = riscv_illegal_instruction_bits((trap_frame.mtval & 0xFFFF) as u16);
            if insn_bits == 0 {
                let insn_half = unsafe { *(trap_frame.mepc as *const u16) };
                insn_bits = riscv_illegal_instruction_bits(insn_half);
            }
            // skip current instruction
            trap_frame.mepc = trap_frame.mepc.wrapping_add(insn_bits);
        }
        Trap::Interrupt(_) => unreachable!(), // filtered out for mie == false
    }
}

// Gets risc-v instruction bits from illegal instruction stval value, or 0 if unknown
#[inline]
fn riscv_illegal_instruction_bits(insn: u16) -> usize {
    if insn == 0 {
        return 0; // mtval[0..16] == 0, unknown
    }
    if insn & 0b11 != 0b11 {
        return 2; // 16-bit
    }
    if insn & 0b11100 != 0b11100 {
        return 4; // 32-bit
    }
    // FIXME: add >= 48-bit instructions in the future if we need to proceed with such instructions
    return 0; // >= 48-bit, unknown from this function by now
}

// Assembly trap handler for instruction detection.
//
// This trap handler shares the same stack from its prospective caller,
// the caller must ensure it has abundant stack size for a trap handler.
//
// This function should not be used in conventional trap handling,
// as it does not preserve a special trap stack, and it's designed to
// handle exceptions only rather than interrupts.
#[naked]
unsafe extern "C" fn on_detect_trap() -> ! {
    asm!(
    ".p2align 2",
    "addi   sp, sp, -8*21",
    "sd     ra, 0*8(sp)",
    "sd     tp, 1*8(sp)",
    "sd     a0, 2*8(sp)",
    "sd     a1, 3*8(sp)",
    "sd     a2, 4*8(sp)",
    "sd     a3, 5*8(sp)",
    "sd     a4, 6*8(sp)",
    "sd     a5, 7*8(sp)",
    "sd     a6, 8*8(sp)",
    "sd     a7, 9*8(sp)",
    "sd     t0, 10*8(sp)",
    "sd     t1, 11*8(sp)",
    "sd     t2, 12*8(sp)",
    "sd     t3, 13*8(sp)",
    "sd     t4, 14*8(sp)",
    "sd     t5, 15*8(sp)",
    "sd     t6, 16*8(sp)",
    "csrr   t0, mstatus",
    "sd     t0, 17*8(sp)",
    "csrr   t1, mepc",
    "sd     t1, 18*8(sp)",
    "csrr   t2, mcause",
    "sd     t2, 19*8(sp)",
    "csrr   t3, mtval",
    "sd     t3, 20*8(sp)",
    "mv     a0, sp",
    "li     t4, (1 << 17)", // clear mstatus.mprv
    "csrc   mstatus, t4",
    "call   {rust_detect_trap}",
    "ld     t0, 17*8(sp)",
    "csrw   mstatus, t0",
    "ld     t1, 18*8(sp)",
    "csrw   mepc, t1",
    "ld     t2, 19*8(sp)",
    "csrw   mcause, t2",
    "ld     t3, 20*8(sp)",
    "csrw   mtval, t3",
    "ld     ra, 0*8(sp)",
    "ld     tp, 1*8(sp)",
    "ld     a0, 2*8(sp)",
    "ld     a1, 3*8(sp)",
    "ld     a2, 4*8(sp)",
    "ld     a3, 5*8(sp)",
    "ld     a4, 6*8(sp)",
    "ld     a5, 7*8(sp)",
    "ld     a6, 8*8(sp)",
    "ld     a7, 9*8(sp)",
    "ld     t0, 10*8(sp)",
    "ld     t1, 11*8(sp)",
    "ld     t2, 12*8(sp)",
    "ld     t3, 13*8(sp)",
    "ld     t4, 14*8(sp)",
    "ld     t5, 15*8(sp)",
    "ld     t6, 16*8(sp)",
    "addi   sp, sp, 8*21",
    "sret",
    rust_detect_trap = sym rust_detect_trap,
    options(noreturn),
    )
}