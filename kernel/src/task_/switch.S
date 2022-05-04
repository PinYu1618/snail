.altmacro
.macro SAVE_SN n
    sd s\n, (\n+2)*8(a0)
.endm
.altmacro
.macro LOAD_SN n
    ld s\n, (\n+2)*8(a1)
.endm
    .section .text
    .globl __switch

__switch:
#   __switch(
#       cur_task_cx_ptr: *mut ProcessContext,
#       next_task_cx_ptr: *const ProcessContext,
#   )
# save kernel stack for current task
    sd sp, 8(a0)
# save ra(=return address) and s0~s11 of current execution
    sd ra, 0(a0)
    .set n, 0
    .rept 12
        SAVE_SN %n
        .set n, n+1
    .endr
# load ra and s0~s11 of next execution
    ld ra, 0(a1)
    .set n, 0
    .rept 12
        SAVE_SN %n
        .set n, n+1
    .endr
# load kernel stack for next task
    ld sp, 8(a1)
    ret