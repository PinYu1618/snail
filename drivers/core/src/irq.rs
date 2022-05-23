//! Ref: zcore/drivers/scheme/irq.rs

pub enum IrqError {
    //
}

pub type IrqResult<T> = Result<T, IrqError>;

pub type IrqHandler = Box<dyn Fn() + Send + Sync>;

pub trait Irq {
    fn is_valid(&self, nr: usize) -> bool;

    fn register_handler(&self, nr: usize, handler: IrqHandler) -> IrqResult<()>;

    fn register_device(&self, nr: usize) -> IrqResult<()>;

    fn unregister_handler(&self, nr: usize) -> IrqResult<()>;
}

#[derive(Debug)]
pub enum TriggerMode {
    Edge,
    Level,
}

#[derive(Debug)]
pub enum Polarity {
    ActiveHigh,
    ActiveLow,
}