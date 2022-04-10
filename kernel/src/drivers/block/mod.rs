pub mod virtio_blk;

use alloc::sync::Arc;
use lazy_static::lazy_static;
use snail_fs::BlockDev;

type BlockDevImpl = virtio_blk::VirtIOBlock;

lazy_static! {
    pub static ref BLOCK_DEV: Arc<dyn BlockDev> = Arc::new(BlockDevImpl::new());
}
