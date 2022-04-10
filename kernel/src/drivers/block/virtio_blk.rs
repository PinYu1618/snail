use alloc::vec::Vec;
use lazy_static::lazy_static;
use snail_fs::BlockDev;
use spin::Mutex;
use virtio_drivers::{VirtIOBlk, VirtIOHeader};

use crate::mm::{
    addr::{PhysAddr, PhysPageNr, Step, VirtAddr},
    frame::{alloc_frame, dealloc_frame, FrameTracker},
    memset::ktoken,
    page::PageTable,
};

const VIRTIO0: usize = 0x10001000;

pub struct VirtIOBlock(Mutex<VirtIOBlk<'static>>);

impl VirtIOBlock {
    pub fn new() -> Self {
        Self(Mutex::new(
            VirtIOBlk::new(unsafe { &mut *(VIRTIO0 as *mut VirtIOHeader) }).unwrap(),
        ))
    }
}

impl BlockDev for VirtIOBlock {
    fn read_block(&self, block_id: usize, buf: &mut [u8]) {
        self.0
            .lock()
            .read_block(block_id, buf)
            .expect("Error when reading VirtIOBlk");
    }

    fn write_block(&self, block_id: usize, buf: &[u8]) {
        self.0
            .lock()
            .write_block(block_id, buf)
            .expect("Error when writing VirtIOBlk");
    }
}

lazy_static! {
    static ref QUEUE_FRAMES: Mutex<Vec<FrameTracker>> = Mutex::new(Vec::new());
}

#[no_mangle]
pub extern "C" fn virtio_dma_alloc(pages: usize) -> PhysAddr {
    let mut ppn_base = PhysPageNr::from(0);
    for i in 0..pages {
        let frame = alloc_frame().unwrap();
        if i == 0 {
            ppn_base = frame.ppn();
        }
        assert_eq!(frame.ppn().as_usize(), ppn_base.as_usize() + i);
        QUEUE_FRAMES.lock().push(frame);
    }
    ppn_base.into()
}

#[no_mangle]
pub extern "C" fn virtio_dma_dealloc(pa: PhysAddr, pages: usize) -> i32 {
    let mut ppn_base: PhysPageNr = pa.into();
    for _ in 0..pages {
        dealloc_frame(ppn_base);
        ppn_base.step();
    }
    0
}

#[no_mangle]
pub extern "C" fn virtio_phys_to_virt(paddr: PhysAddr) -> VirtAddr {
    VirtAddr::from(paddr.as_usize())
}

#[no_mangle]
pub extern "C" fn virtio_virt_to_phys(vaddr: VirtAddr) -> PhysAddr {
    PageTable::from_token(ktoken()).translate_va(vaddr).unwrap()
}
