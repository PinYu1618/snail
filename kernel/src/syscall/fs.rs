use crate::mm::{PageTable, UserBuffer};
use crate::task_::Processor;
use crate::fs;

pub fn sys_open(path: *const u8, flags: u32) -> isize {
    let process = Processor::current_process().unwrap();
    let token = Processor::current_user_token();
    let path = PageTable::translated_str(token, path);
    if let Some(inode) = fs::open_file(path.as_str(), fs::OpenFlags::from_bits(flags).unwrap()) {
        let mut inner = process.inner_exclusive_access();
        let fd = inner.alloc_fd();
        inner.fd_table[fd] = Some(inode);
        fd as isize
    } else {
        -1
    }
}

pub fn sys_close(fd: usize) -> isize {
    let pcb = Processor::current_process().unwrap();
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
    let token = Processor::current_user_token();
    let pcb = Processor::current_process().unwrap();
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
        file.read(UserBuffer::new(PageTable::translated_byte_buf(
            token, buf, len,
        ))) as isize
    } else {
        -1
    }
}

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    let token = Processor::current_user_token();
    let pcb = Processor::current_process().unwrap();
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
        file.write(UserBuffer::new(PageTable::translated_byte_buf(
            token, buf, len,
        ))) as isize
    } else {
        -1
    }
}

pub fn sys_pipe(_pipe: *mut usize) -> isize {
    let _task = Processor::current_process().unwrap();
    let _token = Processor::current_user_token();
    unimplemented!()
}

//pub fn sys_linkat()
