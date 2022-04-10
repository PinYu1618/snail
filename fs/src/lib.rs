#![no_std]
#![allow(unused)]

extern crate alloc;

mod bitmap;
mod block_dev;
mod cache;
mod layout;
mod sfs;
mod vfs;

pub const BLOCK_SZ: usize = 512;
pub use block_dev::BlockDev;
pub use sfs::SnailFileSystem;
pub use vfs::Inode;

use bitmap::Bitmap;
use cache::{cache_block, BlockCache, BlockCacher, BLOCK_CACHER};
use layout::*;
