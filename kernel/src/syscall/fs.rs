pub const FD_STDOUT: usize = 1;

//pub fn sys_linkat()

use crate::{
    mm::page::translated_str,
    task::processor::{current_process, current_user_token},
};

pub fn sys_close(fd: usize) -> isize {
    unimplemented!()
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
    let token = current_user_token().unwrap();
    let path = translated_str(token, path);

    unimplemented!()
}
