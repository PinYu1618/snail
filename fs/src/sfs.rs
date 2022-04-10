use spin::Mutex;

use alloc::sync::Arc;

use super::{
    cache_block, Bitmap, BlockDev, DataBlock, DiskInode, DiskInodeType, Inode, SuperBlock, BLOCK_SZ,
};

pub struct SnailFileSystem {
    pub block_dev: Arc<dyn BlockDev>,
    /// inode bitmap
    pub imap: Bitmap,
    /// data bitmap
    pub dmap: Bitmap,
    /// inode zone start
    izone_start: u32,
    /// data zone start
    dzone_start: u32,
}

impl SnailFileSystem {
    /// create and initialize a fs on the block device
    pub fn create(
        block_dev: Arc<dyn BlockDev>,
        total_blocks: u32,
        imap_blocks: u32,
    ) -> Arc<Mutex<Self>> {
        // calculate block size of areas & create bitmaps
        let imap = Bitmap::new(1, imap_blocks as usize);
        let inode_num = imap.max();
        let izone_blocks =
            ((inode_num * core::mem::size_of::<DiskInode>() + BLOCK_SZ - 1) / BLOCK_SZ) as u32;
        let itotal_blocks = imap_blocks + izone_blocks;
        let dtotal_blocks = total_blocks - 1 - itotal_blocks;
        let dmap_blocks = (dtotal_blocks + 4096) / 4097;
        let dzone_blocks = dtotal_blocks - dmap_blocks;
        let dmap = Bitmap::new(
            (1 + imap_blocks + izone_blocks) as usize,
            dmap_blocks as usize,
        );
        let mut sfs = Self {
            block_dev: Arc::clone(&block_dev),
            imap,
            dmap,
            izone_start: 1 + imap_blocks,
            dzone_start: 1 + itotal_blocks + dmap_blocks,
        };
        // clear all blocks
        for i in 0..total_blocks {
            cache_block(i as usize, Arc::clone(&block_dev))
                .lock()
                .modify(0, |data_block: &mut DataBlock| {
                    for byte in data_block.iter_mut() {
                        *byte = 0;
                    }
                });
        }
        // initialize SuperBlock
        cache_block(0, Arc::clone(&block_dev))
            .lock()
            .modify(0, |super_block: &mut SuperBlock| {
                super_block.init(
                    total_blocks,
                    imap_blocks,
                    izone_blocks,
                    dmap_blocks,
                    dzone_blocks,
                );
            });
        // write back immediately
        // create a inode for root node "/"
        assert_eq!(sfs.alloc_inode(), 0);
        let (root_inode_block_id, root_inode_offset) = sfs.disk_inode_pos(0);
        cache_block(root_inode_block_id as usize, Arc::clone(&block_dev))
            .lock()
            .modify(root_inode_offset, |disk_inode: &mut DiskInode| {
                disk_inode.init(DiskInodeType::Directory);
            });
        Arc::new(Mutex::new(sfs))
    }

    /// open the snail fs on block devices that already has fs image
    pub fn open(block_dev: Arc<dyn BlockDev>) -> Arc<Mutex<Self>> {
        // read SuperBlock
        cache_block(0, Arc::clone(&block_dev))
            .lock()
            .read(0, |super_block: &SuperBlock| {
                assert!(super_block.is_valid(), "[Error] failed loading snail fs");
                let itotal_blocks = super_block.imap_blocks + super_block.izone_blocks;
                let sfs = Self {
                    block_dev,
                    imap: Bitmap::new(1, super_block.imap_blocks as usize),
                    dmap: Bitmap::new(
                        (1 + itotal_blocks) as usize,
                        super_block.dmap_blocks as usize,
                    ),
                    izone_start: 1 + super_block.imap_blocks,
                    dzone_start: 1 + itotal_blocks + super_block.dmap_blocks,
                };
                Arc::new(Mutex::new(sfs))
            })
    }

    pub fn root_inode(sfs: &Arc<Mutex<Self>>) -> Inode {
        let block_dev = Arc::clone(&sfs.lock().block_dev);
        // acquire sfs lock temporarily
        let (block_id, block_offset) = sfs.lock().disk_inode_pos(0);
        // release sfs lock
        Inode::new(block_id, block_offset, Arc::clone(sfs), block_dev)
    }

    pub fn disk_inode_pos(&self, inode_id: u32) -> (u32, usize) {
        let inode_size = core::mem::size_of::<DiskInode>();
        let inodes_per_block = (BLOCK_SZ / inode_size) as u32;
        let block_id = self.izone_start + inode_id / inodes_per_block;
        (
            block_id,
            (inode_id % inodes_per_block) as usize * inode_size,
        )
    }

    pub fn data_block_id(&self, data_block_id: u32) -> u32 {
        self.dzone_start + data_block_id
    }

    pub fn alloc_inode(&mut self) -> u32 {
        self.imap.alloc(&self.block_dev).unwrap() as u32
    }

    /// Return a block ID not ID in the data area.
    pub fn alloc_data(&mut self) -> u32 {
        self.dmap.alloc(&self.block_dev).unwrap() as u32 + self.dzone_start
    }

    pub fn dealloc_data(&mut self, block_id: u32) {
        cache_block(block_id as usize, Arc::clone(&self.block_dev))
            .lock()
            .modify(0, |data_block: &mut DataBlock| {
                data_block.iter_mut().for_each(|p| {
                    *p = 0;
                })
            });
        self.dmap
            .dealloc(&self.block_dev, (block_id - self.dzone_start) as usize)
    }
}
