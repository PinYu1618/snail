use alloc::vec;
use alloc::{
    sync::{Arc, Weak},
    vec::Vec,
};

use core::cell::RefMut;

use crate::{
    config::TRAP_CONTEXT_BASE,
    fs::stdio::{Stdin, Stdout},
    mm::{
        addr::{PhysPageNr, VirtAddr},
        memset::{MemorySet, KSPACE},
    },
    sync::up::UPSafeCell,
    task::pid::alloc_pid,
    trap::trap_handler,
};
use crate::{fs::File, trap::context::TrapContext};

use super::{
    context::ProcessContext,
    pid::{KStack, PidHandle},
};

#[derive(Copy, Clone, PartialEq)]
pub enum ProcessStatus {
    Ready,
    Running,
    Zombie,
}

#[derive(Clone)]
pub struct ProcessCtrlBlock {
    // immutable
    pub pid: PidHandle,
    pub kstack: KStack,
    // mutable
    inner: UPSafeCell<PcbInner>,
}

#[derive(Clone)]
pub struct PcbInner {
    pub trap_cx_ppn: PhysPageNr,
    pub base_size: usize,
    pub process_cx: ProcessContext,
    pub status: ProcessStatus,
    pub exit_code: i32,
    pub parent: Option<Weak<ProcessCtrlBlock>>,
    pub children: Vec<Arc<ProcessCtrlBlock>>,
    pub memset: MemorySet,
    pub fd_table: Vec<Option<Arc<dyn File + Send + Sync>>>,
}

impl ProcessCtrlBlock {
    pub fn inner_exclusive_access(&self) -> RefMut<'_, PcbInner> {
        self.inner.exclusive_access()
    }

    pub fn new(elf_data: &[u8]) -> Self {
        let (memset, usp, entry) = MemorySet::from_elf(elf_data);
        // map trap context ppn
        let trap_cx_ppn = memset
            .translate(VirtAddr::from(TRAP_CONTEXT_BASE).into())
            .unwrap()
            .ppn();
        // alloc a pid
        let pid_handle = alloc_pid();
        // alloc a kstack in kspace
        let kstack = KStack::new(&pid_handle);
        let ksp = kstack.top();

        let pcb = Self {
            pid: pid_handle,
            kstack,
            inner: unsafe {
                UPSafeCell::new(PcbInner {
                    trap_cx_ppn,
                    base_size: usp,
                    process_cx: ProcessContext::goto_trap_return(ksp),
                    status: ProcessStatus::Ready,
                    memset,
                    parent: None,
                    children: Vec::new(),
                    exit_code: 0,
                    fd_table: vec![
                        // 0 -> stdin
                        Some(Arc::new(Stdin)),
                        // 1 -> stdout
                        Some(Arc::new(Stdout)),
                        // 2 -> stderr
                        Some(Arc::new(Stdout)),
                    ],
                })
            },
        };

        // prepare trap context in user space
        let trap_cx = pcb.inner_exclusive_access().trap_cx();
        *trap_cx = TrapContext::init_app_cx(
            entry,
            usp,
            KSPACE.exclusive_access().token(),
            ksp,
            trap_handler as usize,
        );

        pcb
    }

    pub fn pid(&self) -> usize {
        self.pid.0
    }

    pub fn fork(self: &Arc<ProcessCtrlBlock>) -> Arc<ProcessCtrlBlock> {
        unimplemented!()
    }

    pub fn exec(&self, elf_data: &[u8]) {
        unimplemented!()
    }
}

impl PcbInner {
    pub fn trap_cx(&self) -> &'static mut TrapContext {
        self.trap_cx_ppn.get_mut()
    }

    pub fn user_token(&self) -> usize {
        self.memset.token()
    }

    pub fn is_zombie(&self) -> bool {
        self.status() == ProcessStatus::Zombie
    }

    fn status(&self) -> ProcessStatus {
        self.status
    }
}
