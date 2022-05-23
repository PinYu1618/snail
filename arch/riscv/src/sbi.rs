#![cfg(target_arch = "riscv64")]
#![allow(unused)]

/// Note! user of these sbi functions (aka supervisor) must save all
/// registers except a0, a1 before calling it.

use core::arch::asm;

// Binary Encoding

#[repr(C)]
pub struct SbiRet {
    pub error: isize,
    pub value: isize,
}

pub enum SbiError {
    Success,
    Failed,
    NotSupported,
    InvalidParam,
    Denied,
    InvalidAddr,
    AreadyAvailable,
    AreadyStarted,
    AreadyStopped,
}

// Base extensions

/// Returns the current SBI specification version. This function must always succeed. The minor number of the SBI specification is encoded in the low 24 bits, with the major number encoded in the next 7 bits. Bit 31 must be 0 and is reserved for future expansion.
pub fn get_spec_version() -> SbiRet {
    sbi_call(eid::BASE, fid::base::GET_SPEC_VERSION, 0, 0)
}

/// Returns the current SBI implementation ID, which is different for every SBI implementation. It is intended that this implementation ID allows software to probe for SBI implementation quirks.
pub fn get_impl_id() -> SbiRet {
    sbi_call(eid::BASE, fid::base::GET_IMPL_ID, 0, 0)
}

/// Returns the current SBI implementation version. The encoding of this version number is specific to the SBI implementation.
pub fn get_impl_version() -> SbiRet {
    sbi_call(eid::BASE, fid::base::GET_IMPL_VERSION, 0, 0)
}

/// Returns 0 if the given SBI extension ID (EID) is not available, or 1 if it is available unless defined as any other non-zero value by the implementation.
pub fn probe_extension(extension_id: usize) -> SbiRet {
    sbi_call(eid::BASE, fid::base::PROBE_EXTENSION, extension_id, 0)
}

pub fn get_mvendorid() -> SbiRet {
    sbi_call(eid::BASE, fid::base::MVENDOR_ID, 0, 0)
}

pub fn get_marchid() -> SbiRet {
    sbi_call(eid::BASE, fid::base::PROBE_EXTENSION, 0, 0)
}

pub fn get_mimpid() -> SbiRet {
    sbi_call(eid::BASE, fid::base::MIMP_ID, 0, 0)
}

/// Timer externsion
pub fn set_timer(stime_value: u64) -> SbiRet {
    sbi_call(eid::TIMER, fid::timer::SET_TIMER, stime_value as usize, 0)
}

/// Ipi externsion
pub fn sent_ipi(hart_mask: usize, hart_mask_base: usize) -> SbiRet {
    sbi_call(eid::IPI, fid::ipi::SEND_IPI, hart_mask, hart_mask_base)
}

/// Rfence extension
pub fn remote_fence_i(hart_mask: usize, hart_mask_base: usize) -> SbiRet {
    sbi_call(eid::RFENCE, fid::rfence::REMOTE_FENCE_I, hart_mask, hart_mask_base)
}

pub fn remote_sfence_vma(hart_mask: usize, hart_mask_base: usize, start_addr: usize, size: usize) -> SbiRet {
    todo!()
}

pub fn remote_sfence_vma_asid(hart_mask: usize, hart_mask_base: usize, start_addr: usize, size: usize, asid: usize) -> SbiRet {
    todo!()
}

/// Hsm extension
pub fn hart_start(hartid: usize, start_addr: usize, opaque: usize) -> SbiRet {
    todo!()
}

pub fn hart_stop() -> SbiRet {
    todo!()
}

pub fn hart_get_status(hartid: usize) -> SbiRet {
    todo!()
}

pub fn hart_suspend(suspend_type: u32, resume_addr: usize, opaque: usize) -> SbiRet {
    todo!()
}

/// System reset extension
pub fn system_reset(reset_type: u32, reset_reason: u32) -> SbiRet {
    todo!()
}

pub fn pmu_num_counters() -> SbiRet {
    todo!()
}

pub fn pmu_counter_get_info(counter_idx: usize) -> SbiRet {
    todo!()
}

#[inline(always)]
fn sbi_call(extension: usize, function: usize, arg0: usize, arg1: usize) -> SbiRet {
    let (error, value);
    match () {
        #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
        () => unsafe { asm!(
            "ecall",
            in("a0") arg0, in("a1") arg1,
            in("a6") function, in("a7") extension,
            lateout("a0") error, lateout("a1") value,
        ) },
        #[cfg(not(any(target_arch = "riscv32", target_arch = "riscv64")))]
        () => {
            drop((extension, function, arg0, arg1));
            unimplemented!("not RISC-V instruction set architecture")
        }
    };
    SbiRet { error, value }
}

mod eid {
    pub const BASE: usize = 0x10;
    pub const TIMER: usize = 0x54494D45;
    pub const IPI: usize = 0x735049;
    pub const RFENCE: usize = 0x52464E43;
}

mod fid {
    pub(super) mod base {
        pub const GET_SPEC_VERSION: usize = 0;
        pub const GET_IMPL_ID: usize = 1;
        pub const GET_IMPL_VERSION: usize = 2;
        pub const PROBE_EXTENSION: usize = 3;
        pub const MVENDOR_ID: usize = 4;
        pub const MARCH_ID: usize = 5;
        pub const MIMP_ID: usize = 6;
    }

    pub(super) mod timer {
        pub const SET_TIMER: usize = 0;
    }

    pub(super) mod ipi {
        pub const SEND_IPI: usize = 0;
    }

    pub(super) mod rfence {
        pub const REMOTE_FENCE_I: usize = 0;
        pub const REMOTE_SFENCE_VMA: usize = 0;
        pub const REMOTE_SFENCE_VMA_ASID: usize = 0;
    }
}

/// Legacy extensions
pub fn legacy_set_timer(stime_value: usize) {
    sbi_call_legacy(SBI_SET_TIMER, stime_value, 0, 0);
}

pub fn legacy_console_putchar(c: usize) {
    sbi_call_legacy(SBI_CONSOLE_PUTCHAR, c, 0, 0);
}

pub fn legacy_console_getchar() -> usize {
    sbi_call_legacy(SBI_CONSOLE_GETCHAR, 0, 0, 0)
}

pub fn legacy_shutdown() -> ! {
    sbi_call_legacy(SBI_SHUTDOWN, 0, 0, 0);
    panic!("It should shutdown!");
}

const SBI_SET_TIMER: usize = 0;
const SBI_CONSOLE_PUTCHAR: usize = 1;
const SBI_CONSOLE_GETCHAR: usize = 2;
const SBI_CLEAR_IPI: usize = 3;
const SBI_SEND_IPI: usize = 4;
const SBI_REMOTE_FENCE_I: usize = 5;
const SBI_REMOTE_SFENCE_VMA: usize = 6;
const SBI_REMOTE_SFENCE_VMA_ASID: usize = 7;
const SBI_SHUTDOWN: usize = 8;

#[inline(always)]
fn sbi_call_legacy(which: usize, arg0: usize, arg1: usize, arg2: usize) -> usize {
    let mut ret;
    unsafe {
        asm!(
            "ecall",
            inlateout("x10") arg0 => ret,
            in("x11") arg1,
            in("x12") arg2,
            in("x17") which,
        );
    }
    ret
}