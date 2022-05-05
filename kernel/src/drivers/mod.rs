pub mod block;
mod ns16550a;

pub trait CharDevice {
    fn read(&self) -> u8;
    fn write(&self, ch: u8);
    fn handle_irq(&self);
}
