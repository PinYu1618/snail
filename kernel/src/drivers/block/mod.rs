pub mod virtio_blk;

use snail_fs::BlockDevice;
use alloc::sync::Arc;

type BlockDevImpl = virtio_blk::VirtIOBlock;

lazy_static! {
    pub static ref BLOCK_DEV: Arc<dyn BlockDevice> = Arc::new(BlockDevImpl::new());
}
