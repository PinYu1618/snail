use crate::task_::{TaskContext, TaskCtrller, TaskStatus};
use crate::sync::UPSafeCell;
use crate::task_::TaskCtrlBlock;
use crate::trap::TrapContext;
use alloc::sync::Arc;

lazy_static! {
    static ref PROCESSOR: UPSafeCell<Processor> =
        unsafe { UPSafeCell::new(Processor::default()) };
}

pub struct Processor {
    current: Option<Arc<TaskCtrlBlock>>,
    idle_task_cx: TaskContext,
}

impl Processor {
    pub fn new() -> Self {
        Self {
            current: None,
            idle_task_cx: TaskContext::zero_init(),
        }
    }

    pub fn idle_task_cx_ptr(&mut self) -> *mut TaskContext {
        &mut self.idle_task_cx as *mut _
    }

    pub fn take_current(&mut self) -> Option<Arc<TaskCtrlBlock>> {
        self.current.take()
    }

    pub fn current(&self) -> Option<Arc<TaskCtrlBlock>> {
        self.current.as_ref().map(Arc::clone)
    }

    pub fn current_process() -> Option<Arc<TaskCtrlBlock>> {
        PROCESSOR.exclusive_access().current()
    }

    pub fn take_current_process() -> Option<Arc<TaskCtrlBlock>> {
        PROCESSOR.exclusive_access().take_current()
    }

    pub fn current_user_token() -> usize {
        let pcb = Self::current_process().unwrap();
        let token = pcb.inner_exclusive_access().user_token();
        token
    }

    pub fn current_trap_cx() -> &'static mut TrapContext {
        Self::current_process()
            .unwrap()
            .inner_exclusive_access()
            .trap_cx()
    }

    pub fn run_tasks() {
        loop {
            let mut processor = PROCESSOR.exclusive_access();
            if let Some(task) = TaskCtrller::fetch_task() {
                let _idle_task_cx_ptr = processor.idle_task_cx_ptr();
    
                // access coming task pcb exclusively
                let mut _pcb_inner = task.inner_exclusive_access();
            }
            unimplemented!()
        }
    }

    pub fn schedule(_process_cx_ptr: *mut TaskContext) {
        unimplemented!()
    }

    pub fn suspend_current_and_run_next() {
        if let Some(pcb) = Self::take_current_process() {
            // ---- access current pcb exclusively
            let mut pcb_inner = pcb.inner_exclusive_access();
            let process_cx_ptr = &mut pcb_inner.task_cx as *mut TaskContext;
            // change status to ready
            pcb_inner.task_status = TaskStatus::Ready;
            drop(pcb_inner);
            // ---- release current pcb
    
            // push back to ready queue
            TaskCtrller::add_task(pcb);
            // jump to scheduling cycle
            Processor::schedule(process_cx_ptr);
        } else {
            warn!("No application running");
        }
    }
}

impl Default for Processor {
    fn default() -> Self {
        Self::new()
    }
}
