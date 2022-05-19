use core::arch::asm;

#[allow(dead_code)]
#[repr(usize)]
enum SyscallNr {
    IoSetup = 0,
    IoDestroy = 1,
    IoSubmit = 2,
    Dup = 23,
    Dup3 = 24,
    Fcntl = 25,
    UnlinkAt = 35,
    SymLinkAt = 36,
    LinkAt = 37,
    RenameAt = 38,
    Chdir = 49,
    Chroot = 51,
    Open = 56,
    Close = 57,
    Pipe = 59,
    Mount = 40,
    Read = 63,
    Write = 64,
    Fstat = 80,
    Exit = 93,
    SysLog = 116,
    Yield = 124,
    Kill = 129,
    Fork = 220,
    Exec = 221,
    WaitPid = 260,
}

pub(crate) fn sys_open(path: &str, flags: u32) -> isize {
    syscall(SyscallNr::Open, [path.as_ptr() as usize, flags as usize, 0, 0, 0, 0])
}

pub(crate) fn sys_close(fd: usize) -> isize {
    syscall(SyscallNr::Close, [fd, 0, 0, 0, 0, 0])
}

pub(crate) fn sys_pipe(pipe: &mut [usize]) -> isize {
    syscall(SyscallNr::Pipe, [pipe.as_mut_ptr() as usize, 0, 0, 0, 0, 0])
}

pub(crate) fn sys_read(fd: usize, buf: &mut [u8]) -> isize {
    syscall(SyscallNr::Read, [fd, buf.as_mut_ptr() as usize, buf.len(), 0, 0, 0])
}

pub(crate) fn sys_write(fd: usize, buf: &[u8]) -> isize {
    syscall(SyscallNr::Write, [fd, buf.as_ptr() as usize, buf.len(), 0, 0, 0])
}

pub(crate) fn sys_exit(xstate: i32) -> ! {
    syscall(SyscallNr::Exit, [xstate as usize, 0, 0, 0, 0, 0]);
    panic!("Should not reach here.")
}

pub(crate) fn sys_yield() -> isize {
    syscall(SyscallNr::Yield, [0, 0, 0, 0, 0, 0])
}

pub(crate) fn sys_fork() -> isize {
    syscall(SyscallNr::Fork, [0, 0, 0, 0, 0, 0])
}

pub(crate) fn sys_exec(path: &str) -> isize {
    syscall(SyscallNr::Exec, [path.as_ptr() as usize, 0, 0, 0, 0, 0])
}

pub(crate) fn sys_waitpid(_pid: isize, _exit_code: *mut i32) -> isize {
    todo!()
}

fn syscall(id: SyscallNr, args: [usize; 6]) -> isize {
    let mut ret: isize;
    unsafe {
        asm!(
            "ecall",
            inlateout("x10") args[0] => ret,
            in("x11") args[1],
            in("x12") args[2],
            in("x13") args[3],
            in("x14") args[4],
            in("x15") args[5],
            in("x17") id as usize
        );
    }
    ret
}