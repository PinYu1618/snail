pub mod inode;
pub mod stdio;

use crate::mm::UserBuffer;

pub use inode::{open_file, OpenFlags};
pub use stdio::{Stdin, Stdout};

pub trait File: Send + Sync {
    fn read(&self, buf: UserBuffer) -> usize;
    fn write(&self, buf: UserBuffer) -> usize;
    fn readable(&self) -> bool;
    fn writable(&self) -> bool;
}

pub use inode::list_all_apps;
