use alloc::sync::Arc;
use lazy_static::lazy_static;
use log::warn;

use crate::{
    fs::inode::{open_file, OpenFlags},
    task::{
        context::ProcessContext,
        ctrl::add_task,
        process::{ProcessCtrlBlock, ProcessStatus},
        processor::schedule,
    },
};

pub mod context;
pub mod ctrl;
pub mod pid;
pub mod process;
pub mod processor;
pub mod switch;

pub fn suspend_current_and_run_next() {
    if let Some(pcb) = processor::take_current_process() {
        // ---- access current pcb exclusively
        let mut pcb_inner = pcb.inner_exclusive_access();
        let process_cx_ptr = &mut pcb_inner.process_cx as *mut ProcessContext;
        // change status to ready
        pcb_inner.status = ProcessStatus::Ready;
        drop(pcb_inner); // why ???
                         // ---- release current pcb

        // push back to ready queue
        add_task(pcb);
        // jump to scheduling cycle
        schedule(process_cx_ptr);
    } else {
        warn!("No application running");
    }
}

lazy_static! {
    pub static ref INITPROC: Arc<ProcessCtrlBlock> = Arc::new({
        let inode = open_file("initproc", OpenFlags::RDONLY).unwrap();
        let v = inode.read_all();
        ProcessCtrlBlock::new(v.as_slice())
    });
}
