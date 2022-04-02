#
#   Code Ref:
#   (https://rcore-os.github.io/rCore-Tutorial-Book-v3/chapter2/6multitasking-based-on-as.html)
#

.altmacro
.macro SAVE_GP n
    sd x\n, \n*8(sp)
.endm
.macro LOAD_GP n
    ld x\n, \n*8(sp)
.endm

    .section .text.trampoline
    .globl __alltraps
    .globl __restore
    .align 2

__alltraps:
    csrrw sp, sscratch, sp    # sp -> *TrapContext in user space, sscratch -> user stack

# save general-purpose registers

    sd x1, 1*8(sp)    # save x1. skip sp(x2), we will save it later
    sd x3, 3*8(sp)    # save x3. skip tp(x4), application does not use it
    .set n, 5         # save x5~x31
    .rept 27
        SAVE_GP %n
        .set n, n+1
    .endr

# save other registers
# Note: we can use t0/t1/t2 freely, because they were saved in TrapContext

    csrr t0, sstatus
    csrr t1, sepc
    sd t0, 32*8(sp)
    sd t1, 33*8(sp)

# read user stack from sscratch and save it in TrapContext

    csrr t2, sscratch
    sd t2, 2*8(sp)

    ld t0, 34*8(sp)    # t0 <- kernel_satp
    ld t1, 36*8(sp)    # t1 <- trap_handler

    ld sp, 35*8(sp)    # move to kernel_sp
    csrw satp, t0      # switch to kernel space
    sfence.vma
    jr t1              # jump to trap_handler

__restore:

# switch to user space
# a0: *TrapContext in user space (constant)
# a1: user space token

    csrw satp, a1
    sfence.vma
    csrw sscratch, a0
    mv sp, a0

# now sp->TrapContext in user space, start restoring based on it
# restore sstatus/sepc

    ld t0, 32*8(sp)
    ld t1, 33*8(sp)
    csrw sstatus, t0
    csrw sepc, t1

# restore general-purpuse registers except x0/sp/tp

    ld x1, 1*8(sp)
    ld x3, 3*8(sp)
    .set n, 5
    .rept 27
        LOAD_GP %n
        .set n, n+1
    .endr

# back to user stack

    ld sp, 2*8(sp)
    sret