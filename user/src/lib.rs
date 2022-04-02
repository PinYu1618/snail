#![no_std]
#![allow(unused)]

#[macro_use]
mod console;
mod syscall;

use syscall::*;

pub fn close(fd: usize) -> isize { sys_close(fd) }

pub fn read(fd: usize, buf: &mut [u8]) -> isize { sys_read(fd, buf) }

pub fn write(fd: usize, buf: &[u8]) -> isize { sys_write(fd, buf) }

pub fn exit(exit_code: i32) -> isize { sys_exit(exit_code) }

pub fn yield_() -> isize { sys_yield() }

pub fn wait(exit_code: &mut i32) -> isize {
    loop {
        match sys_waitpid(-1, exit_code as *mut _) {
            -2 => { yield_(); }
            // -1 or a real pid
            exit_pid => return exit_pid
        }
    }
}

pub fn waitpid(pid: usize, exit_code: &mut i32) -> isize {
    loop {
        match sys_waitpid(pid as isize, exit_code as *mut _) {
            -2 => { yield_(); }
            // -1 or a real pid
            exit_pid => return exit_pid
        }
    }
}

pub fn fstat(fd: i32, st: *mut Stat) -> i32 {
    unimplemented!()
}

pub fn unlinkat(dirfd: i32, path: *const u8, flags: u32) -> i32 {
    unimplemented!()
}

pub fn linkat(
    olddirfd: i32, oldpath: *const u8, newdirfd: i32, newpath: *const u8, flags: u32
) -> i32 {
    unimplemented!()
}

// ^TODO
pub struct Stat;