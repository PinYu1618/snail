mod fd;
pub mod inode;
pub mod pipe;
pub mod stdio;

use crate::mm::UserBuffer;

pub trait File: Send + Sync {
    fn read(&self, buf: UserBuffer) -> usize;
    fn write(&self, buf: UserBuffer) -> usize;
    fn readable(&self) -> bool;
    fn writable(&self) -> bool;
}

/// Re-export
pub use fd::{FileDescriptor, FileDescriptorTable};
pub use inode::{list_all_apps, open_file, OpenFlags};
pub use pipe::make_pipe;
pub use stdio::{Stdin, Stdout, Stderr};
