use spin::Mutex;

use alloc::sync::Arc;

use snail_fs::Inode;

use crate::mm::page::UserBuffer;

use super::File;

pub struct KInode {
    readable: bool,
    writable: bool,
    inner: Mutex<KInodeInner>,
}

pub struct KInodeInner {
    offset: usize,
    inode: Arc<Inode>,
}

impl KInode {
    pub fn new(readable: bool, writable: bool, inode: Arc<Inode>) -> Self {
        Self {
            readable,
            writable,
            inner: Mutex::new(KInodeInner {
                offset: 0,
                inode,
            }),
        }
    }
}

impl File for KInode {
    fn read(&self, mut buf: UserBuffer) -> usize {
        unimplemented!()
    }

    fn write(&self, buf: UserBuffer) -> usize {
        unimplemented!()
    }

    fn readable(&self) -> bool { self.readable }

    fn writable(&self) -> bool { self.writable }
}