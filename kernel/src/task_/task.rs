use alloc::string::String;
use alloc::vec;
use alloc::{
    sync::{Arc, Weak},
    vec::Vec,
};

use crate::{
    config::TRAP_CONTEXT_BASE,
    fs::{Stdin, Stdout},
    mm::{memset::KSPACE, MemorySet, PageTable, PhysPageNr, VirtAddr},
    sync::UPSafeCell,
    task_::PidAllocator,
    trap::trap_handler,
};
use crate::{fs::File, trap::TrapContext};

use super::{
    context::TaskContext,
    pid::{KStack, PidHandle},
};

use core::cell::RefMut;

#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    Ready,
    Running,
    Zombie,
}

#[derive(Clone)]
pub struct TaskCtrlBlock {
    // immutable
    pub pid: PidHandle,
    pub kstack: KStack,
    // mutable
    inner: UPSafeCell<TaskInner>,
}

#[derive(Clone)]
pub struct TaskInner {
    pub trap_cx_ppn: PhysPageNr,
    pub base_size: usize,
    pub task_cx: TaskContext,
    pub task_status: TaskStatus,
    pub exit_code: i32,
    pub parent: Option<Weak<TaskCtrlBlock>>,
    pub children: Vec<Arc<TaskCtrlBlock>>,
    pub memset: MemorySet,
    pub fd_table: Vec<Option<Arc<dyn File + Send + Sync>>>,
}

impl TaskCtrlBlock {
    pub fn inner_exclusive_access(&self) -> RefMut<'_, TaskInner> {
        self.inner.exclusive_access()
    }

    pub fn new(elf_data: &[u8]) -> Self {
        let (memset, usp, entry) = MemorySet::from_elf(elf_data);
        // map trap context ppn
        let trap_cx_ppn = memset
            .translate(VirtAddr::from(TRAP_CONTEXT_BASE).into())
            .unwrap()
            .ppn();
        trace!("New process trap context ppn: {:?}", trap_cx_ppn);

        // alloc a pid
        let pid_handle = PidAllocator::alloc_pid();
        // alloc a kstack in kspace
        let kstack = KStack::new(&pid_handle);
        let ksp = kstack.top();

        let pcb = Self {
            pid: pid_handle,
            kstack,
            inner: unsafe {
                UPSafeCell::new(TaskInner {
                    trap_cx_ppn,
                    base_size: usp,
                    task_cx: TaskContext::goto_trap_return(ksp),
                    task_status: TaskStatus::Ready,
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

    pub fn fork(self: &Arc<TaskCtrlBlock>) -> Arc<TaskCtrlBlock> {
        // ---- hold parent PCB lock
        let mut parent_inner = self.inner_exclusive_access();
        // copy user space(include trap context)
        let memset = MemorySet::from_existed_user(&parent_inner.memset);
        let trap_cx_ppn = memset
            .translate(VirtAddr::from(TRAP_CONTEXT_BASE).into())
            .unwrap()
            .ppn();
        // alloc a pid and a kernel stack in kernel space
        let pid_handle = PidAllocator::alloc_pid();
        let kernel_stack = KStack::new(&pid_handle);
        let kernel_stack_top = kernel_stack.top();
        // copy fd table
        let mut new_fd_table: Vec<Option<Arc<dyn File + Send + Sync>>> = Vec::new();
        for fd in parent_inner.fd_table.iter() {
            if let Some(file) = fd {
                new_fd_table.push(Some(file.clone()));
            } else {
                new_fd_table.push(None);
            }
        }
        let task_control_block = Arc::new(TaskCtrlBlock {
            pid: pid_handle,
            kstack: kernel_stack,
            inner: unsafe {
                UPSafeCell::new(TaskInner {
                    trap_cx_ppn,
                    base_size: parent_inner.base_size,
                    task_cx: TaskContext::goto_trap_return(kernel_stack_top),
                    task_status: TaskStatus::Ready,
                    memset,
                    parent: Some(Arc::downgrade(self)),
                    children: Vec::new(),
                    exit_code: 0,
                    fd_table: new_fd_table,
                })
            },
        });
        // add child
        parent_inner.children.push(task_control_block.clone());
        // modify kernel_sp in trap_cx
        // **** access child PCB exclusively
        let trap_cx = task_control_block.inner_exclusive_access().trap_cx();
        trap_cx.kernel_stack_top = kernel_stack_top;
        // return
        task_control_block
        // **** release child PCB
        // ---- release parent PCB
    }

    pub fn exec(&self, elf_data: &[u8], args: Vec<String>) {
        // memory_set with elf program headers/trampoline/trap context/user stack
        let (memset, mut user_sp, entry) = MemorySet::from_elf(elf_data);
        let trap_cx_ppn = memset
            .translate(VirtAddr::from(TRAP_CONTEXT_BASE).into())
            .unwrap()
            .ppn();
        // push arguments on user stack
        user_sp -= (args.len() + 1) * core::mem::size_of::<usize>();
        let argv_base = user_sp;
        let mut argv: Vec<_> = (0..args.len() + 1)
            .map(|arg| {
                PageTable::translated_refmut(
                    memset.token(),
                    (argv_base + arg * core::mem::size_of::<usize>()) as *mut usize,
                )
            })
            .collect();
        *argv[args.len()] = 0;
        for i in 0..args.len() {
            user_sp -= args[i].len() + 1;
            *argv[i] = user_sp;
            let mut p = user_sp;
            for c in args[i].as_bytes() {
                *PageTable::translated_refmut(memset.token(), p as *mut u8) = *c;
                p += 1;
            }
            *PageTable::translated_refmut(memset.token(), p as *mut u8) = 0;
        }
        // make the user_sp aligned to 8B for k210 platform
        user_sp -= user_sp % core::mem::size_of::<usize>();

        // **** access current TCB exclusively
        let mut inner = self.inner_exclusive_access();
        // substitute memory_set
        inner.memset = memset;
        // update trap_cx ppn
        inner.trap_cx_ppn = trap_cx_ppn;
        // initialize trap_cx
        let mut trap_cx = TrapContext::init_app_cx(
            entry,
            user_sp,
            KSPACE.exclusive_access().token(),
            self.kstack.top(),
            trap_handler as usize,
        );
        trap_cx.x[10] = args.len();
        trap_cx.x[11] = argv_base;
        *inner.trap_cx() = trap_cx;
        // **** release current PCB
    }
}

impl TaskInner {
    pub fn trap_cx(&self) -> &'static mut TrapContext {
        self.trap_cx_ppn.get_mut()
    }

    pub fn user_token(&self) -> usize {
        self.memset.token()
    }

    pub fn is_zombie(&self) -> bool {
        self.status() == TaskStatus::Zombie
    }

    pub fn alloc_fd(&mut self) -> usize {
        if let Some(fd) = (0..self.fd_table.len()).find(|fd| self.fd_table[*fd].is_none()) {
            fd
        } else {
            self.fd_table.push(None);
            self.fd_table.len() - 1
        }
    }

    fn status(&self) -> TaskStatus {
        self.task_status
    }
}
