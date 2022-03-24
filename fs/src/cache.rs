use lazy_static::*;
use spin::Mutex;
use alloc::sync::Arc;
use alloc::collections::VecDeque;
use super::BLOCK_SZ;
use super::BlockDevice;

const BLOCK_CACHE_LIMIT: usize = 16;

pub struct BlockCache {
    content: [u8; BLOCK_SZ],
    block_id: usize,
    block_dev: Arc<dyn BlockDevice>,
    modified: bool,
}

impl BlockCache {
    // Load a new BlockCache from disk.
    pub fn new(id: usize, dev: Arc<dyn BlockDevice>) -> Self {
        let mut cache = [0_u8; BLOCK_SZ];
        dev.read_block(id, &mut cache);
        Self {
            content: cache,
            block_id: id,
            block_dev: dev,
            modified: false,
        }
    }

    pub fn read<T, V>(&self, offset: usize, f: impl FnOnce(&T) -> V) -> V {
        f(self.get_ref(offset))
    }

    pub fn modify<T, V>(&mut self, offset:usize, f: impl FnOnce(&mut T) -> V) -> V {
        f(self.get_mut(offset))
    }
    
    fn get_ref<T>(&self, offset: usize) -> &T where T: Sized {
        let type_size = core::mem::size_of::<T>();
        assert!(offset + type_size <= BLOCK_SZ);
        let addr = self.addr_of_offset(offset);
        unsafe { &*(addr as *const T) }
    }

    fn get_mut<T>(&mut self, offset: usize) -> &mut T where T: Sized {
        let type_size = core::mem::size_of::<T>();
        assert!(offset + type_size <= BLOCK_SZ);
        self.modified = true;
        let addr = self.addr_of_offset(offset);
        unsafe { &mut *(addr as *mut T) }
    }

    fn sync(&mut self) {
        if self.modified {
            self.modified = false;
            self.block_dev.write_block(self.block_id, &self.content);
        }
    }
    
    fn addr_of_offset(&self, offset: usize) -> usize {
        &self.content[offset] as *const _ as usize
    }
}

impl Drop for BlockCache {
    fn drop(&mut self) {
        self.sync()
    }
}

pub struct BlockCacher {
    queue: VecDeque<(usize, Arc<Mutex<BlockCache>>)>,
}

impl BlockCacher {
    pub fn new() -> Self {
        Self { queue: VecDeque::new() }
    }

    fn cache_block(&mut self, id: usize, dev: Arc<dyn BlockDevice>) -> Arc<Mutex<BlockCache>> {
        if let Some(pair) = self.queue.iter()
        .find(|pair| pair.0 == id) {
            Arc::clone(&pair.1)
        } else {
            // substitute
            if self.queue.len() == BLOCK_CACHE_LIMIT {
                // from front to tail
                if let Some((idx, _)) = self.queue.iter().enumerate()
                .find(|(_, pair)| Arc::strong_count(&pair.1) == 1) {
                    self.queue.drain(idx..=idx);
                } else {
                    panic!("Run out of BlockCache!");
                }
            }
            // load block into mem and push back
            let block_cache = Arc::new(Mutex::new(
                BlockCache::new(id, Arc::clone(&dev))
            ));
            self.queue.push_back((id, Arc::clone(&block_cache)));
            block_cache
        }
    }
}

lazy_static! {
    pub static ref BLOCK_CACHER: Mutex<BlockCacher> = Mutex::new(
        BlockCacher::new()
    );
}

pub fn cache_block(id: usize, dev: Arc<dyn BlockDevice>) -> Arc<Mutex<BlockCache>> {
    BLOCK_CACHER.lock().cache_block(id, dev)
}