mod inode;
pub mod stdio;

use crate::mm::page::UserBuf;

pub trait File: Send + Sync {
    fn read(&self, buf: UserBuf) -> usize;
    fn write(&self, buf: UserBuf) -> usize;
    fn readable(&self) -> bool;
    fn writable(&self) -> bool;
}
