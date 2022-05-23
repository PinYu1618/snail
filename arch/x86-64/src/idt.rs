use crate::gdt;
use crate::InterruptDescriptorTable;
use crate::InterruptStackFrame;

pub fn init() {
    IDT.load();
}

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(handle_breakpoint);
        unsafe {
            idt.double_fault.set_handler_fn(handle_double_fault)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt
    };
}

extern "x86-interrupt" fn handle_breakpoint(_stack_frame: InterruptStackFrame) {
    todo!()
}

extern "x86-interrupt" fn handle_double_fault(stack_frame: InterruptStackFrame, _error_code: u64) -> ! {
    panic!("Double Fault\n:{:#?}", stack_frame)
}