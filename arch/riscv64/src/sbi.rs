///! Note! user of these sbi functions (aka supervisor) must save all
///! registers except a0, a1 before calling it.
///! 
///! Ref: rustsbi-qemu/test-kernel/src/sbi.rs

use core::arch::asm;
use core::fmt;

// Base extensions

/// Returns the current SBI specification version. This function must always succeed. The minor number of the SBI specification is encoded in the low 24 bits, with the major number encoded in the next 7 bits. Bit 31 must be 0 and is reserved for future expansion.
#[inline]
pub fn get_spec_version() -> usize {
    sbi_call_0(eid::BASE, fid::base::GET_SPEC_VERSION).value
}

/// Returns the current SBI implementation ID, which is different for every SBI implementation. It is intended that this implementation ID allows software to probe for SBI implementation quirks.
#[inline]
pub fn get_impl_id() -> usize {
    sbi_call_0(eid::BASE, fid::base::GET_IMPL_ID).value
}

/// Returns the current SBI implementation version. The encoding of this version number is specific to the SBI implementation.
#[inline]
pub fn get_impl_version() -> usize {
    sbi_call_0(eid::BASE, fid::base::GET_IMPL_VERSION).value
}

/// Returns 0 if the given SBI extension ID (EID) is not available, or 1 if it is available unless defined as any other non-zero value by the implementation.
#[inline]
pub fn probe_extension(extension_id: usize) -> usize {
    sbi_call_1(eid::BASE, fid::base::PROBE_EXTENSION, extension_id).value
}

#[inline]
pub fn get_mvendorid() -> usize {
    sbi_call_0(eid::BASE, fid::base::MVENDOR_ID).value
}

#[inline]
pub fn get_marchid() -> usize {
    sbi_call_0(eid::BASE, fid::base::MARCH_ID).value
}

#[inline]
pub fn get_mimpid() -> usize {
    sbi_call_0(eid::BASE, fid::base::MIMP_ID).value
}

// Timer externsion

pub fn set_timer(stime_value: u64) -> SbiRet {
    sbi_call_1(eid::TIMER, fid::timer::SET_TIMER, stime_value as usize)
}

// Ipi externsion

pub fn send_ipi(hart_mask: usize, hart_mask_base: usize) -> SbiRet {
    sbi_call_2(eid::IPI, fid::ipi::SEND_IPI, hart_mask, hart_mask_base)
}

// Rfence extension

pub fn remote_fence_i(hart_mask: usize, hart_mask_base: usize) -> SbiRet {
    sbi_call_2(eid::RFENCE, fid::rfence::REMOTE_FENCE_I, hart_mask, hart_mask_base)
}

#[allow(unused)]
pub fn remote_sfence_vma(hart_mask: usize, hart_mask_base: usize, start_addr: usize, size: usize) -> SbiRet {
    todo!()
}

#[allow(unused)]
pub fn remote_sfence_vma_asid(hart_mask: usize, hart_mask_base: usize, start_addr: usize, size: usize, asid: usize) -> SbiRet {
    todo!()
}

// Hsm extension

pub fn hart_start(hartid: usize, start_addr: usize, opaque: usize) -> SbiRet {
    sbi_call_3(eid::HSM, fid::hsm::HART_START, hartid, start_addr, opaque)
}

pub fn hart_stop(hartid: usize) -> SbiRet {
    sbi_call_1(eid::HSM, fid::hsm::HART_STOP, hartid)
}

pub fn hart_get_status(hartid: usize) -> SbiRet {
    sbi_call_1(eid::HSM, fid::hsm::HART_GET_STATUS, hartid)
}

pub fn hart_suspend(suspend_type: u32, resume_addr: usize, opaque: usize) -> SbiRet {
    sbi_call_3(eid::HSM, fid::hsm::HART_SUSPEND, suspend_type as usize, resume_addr, opaque)
}

// System reset extension

#[inline]
pub fn system_reset(reset_type: usize, reset_reason: usize) -> SbiRet {
    sbi_call_2(eid::SRST, fid::srst::SYSTEM_RESET, reset_type, reset_reason)
}

pub fn shutdown() -> ! {
    sbi_call_2(eid::SRST, fid::srst::SYSTEM_RESET, RESET_TYPE_SHUTDOWN, RESET_REASON_NO_REASON);
    unreachable!()
}

pub fn pmu_num_counters() -> SbiRet {
    todo!()
}

#[allow(unused)]
pub fn pmu_counter_get_info(counter_idx: usize) -> SbiRet {
    todo!()
}

// Legacy extensions

pub fn legacy_set_timer(stime_value: usize) {
    sbi_call_legacy(fid::legacy::SET_TIMER, stime_value, 0, 0);
}

pub fn legacy_console_putchar(c: usize) {
    sbi_call_legacy(fid::legacy::CONSOLE_PUTCHAR, c, 0, 0);
}

pub fn legacy_console_getchar() -> usize {
    sbi_call_legacy(fid::legacy::CONSOLE_GETCHAR, 0, 0, 0)
}

pub fn legacy_shutdown() -> ! {
    sbi_call_legacy(fid::legacy::SHUTDOWN, 0, 0, 0);
    unreachable!()
}

pub const RESET_TYPE_SHUTDOWN: usize = 0x0000_0000;
pub const RESET_TYPE_COLD_REBOOT: usize = 0x0000_0001;
pub const RESET_TYPE_WARM_REBOOT: usize = 0x0000_0002;
pub const RESET_REASON_NO_REASON: usize = 0x0000_0000;
pub const RESET_REASON_SYSTEM_FAILURE: usize = 0x0000_0001;

// Binary Encoding

#[repr(C)]
pub struct SbiRet {
    pub error: usize,
    pub value: usize,
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

impl fmt::Debug for SbiRet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.error {
            SBI_SUCCESS => write!(f, "{:?}", self.value),
            SBI_ERR_FAILED => write!(f, "<SBI call failed>"),
            SBI_ERR_NOT_SUPPORTED => write!(f, "<SBI feature not supported>"),
            SBI_ERR_INVALID_PARAM => write!(f, "<SBI invalid parameter>"),
            SBI_ERR_DENIED => write!(f, "<SBI denied>"),
            SBI_ERR_INVALID_ADDRESS => write!(f, "<SBI invalid address>"),
            SBI_ERR_ALREADY_AVAILABLE => write!(f, "<SBI already available>"),
            SBI_ERR_ALREADY_STARTED => write!(f, "<SBI already started>"),
            SBI_ERR_ALREADY_STOPPED => write!(f, "<SBI already stopped>"),
            unknown => write!(f, "[SBI Unknown error: {}]", unknown),
        }
    }
}

pub mod eid {
    pub const BASE: usize = 0x10;
    pub const TIMER: usize = 0x54494D45;
    pub const IPI: usize = 0x735049;
    pub const RFENCE: usize = 0x52464E43;
    pub const HSM: usize = 0x48534D;
    pub const SRST: usize = 0x53525354;
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

    pub(super) mod hsm {
        pub const HART_START: usize = 0x0;
        pub const HART_STOP: usize = 0x1;
        pub const HART_GET_STATUS: usize = 0x2;
        pub const HART_SUSPEND: usize = 0x3;
    }

    #[allow(unused)]
    pub(super) mod rfence {
        pub const REMOTE_FENCE_I: usize = 0;
        pub const REMOTE_SFENCE_VMA: usize = 0;
        pub const REMOTE_SFENCE_VMA_ASID: usize = 0;
    }

    pub(super) mod srst {
        pub const SYSTEM_RESET: usize = 0x0;
    }

    #[allow(unused)]
    pub(super) mod legacy {
        pub const SET_TIMER: usize = 0;
        pub const CONSOLE_PUTCHAR: usize = 1;
        pub const CONSOLE_GETCHAR: usize = 2;
        pub const CLEAR_IPI: usize = 3;
        pub const SEND_IPI: usize = 4;
        pub const REMOTE_FENCE_I: usize = 5;
        pub const REMOTE_SFENCE_VMA: usize = 6;
        pub const REMOTE_SFENCE_VMA_ASID: usize = 7;
        pub const SHUTDOWN: usize = 8;
    }
}

const SBI_SUCCESS: usize = 0;
const SBI_ERR_FAILED: usize = usize::from_ne_bytes(isize::to_ne_bytes(-1));
const SBI_ERR_NOT_SUPPORTED: usize = usize::from_ne_bytes(isize::to_ne_bytes(-2));
const SBI_ERR_INVALID_PARAM: usize = usize::from_ne_bytes(isize::to_ne_bytes(-3));
const SBI_ERR_DENIED: usize = usize::from_ne_bytes(isize::to_ne_bytes(-4));
const SBI_ERR_INVALID_ADDRESS: usize = usize::from_ne_bytes(isize::to_ne_bytes(-5));
const SBI_ERR_ALREADY_AVAILABLE: usize = usize::from_ne_bytes(isize::to_ne_bytes(-6));
const SBI_ERR_ALREADY_STARTED: usize = usize::from_ne_bytes(isize::to_ne_bytes(-7));
const SBI_ERR_ALREADY_STOPPED: usize = usize::from_ne_bytes(isize::to_ne_bytes(-8));

#[inline(always)]
fn sbi_call_0(extension: usize, function: usize) -> SbiRet {
    let (error, value);
    match () {
        #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
        () => unsafe {
            asm!(
                "ecall",
                in("a6") function, in("a7") extension,
                lateout("a0") error, lateout("a1") value,
            )
        },
        #[cfg(not(any(target_arch = "riscv32", target_arch = "riscv64")))]
        () => {
            drop((extension, function));
            unimplemented!("not RISC-V instruction set architecture")
        }
    };
    SbiRet { error, value }
}

#[inline(always)]
fn sbi_call_1(extension: usize, function: usize, arg0: usize) -> SbiRet {
    let (error, value);
    match () {
        #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
        () => unsafe {
            asm!(
                "ecall",
                in("a0") arg0,
                in("a6") function, in("a7") extension,
                lateout("a0") error, lateout("a1") value,
            )
        },
        #[cfg(not(any(target_arch = "riscv32", target_arch = "riscv64")))]
        () => {
            drop((extension, function, arg0));
            unimplemented!("not RISC-V instruction set architecture")
        }
    };
    SbiRet { error, value }
}

#[inline(always)]
fn sbi_call_2(extension: usize, function: usize, arg0: usize, arg1: usize) -> SbiRet {
    let (error, value);
    match () {
        #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
        () => unsafe {
            asm!(
                "ecall",
                in("a0") arg0, in("a1") arg1,
                in("a6") function, in("a7") extension,
                lateout("a0") error, lateout("a1") value,
            )
        },
        #[cfg(not(any(target_arch = "riscv32", target_arch = "riscv64")))]
        () => {
            drop((extension, function, arg0, arg1));
            unimplemented!("not RISC-V instruction set architecture")
        }
    };
    SbiRet { error, value }
}

#[inline(always)]
fn sbi_call_3(extension: usize, function: usize, arg0: usize, arg1: usize, arg2: usize) -> SbiRet {
    let (error, value);
    match () {
        #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
        () => unsafe {
            asm!(
                "ecall",
                in("a0") arg0, in("a1") arg1, in("a2") arg2,
                in("a6") function, in("a7") extension,
                lateout("a0") error, lateout("a1") value,
            )
        },
        #[cfg(not(any(target_arch = "riscv32", target_arch = "riscv64")))]
        () => {
            drop((extension, function, arg0, arg1, arg2));
            unimplemented!("not RISC-V instruction set architecture")
        }
    };
    SbiRet { error, value }
}

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