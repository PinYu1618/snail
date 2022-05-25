//! Ref: linux/drivers/virtio/virtio_ring.c

use crate::{desc::Desc, avail::Avail, used::Used};

#[repr(C)]
pub struct VirtioQueue<'a> {
    /// DMA guard
    //dma: DMA,
    pub desc: &'a mut [Desc],
    pub avail: &'a mut Avail,
    pub used: &'a mut Used,

    pub avail_idx: u16,
    
    queue_idx: u32,
    queue_size: u16,
    num_used: u16,
    pub free_head: u16,
    pub last_used_idx: u16,
}
