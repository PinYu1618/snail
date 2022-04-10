use lazy_static::lazy_static;
use spin::Mutex;

use alloc::sync::Arc;

use snail_fs::{Inode, SnailFileSystem};

use crate::{drivers::block::BLOCK_DEV, mm::page::UserBuf};

use super::File;

/// A regular file or directory which is opened in a process
pub struct KInode {
    readable: bool,
    writable: bool,
    inner: Mutex<KInodeInner>,
}

pub struct KInodeInner {
    /// current reading/writng offset of the file
    offset: usize,
    inode: Arc<Inode>,
}

bitflags! {
    pub struct OpenFlags: u32 {
        const RDONLY = 0;
        const WRONLY = 1 << 0;
        const RDWR = 1 << 1;
        const CREATE = 1 << 9;
        const TRUNC = 1 << 10;
    }
}

impl KInode {
    pub fn new(readable: bool, writable: bool, inode: Arc<Inode>) -> Self {
        Self {
            readable,
            writable,
            inner: Mutex::new(KInodeInner { offset: 0, inode }),
        }
    }
}

impl File for KInode {
    fn read(&self, mut buf: UserBuf) -> usize {
        let mut inner = self.inner.lock();
        let mut total_read_size = 0_usize;
        for slice in buf.buffers.iter_mut() {
            let read_size = inner.inode.read_at(inner.offset, *slice);
            if read_size == 0 {
                break;
            }
            inner.offset += read_size;
            total_read_size += read_size;
        }
        total_read_size
    }

    fn write(&self, buf: UserBuf) -> usize {
        let mut inner = self.inner.lock();
        let mut total_write_size = 0_usize;
        for slice in buf.buffers.iter() {
            let write_size = inner.inode.write_at(inner.offset, *slice);
            assert_eq!(write_size, slice.len()); // why check this ?
            inner.offset += write_size;
            total_write_size += write_size;
        }
        total_write_size
    }

    fn readable(&self) -> bool {
        self.readable
    }

    fn writable(&self) -> bool {
        self.writable
    }
}

impl OpenFlags {
    /// Do not check validity for simplicity
    /// Return (readable, writable)
    pub fn read_write(&self) -> (bool, bool) {
        if self.is_empty() {
            (true, false)
        } else if self.contains(Self::WRONLY) {
            (false, true)
        } else {
            (true, true)
        }
    }
}

lazy_static! {
    pub static ref ROOT_INODE: Arc<Inode> = {
        let sfs = SnailFileSystem::open(BLOCK_DEV.clone());
        Arc::new(SnailFileSystem::root_inode(&sfs))
    };
}

pub fn list_all_apps() {
    println!("-------- APPS --------");
    for app in ROOT_INODE.ls() {
        println!("{}", app);
    }
    println!("----------------------");
}

pub fn open_file(name: &str, flags: OpenFlags) -> Option<Arc<KInode>> {
    let (readable, writable) = flags.read_write();
    if flags.contains(OpenFlags::CREATE) {
        if let Some(inode) = ROOT_INODE.find(name) {
            // clear size
            inode.clear();
            Some(Arc::new(KInode::new(readable, writable, inode)))
        } else {
            // create file
            ROOT_INODE
                .create(name)
                .map(|inode| Arc::new(KInode::new(readable, writable, inode)))
        }
    } else {
        ROOT_INODE.find(name).map(|inode| {
            if flags.contains(OpenFlags::TRUNC) {
                inode.clear();
            }
            Arc::new(KInode::new(readable, writable, inode))
        })
    }
}
