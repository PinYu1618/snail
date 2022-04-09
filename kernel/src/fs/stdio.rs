use crate::mm::page::UserBuf;

use super::File;

pub struct Stdin;

pub struct Stdout;

impl File for Stdin {
    fn read(&self, mut buf: UserBuf) -> usize {
        unimplemented!()
    }

    fn write(&self, buf: UserBuf) -> usize {
        unimplemented!()
    }

    fn readable(&self) -> bool { true }

    fn writable(&self) -> bool { false }
}

impl File for Stdout {
    fn read(&self, _user_buf: UserBuf) -> usize {
        panic!("Read is not supported for stdout");
    }

    fn write(&self, user_buf: UserBuf) -> usize {
        for buf in user_buf.buffers.iter() {
            print!("{}", core::str::from_utf8(*buf).unwrap());
        }
        user_buf.len()
    }

    fn readable(&self) -> bool { false }

    fn writable(&self) -> bool { true }
}