#![no_std]

#[macro_use]
extern crate lazy_static;
extern crate alloc;

pub mod bitmap;
pub mod block_dev;
pub mod cache;
pub mod layout;
pub mod sfs;
pub mod vfs;

pub const BLOCK_SIZE: usize = 512;

/// Re-export
pub use bitmap::Bitmap;
pub use block_dev::BlockDevice;
pub use cache::BlockCacher;
pub use sfs::SnailFileSystem;
pub use vfs::Inode;
use layout::*;
