use core::arch::global_asm;

use super::context::ProcessContext;

extern "C" {
    pub fn __switch(cur_task_cx_ptr: *mut ProcessContext, next_task_cx_ptr: *const ProcessContext);
}

global_asm!(include_str!("switch.S"));
