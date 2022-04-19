use super::File;
use crate::mm::UserBuffer;

pub struct Stdin;

impl File for Stdin {
    fn read(&self, mut _buf: UserBuffer) -> usize {
        unimplemented!()
    }

    fn write(&self, _buf: UserBuffer) -> usize {
        panic!("Cannot write to Stdin.");
    }

    fn readable(&self) -> bool {
        true
    }

    fn writable(&self) -> bool {
        false
    }
}

pub struct Stdout;

impl File for Stdout {
    fn read(&self, _user_buf: UserBuffer) -> usize {
        panic!("Read is not supported for stdout");
    }

    fn write(&self, user_buf: UserBuffer) -> usize {
        for buf in user_buf.buffers.iter() {
            print!("{}", core::str::from_utf8(*buf).unwrap());
        }
        user_buf.len()
    }

    fn readable(&self) -> bool {
        false
    }

    fn writable(&self) -> bool {
        true
    }
}
