mod stdio;
mod fcntl;
mod inode;

use crate::mm::page::UserBuffer;

pub trait File : Send + Sync {
    fn read(&self, buf: UserBuffer) -> usize;
    fn write(&self, buf: UserBuffer) -> usize;
    fn readable(&self) -> bool;
    fn writable(&self) -> bool;
}