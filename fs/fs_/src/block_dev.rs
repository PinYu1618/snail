use core::any::Any;

pub trait BlockDevice: Send + Sync + Any {
    fn read_block(&self, id: usize, buf: &mut [u8]);
    fn write_block(&self, id: usize, buf: &[u8]);
}
