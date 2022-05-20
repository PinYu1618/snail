use core::time::Duration;

use alloc::boxed::Box;

pub trait Timer: Send {
    fn now(&self) -> Duration;

    fn set(&self, deadline: Duration, callback: Box<dyn FnOnce(Duration) + Send + Sync>) {
        todo!()
    }
}

pub fn now() -> Duration {
    todo!()
}