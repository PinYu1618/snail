use crate::{
    mm::page::{translated_str, UserBuf, translated_byte_buf},
    task::processor::{current_process, current_user_token},
};

pub fn sys_open(path: *const u8, flags: u32) -> isize {
    let process = current_process().unwrap();
    let token = current_user_token();
    let path = translated_str(token, path);

    unimplemented!()
}

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
    let token = current_user_token();
    let pcb = current_process().unwrap();
    let inner = pcb.inner_exclusive_access();
    if fd >= inner.fd_table.len() {
        return -1;
    }
    if let Some(file) = &inner.fd_table[fd] {
        let file = file.clone();
        if !file.readable() {
            return -1;
        }
        // release current task pcb manually to avoid multi-borrow
        drop(inner);
        file.read(UserBuf::new(translated_byte_buf(token, buf, len))) as isize
    } else {
        -1
    }
}

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    let token = current_user_token();
    let pcb = current_process().unwrap();
    let inner = pcb.inner_exclusive_access();
    if fd >= inner.fd_table.len() {
        return -1;
    }
    if let Some(file) = &inner.fd_table[fd] {
        if !file.writable() {
            return -1;
        }
        let file = file.clone();
        // release current task TCB manually to avoid multi-borrow
        drop(inner);
        file.write(UserBuf::new(translated_byte_buf(token, buf, len))) as isize
    } else {
        -1
    }
}

//pub fn sys_linkat()
