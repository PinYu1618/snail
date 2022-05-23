//! GDT in long mode.
//! 
//! Notes: 
//!     In long mode:
//!         - Code-Segment Descriptors: only L(long), D(default size), DPL fields are used.
//!         - Data-Segment Descriptors: DS, ES, SS are ignored.
//!         - 
//! Ref: amd64 programmer mannual volume 2
//! Ref: blog-os

use crate::CS;
use crate::Descriptor;
use crate::GlobalDescriptorTable;
use crate::load_tss;
use crate::Segment;
use crate::SegmentSelector;
use crate::TaskStateSegment;
use crate::VirtAddr;

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

pub fn init() {
    GDT.0.load();
    unsafe {
        CS::set_reg(GDT.1.code_selector);
        load_tss(GDT.1.tss_selector);
    }
}

lazy_static! {
    static ref GDT: (GlobalDescriptorTable, Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();
        let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
        let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS));
        (gdt, Selectors { code_selector, tss_selector })
    };
}

lazy_static! {
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            const STACK_SIZE: usize = 4096 * 5;
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

            let stack_start = VirtAddr::from_ptr(unsafe { &STACK });
            let stack_end = stack_start + STACK_SIZE;
            stack_end
        };
        tss
    };
}

struct Selectors {
    code_selector: SegmentSelector,
    tss_selector: SegmentSelector,
}