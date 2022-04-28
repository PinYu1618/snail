use alloc::sync::{Arc, Weak};
use spin::Mutex;

pub const PIPE_RING_BUFFER_SIZE: usize = 32;

/// Pipe end, either readable or writable.
pub struct Pipe {
    pub readable: bool,
    pub writable: bool,
    pub buffer: Arc<Mutex<PipeRingBuffer>>,
}

impl Pipe {
    /// Create new read end from an exist `PipeRingBuffer`.
    pub fn new_read_end(buf: Arc<Mutex<PipeRingBuffer>>) -> Self {
        Self {
            readable: true,
            writable: false,
            buffer: buf,
        }
    }

    /// Create new write end from an exist `PipeRingBuffer`.
    pub fn new_write_end(buf: Arc<Mutex<PipeRingBuffer>>) -> Self {
        Self {
            readable: false,
            writable: false,
            buffer: buf,
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum PipeRingBufferStatus {
    Empty,
    Full,
    Normal,
}

pub struct PipeRingBuffer {
    pub buffer: [u8; PIPE_RING_BUFFER_SIZE],
    pub head: usize,
    pub tail: usize,
    pub status: PipeRingBufferStatus,
    pub write_end: Option<Weak<Pipe>>,
}

impl PipeRingBuffer {
    pub fn new() -> Self {
        Self {
            buffer: [0; PIPE_RING_BUFFER_SIZE],
            head: 0,
            tail: 0,
            status: PipeRingBufferStatus::Empty,
            write_end: None,
        }
    }

    pub fn set_write_end(&mut self, write_end: &Arc<Pipe>) {
        self.write_end = Some(Arc::downgrade(write_end));
    }
}

impl Default for PipeRingBuffer {
    fn default() -> Self {
        Self::new()
    }
}

pub fn make_pipe() -> (Arc<Pipe>, Arc<Pipe>) {
    let buf = Arc::new(Mutex::new(PipeRingBuffer::default()));
    let read_end = Arc::new(Pipe::new_read_end(buf.clone()));
    let write_end = Arc::new(Pipe::new_write_end(buf.clone()));
    buf.lock().set_write_end(&write_end);
    (read_end, write_end)
}
