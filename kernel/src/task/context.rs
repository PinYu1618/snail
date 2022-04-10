#[derive(Copy, Clone)]
#[repr(C)]
pub struct ProcessContext {
    ra: usize,      // return address
    sp: usize,      // kernel stack top
    s: [usize; 12], // s0~s11
}

impl ProcessContext {
    pub fn zero_init() -> Self {
        Self {
            ra: 0,
            sp: 0,
            s: [0; 12],
        }
    }

    pub fn goto_trap_return(kstack_ptr: usize) -> Self {
        unimplemented!()
    }
}
