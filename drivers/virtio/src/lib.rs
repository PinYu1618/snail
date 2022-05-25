//! Ref: linux/include/uapi/linux/virtio_config.h
//! Ref: linux/include/uapi/linux/virtio_ids.h

#![no_std]

use enumflags2::bitflags;

pub mod mmio;

/// The error type of VirtIO drivers.
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

pub type Result<T> = core::result::Result<T, Error>;

#[repr(u8)]
pub enum VirtioId {
    Net = 1,
    Block = 2,
    Console = 3,
    IoMemory = 6,
    Gpu = 16,
    Input = 18,
    Sound = 25,
    Can = 36,
}

impl VirtioId {
    pub fn as_u8(self) -> u8 {
        self as u8
    }

    pub fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}

#[repr(u16)]
pub enum TransitionalId {
    Net = 0x1000,
    Block = 0x1001,
    Balloon = 0x1002,
    Console = 0x1003,
    Scsi = 0x1004,
    Rng = 0x1005,
    _9P = 0x1009,
}

impl TransitionalId {
    pub fn as_u8(self) -> u8 {
        self as u8
    }

    pub fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}

#[bitflags]
#[repr(u64)]
#[derive(Clone, Copy, PartialEq)]
pub enum TransportFeature {
    Start = 1 << 28,
    End = 1 << 38,
}

#[bitflags]
#[repr(u64)]
#[derive(Clone, Copy, PartialEq)]
pub enum Feature {
    Version1 = 1 << 32,
    AccessPlatform = 1 << 33,
    RingPacked = 1 << 34,
    InOrder = 1 << 35,
    OrderPlatform = 1 << 36,
    SrIov = 1 << 37,
}

pub const VIRTIO_RING_F_INDIRECT_DESC: u8 = 28;
pub const VIRTIO_RING_F_EVENT_IDX: u8 = 29;