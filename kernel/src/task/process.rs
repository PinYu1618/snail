use crate::{sync::{UPSafeCell, Condvar, Semaphore, Mutex}, mm::MemorySet, fs::FileDescriptorTable, task::{ThreadCtrlBlock, PidHandle, RecycleAllocator}};
use alloc::{vec::Vec, sync::{Arc, Weak}, string::String};
use core::cell::RefMut;

pub struct ProcessCtrlBlock {
    // immutable
    pub pid_handle: PidHandle,
    // mutable
    inner: UPSafeCell<ProcessCtrlBlockInner>,
}

pub struct ProcessCtrlBlockInner {
    pub is_zombie: bool,
    pub memory_set: MemorySet,
    pub exit_code: i32,
    pub parent: Option<Weak<ProcessCtrlBlock>>,
    pub children: Vec<Arc<ProcessCtrlBlock>>,
    pub fd_table: FileDescriptorTable,
    pub threads: Vec<Option<Arc<ThreadCtrlBlock>>>,
    thread_res_allocator: RecycleAllocator,
    pub mutex_list: Vec<Option<Arc<dyn Mutex>>>,
    pub semaphore_list: Vec<Option<Arc<Semaphore>>>,
    pub condvar_list: Vec<Option<Arc<Condvar>>>,
}

impl ProcessCtrlBlock {
    pub fn inner_exclusive_access(&self) -> RefMut<'_, ProcessCtrlBlockInner> {
        self.inner.exclusive_access()
    }

    pub fn pid(&self) -> usize {
        self.pid_handle.pid()
    }

    pub fn new(elf_data: &[u8]) -> Arc<Self> {
        use crate::task;
        use crate::trap;
        use crate::mm;
        let (memory_set, ustack_base, entry) = MemorySet::from_elf(elf_data);
        let pid_handle = task::alloc_pid();
        let process = Arc::new(
            Self {
                pid_handle,
                inner: unsafe { UPSafeCell::new(ProcessCtrlBlockInner::new(memory_set)) }
            }
        );
        // create a main thread
        let thread = Arc::new(ThreadCtrlBlock::new(Arc::clone(&process), ustack_base, true));
        // prepare trap context of main thread
        let thread_inner = thread.inner_exclusive_access();
        let trap_cx = thread_inner.get_trap_cx_mut();
        let ustack_top = thread_inner.res.as_ref().unwrap().ustack_top();
        drop(thread_inner);
        *trap_cx = trap::TrapContext::init_app_cx(
            entry,
            ustack_top,
            mm::ktoken(),
            thread.get_kstack_top(),
            trap::trap_handler as usize
        );
        // add main thread to the process
        let mut process_inner = process.inner_exclusive_access();
        process_inner.threads.push(Some(Arc::clone(&thread)));
        drop(process_inner);
        todo!()
    }

    pub fn exec(self: Arc<Self>, _elf_data: &[u8], _args: Vec<String>) {
        todo!()
    }

    pub fn fork(self: &Arc<Self>) -> Arc<Self> {
        todo!()
    }
}

impl ProcessCtrlBlockInner {
    pub fn is_zombie(&self) -> bool {
        self.is_zombie
    }

    pub fn get_user_token(&self) -> usize {
        self.memory_set.token()
    }

    pub fn threads_count(&self) -> usize {
        self.threads.len()
    }

    pub fn get_thread(&self, tid: usize) -> Arc<ThreadCtrlBlock> {
        self.threads[tid].as_ref().unwrap().clone()
    }

    pub fn alloc_fd(&mut self) -> usize {
        if let Some(fd) = (0..self.fd_table.len()).find(|fd| self.fd_table.entries[*fd].is_none()) {
            fd
        } else {
            self.fd_table.push(None);
            self.fd_table.len() - 1
        }
    }

    pub fn alloc_tid(&mut self) -> usize {
        self.thread_res_allocator.alloc()
    }

    pub fn dealloc_tid(&mut self, tid: usize) {
        self.thread_res_allocator.dealloc(tid)
    }

    pub fn new(memory_set: MemorySet) -> Self {
        Self {
            is_zombie: false,
            memory_set,
            parent: None,
            children: Vec::new(),
            exit_code: 0,
            fd_table: FileDescriptorTable::default(),
            threads: Vec::new(),
            thread_res_allocator: RecycleAllocator::default(),
            mutex_list: Vec::new(),
            semaphore_list: Vec::new(),
            condvar_list: Vec::new(),
        }
    }
}
