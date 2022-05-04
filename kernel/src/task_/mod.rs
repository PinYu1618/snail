pub mod context;
pub mod ctrl;
pub mod pid;
pub mod processor;
pub mod switch;
pub mod task;

pub use ctrl::TaskCtrller;
pub use context::TaskContext;
pub use pid::PidAllocator;
pub use processor::Processor;
pub use task::{TaskCtrlBlock, TaskStatus};

use alloc::sync::Arc;

use crate::fs;

lazy_static! {
    static ref INITPROC: Arc<TaskCtrlBlock> = Arc::new({
        let inode = fs::open_file("initproc", fs::OpenFlags::RDONLY).unwrap();
        let v = inode.read_all();
        TaskCtrlBlock::new(v.as_slice())
    });
}

pub fn add_initproc() {
    TaskCtrller::add_task(INITPROC.clone());
}
