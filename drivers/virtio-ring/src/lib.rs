mod avail;
mod desc;
mod used;
mod queue;

#[macro_use]
extern crate bitflags;

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

pub use queue::Queue;
