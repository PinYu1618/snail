//! Ref: linux/include/uapi/linux/virtio_ring.h

use enumflags2::bitflags;

pub mod avail;
pub mod desc;
pub mod dma;
pub mod packed;
pub mod used;
mod queue;

pub type Result<T = ()> = core::result::Result<T, Error>;

#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    /// The buffer is too small.
    BufferTooSmall,
    /// The device is not ready.
    NotReady,
    /// The queue is already in use.
    AlreadyUsed,
    /// Invalid parameter.
    InvalidParam,
    /// Failed to alloc DMA memory.
    DmaError,
    /// I/O Error
    IoError,
}

pub use queue::VirtioQueue;

#[bitflags]
#[repr(u64)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Feature {
    IndirectDesc = 1 << 28,
    EventIdx = 1 << 29,

    Version1 = 1 << 32,
    AccessPlatform = 1 << 33,
    RingPacked = 1 << 34,
    InOrder = 1 << 35,
    OrderPlatform = 1 << 36,
    SrIov = 1 << 37,
}