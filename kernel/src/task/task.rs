use core::cell::RefMut;

use alloc::{vec::Vec, sync::{Arc, Weak}};

use crate::{mm::{memset::MemorySet, addr::PhysPageNr}, fs::File, sync::UPSafeCell, trap::context::TrapContext};

use super::context::TaskContext;

#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    Ready,
    Running,
    Zombie,
}

#[derive(Clone)]
pub struct TaskCtrlBlock {
    // immutable
    //pid: PidHandle,
    // mutable
    inner: UPSafeCell<TaskCtrlBlockInner>,
}

#[derive(Clone)]
pub struct TaskCtrlBlockInner {
    pub trap_cx_ppn: PhysPageNr,
    pub task_cx: TaskContext,
    pub task_status: TaskStatus,
    pub exit_code: i32,
    pub parent: Option<Weak<TaskCtrlBlock>>,
    pub children: Vec<Arc<TaskCtrlBlock>>,
    pub memset: MemorySet,
    pub fd_table: Vec<Option<Arc<dyn File + Send + Sync>>>,
}

impl TaskCtrlBlock {
    pub fn inner_exclusive_access(&self) -> RefMut<'_, TaskCtrlBlockInner> {
        self.inner.exclusive_access()
    }

    pub fn new(elf_data: &[u8]) -> Self {
        unimplemented!()
    }

    pub fn pid(&self) -> usize {
        unimplemented!()
    }

    pub fn fork(self: &Arc<TaskCtrlBlock>) -> Arc<TaskCtrlBlock> {
        unimplemented!()
    }

    pub fn exec(&self, elf_data: &[u8]) {
        unimplemented!()
    }
}

impl TaskCtrlBlockInner {
    pub fn trap_cx(&self) -> &'static mut TrapContext {
        self.trap_cx_ppn.get_mut()
    }

    pub fn user_token(&self) -> usize {
        self.memset.token()
    }

    pub fn is_zombie(&self) -> bool {
        self.status() == TaskStatus::Zombie
    }

    fn status(&self) -> TaskStatus { self.task_status }
}