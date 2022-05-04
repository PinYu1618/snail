use alloc::{collections::VecDeque, sync::Arc};

use crate::{sync::UPSafeCell, task_::TaskCtrlBlock};

pub trait Mutex: Sync + Send {
    fn lock(&self);
    fn unlock(&self);
}

pub struct MutexSpin {
    pub locked: UPSafeCell<bool>,
}

pub struct MutexBlocking {
    pub inner: UPSafeCell<MutexBlockingInner>,
}

pub struct MutexBlockingInner {
    pub locked: bool,
    pub wait_queue: VecDeque<Arc<TaskCtrlBlock>>,
}
