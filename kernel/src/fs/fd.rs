use crate::fs::File;
use alloc::vec;
use alloc::{vec::Vec, sync::Arc};

pub type FileDescriptor = dyn File + Send + Sync;

pub struct FileDescriptorTable {
    pub entries: Vec<Option<Arc<FileDescriptor>>>,
}

impl FileDescriptorTable {
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn push(&mut self, fd: Option<Arc<FileDescriptor>>) {
        self.entries.push(fd)
    }
}

impl Default for FileDescriptorTable {
    fn default() -> Self {
        use crate::fs::{Stdin, Stdout, Stderr};
        Self { entries: vec![Some(Arc::new(Stdin)), Some(Arc::new(Stdout)), Some(Arc::new(Stderr))] }
    }
}
