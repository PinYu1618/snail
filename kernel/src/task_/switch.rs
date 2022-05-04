use core::arch::global_asm;

use super::context::TaskContext;

extern "C" {
    pub fn __switch(cur_task_cx_ptr: *mut TaskContext, next_task_cx_ptr: *const TaskContext);
}

global_asm!(include_str!("switch.S"));
