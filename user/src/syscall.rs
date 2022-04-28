use core::arch::asm;

const NR_UNLINKAT: usize = 35;
const NR_LINKAT: usize = 37;
const NR_OPEN: usize = 56;
const NR_CLOSE: usize = 57;
const NR_PIPE: usize = 59;
const NR_READ: usize = 63;
const NR_WRITE: usize = 64;
const NR_FSTAT: usize = 80;
const NR_EXIT: usize = 93;
const NR_YIELD: usize = 124;
const NR_GET_TIME: usize = 169;
const NR_FORK: usize = 220;
const NR_EXEC: usize = 221;
const NR_WAITPID: usize = 260;
const NR_TASK_INFO: usize = 410;

pub fn sys_open(path: &str, flags: u32) -> isize {
    syscall(NR_OPEN, [path.as_ptr() as usize, flags as usize, 0])
}

pub fn sys_close(fd: usize) -> isize {
    syscall(NR_CLOSE, [fd, 0, 0])
}

pub fn sys_pipe(pipe: &mut [usize]) -> isize {
    syscall(NR_PIPE, [pipe.as_mut_ptr() as usize, 0, 0])
}

pub fn sys_read(fd: usize, buf: &mut [u8]) -> isize {
    syscall(NR_READ, [fd, buf.as_mut_ptr() as usize, buf.len()])
}

pub fn sys_write(fd: usize, buf: &[u8]) -> isize {
    syscall(NR_WRITE, [fd, buf.as_ptr() as usize, buf.len()])
}

pub fn sys_exit(xstate: i32) -> ! {
    syscall(NR_EXIT, [xstate as usize, 0, 0]);
    panic!("Should never exit.");
}

pub fn sys_yield() -> isize {
    syscall(NR_YIELD, [0, 0, 0])
}

pub fn sys_fork() -> isize {
    unimplemented!()
}

pub fn sys_exec(path: &str) -> isize {
    syscall(NR_EXEC, [path.as_ptr() as usize, 0, 0])
}

pub fn sys_waitpid(pid: isize, exit_code: *mut i32) -> isize {
    unimplemented!()
}

fn syscall(id: usize, args: [usize; 3]) -> isize {
    let mut ret: isize;
    unsafe {
        asm!(
            "ecall",
            inlateout("x10") args[0] => ret,
            in("x11") args[1],
            in("x12") args[2],
            in("x17") id
        );
    }
    ret
}
