pub const FD_STDOUT: usize = 1;

use crate::{
    mm::page::translated_str,
    task::processor::{current_process, current_user_token},
};

pub fn sys_close(fd: usize) -> isize {
    let pcb = current_process().unwrap();
    let mut inner = pcb.inner_exclusive_access();
    if fd >= inner.fd_table.len() {
        return -1;
    }
    if inner.fd_table[fd].is_none() {
        return -1;
    }
    inner.fd_table[fd].take();
    0
}

pub fn sys_read(fd: usize, buf: *const u8, len: usize) -> isize {
    unimplemented!()
}

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    match fd {
        FD_STDOUT => {
            let slice = unsafe { core::slice::from_raw_parts(buf, len) };
            let string = core::str::from_utf8(slice).unwrap();
            print!("{}", string);
            len as isize
        }
        _ => {
            panic!("Unsupported fd in sys_write!");
        }
    }
}

pub fn sys_open(path: *const u8, flags: u32) -> isize {
    let process = current_process().unwrap();
    let token = current_user_token();
    let path = translated_str(token, path);

    unimplemented!()
}

//pub fn sys_linkat()
