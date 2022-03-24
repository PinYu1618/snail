#![no_std]
#![allow(unused)]

#[macro_use]
mod console;
mod syscall;

use syscall::*;

pub fn write(fd: usize, buf: &[u8]) -> isize { sys_write(fd, buf) }
pub fn exit(exit_code: i32) -> isize { sys_exit(exit_code) }