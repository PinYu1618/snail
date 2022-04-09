#![no_std]
#![allow(unused)]

extern crate alloc;

mod block_dev;
mod cache;
mod layout;
mod bitmap;
mod sfs;
mod vfs;

pub const BLOCK_SZ: usize = 512;
pub use block_dev::BlockDev;
pub use sfs::SnailFileSystem;
pub use vfs::Inode;

use cache::{ BlockCache, BlockCacher, BLOCK_CACHER, cache_block };
use layout::*;
use bitmap::Bitmap;
