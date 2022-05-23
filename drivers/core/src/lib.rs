pub mod display;
pub mod irq;
pub mod uart;

extern crate alloc;

use alloc::sync::Arc;

pub trait Driver: Send + Sync {
    fn handle_irq(&self, nr: usize);

    fn upgrade<'a>(self: Arc<Self>) -> DriverGuard<'a> where Self: 'a;
}

pub type DriverGuard<'a> = Arc<dyn Driver + 'a>;