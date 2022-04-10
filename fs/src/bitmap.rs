use alloc::sync::Arc;

use super::{cache_block, BlockDev, BLOCK_SZ};

const BLOCK_BITS: usize = BLOCK_SZ * 8;

pub type BitmapBlock = [u64; 64];

pub struct Bitmap {
    start_block_id: usize,
    blocks: usize,
}

impl Bitmap {
    pub fn new(start_block_id: usize, blocks: usize) -> Self {
        Self {
            start_block_id,
            blocks,
        }
    }

    pub fn alloc(&self, block_dev: &Arc<dyn BlockDev>) -> Option<usize> {
        for block_id in 0..self.blocks {
            let pos = cache_block(
                block_id + self.start_block_id as usize,
                Arc::clone(block_dev),
            )
            .lock()
            .modify(0, |bitmap_block: &mut BitmapBlock| {
                if let Some((bits64_pos, inner_pos)) = bitmap_block
                    .iter()
                    .enumerate()
                    .find(|(_, bits64)| **bits64 != u64::MAX)
                    .map(|(bits64_pos, bits64)| (bits64_pos, bits64.trailing_ones() as usize))
                {
                    // modify cache
                    bitmap_block[bits64_pos] |= 1u64 << inner_pos;
                    Some(block_id * BLOCK_BITS + bits64_pos * 64 + inner_pos as usize)
                } else {
                    None
                }
            });
            if pos.is_some() {
                return pos;
            }
        }
        None
    }

    pub fn dealloc(&self, block_dev: &Arc<dyn BlockDev>, bit: usize) {
        let (block_pos, bits64_pos, inner_pos) = decomposition(bit);
        cache_block(block_pos + self.start_block_id, Arc::clone(block_dev))
            .lock()
            .modify(0, |bitmap_block: &mut BitmapBlock| {
                assert!(bitmap_block[bits64_pos] & (1u64 << inner_pos) > 0);
                bitmap_block[bits64_pos] -= 1u64 << inner_pos;
            });
    }

    pub fn max(&self) -> usize {
        self.blocks * BLOCK_BITS
    }
}

/// Return (block_pos, bits64_pos, inner_pos)
fn decomposition(mut bit: usize) -> (usize, usize, usize) {
    let block_pos = bit / BLOCK_BITS;
    bit = bit % BLOCK_BITS;
    (block_pos, bit / 64, bit % 64)
}
