use alloc::sync::Arc;
use alloc::vec::Vec;

use super::*;

/// snail fs magic number
const SFS_MAGIC: u32 = 123;

const INODE_DIRECT_COUNT: usize = 28;
const DIRECT_BOUND: usize = INODE_DIRECT_COUNT;

const INODE_INDIRECT1_COUNT: usize = BLOCK_SZ / 4;
const INDIRECT1_BOUND: usize = DIRECT_BOUND + INODE_INDIRECT1_COUNT;

const INODE_INDIRECT2_COUNT: usize = INODE_INDIRECT1_COUNT * INODE_INDIRECT1_COUNT;

const NAME_LEN_LIMIT: usize = 27;
pub const DIRENT_SZ: usize = 32;

pub type DataBlock = [u8; BLOCK_SZ];
pub type IndirectBlock = [u32; BLOCK_SZ / 4];

#[repr(C)]
pub struct SuperBlock {
    magic: u32,
    pub total_blocks: u32,
    pub imap_blocks: u32,
    pub izone_blocks: u32,
    pub dmap_blocks: u32,
    pub dzone_blocks: u32,
}

impl SuperBlock {
    pub fn init(
        &mut self,
        total_blocks: u32,
        imap_blocks: u32,
        izone_blocks: u32,
        dmap_blocks: u32,
        dzone_blocks: u32
    ) {
        *self = Self {
            magic: SFS_MAGIC,
            total_blocks,
            imap_blocks,
            izone_blocks,
            dmap_blocks,
            dzone_blocks,
        }
    }
    pub fn is_valid(&self) -> bool {
        self.magic == SFS_MAGIC
    }
}

#[repr(C)]
pub struct DirEntry {
    /// direntry's name + \0
    name: [u8; NAME_LEN_LIMIT + 1],
    /// inode No.
    ino: u32,
}

impl DirEntry {
    pub fn empty() -> Self {
        Self {
            name: [0_u8; NAME_LEN_LIMIT + 1],
            ino: 0,
        }
    }

    pub fn new(name: &str, ino: u32) -> Self {
        let mut bytes = [0_u8; NAME_LEN_LIMIT + 1];
        &mut bytes[..name.len()].copy_from_slice(name.as_bytes());
        Self {
            name: bytes,
            ino,
        }
    }

    pub fn name(&self) -> &str {
        let len = (0_usize..).find(|i| self.name[*i] == 0).unwrap();
        core::str::from_utf8(&self.name[..len]).unwrap()
    }

    pub fn ino(&self) -> u32 {
        self.ino
    }

    pub fn as_bytes(&self) -> &[u8] {
        unsafe {
            core::slice::from_raw_parts(
                self as *const _ as usize as *const u8,
                DIRENT_SZ,
            )
        }
    }

    pub fn as_bytes_mut(&mut self) -> &mut [u8] {
        unsafe {
            core::slice::from_raw_parts_mut(
                self as *mut _ as usize as *mut u8,
                DIRENT_SZ,
            )
        }
    }
}

#[repr(C)]
pub struct DiskInode {
    /// byte size of its content
    pub size: u32,
    /// direct index data blocks
    pub direct: [u32; INODE_DIRECT_COUNT],
    pub indir1: u32,
    pub indir2: u32,
    type_: DiskInodeType,
}

#[derive(PartialEq)]
pub enum DiskInodeType {
    File,
    Directory,
}

impl DiskInode {
    pub fn init(&mut self, type_: DiskInodeType) {
        self.size = 0;
        self.direct.iter_mut().for_each(|v| *v = 0);
        self.indir1 = 0;
        self.indir2 = 0;
        self.type_ = type_;
    }

    pub fn is_dir(&self) -> bool {
        self.type_ == DiskInodeType::Directory
    }

    pub fn is_file(&self) -> bool {
        self.type_ == DiskInodeType::File
    }

    pub fn get_block_id(&self, inner_id: u32, dev: &Arc<dyn BlockDev>) -> u32 {
        let inner_id = inner_id as usize;
        if inner_id < INODE_DIRECT_COUNT {
            self.direct[inner_id]
        } else if inner_id < INDIRECT1_BOUND {
            cache_block(self.indir1 as usize, Arc::clone(dev))
            .lock()
            .read(0, |indirect_block: &IndirectBlock| {
                indirect_block[inner_id - INODE_DIRECT_COUNT]
            })
        } else {
            let last = inner_id - INDIRECT1_BOUND;
            let indirect1 = cache_block(
                self.indir2 as usize,
                Arc::clone(dev)
            )
            .lock()
            .read(0, |indirect2: &IndirectBlock| {
                indirect2[last / INODE_INDIRECT1_COUNT]
            });
            cache_block(
                indirect1 as usize,
                Arc::clone(dev)
            )
            .lock()
            .read(0, |indirect1: &IndirectBlock| {
                indirect1[last % INODE_INDIRECT1_COUNT]
            })
        }
    }

    pub fn read_at(
        &self,
        offset: usize,
        buf: &mut [u8],
        block_dev: &Arc<dyn BlockDev>,
    ) -> usize {
        let mut start = offset;
        let end = (offset + buf.len()).min(self.size as usize);
        if start >= end {
            return 0;
        }
        let mut start_block = start / BLOCK_SZ;
        let mut read_size = 0usize;
        loop {
            // calculate end of current block
            let mut end_current_block = (start / BLOCK_SZ + 1) * BLOCK_SZ;
            end_current_block = end_current_block.min(end);
            // read and update read size
            let block_read_size = end_current_block - start;
            let dst = &mut buf[read_size..read_size + block_read_size];
            cache_block(
                self.get_block_id(start_block as u32, block_dev) as usize,
                Arc::clone(block_dev),
            )
            .lock()
            .read(0, |data_block: &DataBlock| {
                let src = &data_block[start % BLOCK_SZ..start % BLOCK_SZ + block_read_size];
                dst.copy_from_slice(src);
            });
            read_size += block_read_size;
            // move to next block
            if end_current_block == end { break; }
            start_block += 1;
            start = end_current_block;
        }
        read_size
    }

    pub fn write_at(
        &mut self,
        offset: usize,
        buf: &[u8],
        block_device: &Arc<dyn BlockDev>,
    ) -> usize {
        let mut start = offset;
        let end = (offset + buf.len()).min(self.size as usize);
        assert!(start <= end);
        let mut start_block = start / BLOCK_SZ;
        let mut write_size = 0usize;
        loop {
            // calculate end of current block
            let mut end_current_block = (start / BLOCK_SZ + 1) * BLOCK_SZ;
            end_current_block = end_current_block.min(end);
            // write and update write size
            let block_write_size = end_current_block - start;
            cache_block(
                self.get_block_id(start_block as u32, block_device) as usize,
                Arc::clone(block_device),
            )
            .lock()
            .modify(0, |data_block: &mut DataBlock| {
                let src = &buf[write_size..write_size + block_write_size];
                let dst = &mut data_block[start % BLOCK_SZ..start % BLOCK_SZ + block_write_size];
                dst.copy_from_slice(src);
            });
            write_size += block_write_size;
            // move to next block
            if end_current_block == end {
                break;
            }
            start_block += 1;
            start = end_current_block;
        }
        write_size
    }
    
    pub fn increase_size(
        &mut self,
        new_size: u32,
        new_blocks: Vec<u32>,
        block_dev: &Arc<dyn BlockDev>,
    ) {
        let mut current_blocks = self.data_blocks();
        self.size = new_size;
        let mut total_blocks = self.data_blocks();
        let mut new_blocks = new_blocks.into_iter();
        // fill direct
        while current_blocks < total_blocks.min(INODE_DIRECT_COUNT as u32) {
            self.direct[current_blocks as usize] = new_blocks.next().unwrap();
            current_blocks += 1;
        }
        // alloc indirect1
        if total_blocks > INODE_DIRECT_COUNT as u32 {
            if current_blocks == INODE_DIRECT_COUNT as u32 {
                self.indir1 = new_blocks.next().unwrap();
            }
            current_blocks -= INODE_DIRECT_COUNT as u32;
            total_blocks -= INODE_DIRECT_COUNT as u32;
        } else {
            return;
        }
        // fill indirect1
        cache_block(self.indir1 as usize, Arc::clone(block_dev))
        .lock()
        .modify(0, |indirect1: &mut IndirectBlock| {
            while current_blocks < total_blocks.min(INODE_INDIRECT1_COUNT as u32) {
                indirect1[current_blocks as usize] = new_blocks.next().unwrap();
                current_blocks += 1;
            }
        });
        // alloc indirect2
        if total_blocks > INODE_INDIRECT1_COUNT as u32 {
            if current_blocks == INODE_INDIRECT1_COUNT as u32 {
                self.indir2 = new_blocks.next().unwrap();
            }
            current_blocks -= INODE_INDIRECT1_COUNT as u32;
            total_blocks -= INODE_INDIRECT1_COUNT as u32;
        } else {
            return;
        }
        // fill indirect2 from (a0, b0) -> (a1, b1)
        let mut a0 = current_blocks as usize / INODE_INDIRECT1_COUNT;
        let mut b0 = current_blocks as usize % INODE_INDIRECT1_COUNT;
        let a1 = total_blocks as usize / INODE_INDIRECT1_COUNT;
        let b1 = total_blocks as usize % INODE_INDIRECT1_COUNT;
        // alloc low-level indirect1
        cache_block(self.indir2 as usize, Arc::clone(block_dev))
        .lock()
        .modify(0, |indirect2: &mut IndirectBlock| {
            while (a0 < a1) || (a0 == a1 && b0 < b1) {
                if b0 == 0 {
                    indirect2[a0] = new_blocks.next().unwrap();
                }
                // fill current
                cache_block(indirect2[a0] as usize, Arc::clone(block_dev))
                .lock()
                .modify(0, |indirect1: &mut IndirectBlock| {
                    indirect1[b0] = new_blocks.next().unwrap();
                });
                // move to next
                b0 += 1;
                if b0 == INODE_INDIRECT1_COUNT {
                    b0 = 0;
                    a0 += 1;
                }
            }
        });
    }

    pub fn clear_size(&mut self, block_dev: &Arc<dyn BlockDev>) -> Vec<u32> {
        let mut v: Vec<u32> = Vec::new();
        let mut data_blocks = self.data_blocks() as usize;
        self.size = 0;
        let mut current_blocks = 0usize;
        // direct
        while current_blocks < data_blocks.min(INODE_DIRECT_COUNT) {
            v.push(self.direct[current_blocks]);
            self.direct[current_blocks] = 0;
            current_blocks += 1;
        }
        // indirect1 block
        if data_blocks > INODE_DIRECT_COUNT {
            v.push(self.indir1);
            data_blocks -= INODE_DIRECT_COUNT;
            current_blocks = 0;
        } else {
            return v;
        }
        // indirect1
        cache_block(self.indir1 as usize, Arc::clone(block_dev))
        .lock()
        .modify(0, |indirect1: &mut IndirectBlock| {
            while current_blocks < data_blocks.min(INODE_INDIRECT1_COUNT) {
                v.push(indirect1[current_blocks]);
                //indirect1[current_blocks] = 0;
                current_blocks += 1;
            }
        });
        self.indir1 = 0;
        // indirect2 block
        if data_blocks > INODE_INDIRECT1_COUNT {
            v.push(self.indir2);
            data_blocks -= INODE_INDIRECT1_COUNT;
        } else {
            return v;
        }
        // indirect2
        assert!(data_blocks <= INODE_INDIRECT2_COUNT);
        let a1 = data_blocks / INODE_INDIRECT1_COUNT;
        let b1 = data_blocks % INODE_INDIRECT1_COUNT;
        cache_block(self.indir2 as usize, Arc::clone(block_dev))
        .lock()
        .modify(0, |indirect2: &mut IndirectBlock| {
            // full indirect1 blocks
            for entry in indirect2.iter_mut().take(a1) {
                v.push(*entry);
                cache_block(*entry as usize, Arc::clone(block_dev))
                .lock()
                .modify(0, |indirect1: &mut IndirectBlock| {
                    for entry in indirect1.iter() {
                        v.push(*entry);
                    }
                });
            }
            // last indirect1 block
            if b1 > 0 {
                v.push(indirect2[a1]);
                cache_block(indirect2[a1] as usize, Arc::clone(block_dev))
                .lock()
                .modify(0, |indirect1: &mut IndirectBlock| {
                    for entry in indirect1.iter().take(b1) {
                        v.push(*entry);
                    }
                });
                //indirect2[a1] = 0;
            }
        });
        self.indir2 = 0;
        v
    }

    // Return block number correspond to size.
    pub fn data_blocks(&self) -> u32 {
        Self::_data_blocks(self.size)
    }

    // Return number of blocks needed include indir1/2.
    pub fn total_blocks(size: u32) -> u32 {
        let data_blocks = Self::_data_blocks(size) as usize;
        let mut total = data_blocks as usize;
        // indirect1
        if data_blocks > INODE_DIRECT_COUNT {
            total += 1;
        }
        // indirect2
        if data_blocks > INDIRECT1_BOUND {
            total += 1;
            // sub indirect1
            total += (data_blocks - INDIRECT1_BOUND + INODE_INDIRECT1_COUNT - 1) / INODE_INDIRECT1_COUNT;
        }
        total as u32
    }

    pub fn blocks_num_needed(&self, new_size: u32) -> u32 {
        assert!(new_size >= self.size);
        Self::total_blocks(new_size) - Self::total_blocks(self.size)
    }

    fn _data_blocks(size: u32) -> u32 {
        (size + BLOCK_SZ as u32 - 1) / BLOCK_SZ as u32
    }
}