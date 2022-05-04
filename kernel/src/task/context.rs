#[repr(C)]
pub struct ThreadContext {
    /// return address
    pub ra: usize,
    /// kernel stack top
    pub sp: usize,
    /// s0-s11
    pub s: [usize; 12],
}

impl ThreadContext {
    pub fn zero_init() -> Self {
        Self { ra: 0, sp: 0, s: [0; 12], }
    }

    pub fn goto_trap_return(kstack_ptr: usize) -> Self {
        use crate::trap::trap_return;
        Self {
            ra: trap_return as usize,
            sp: kstack_ptr,
            s: [0; 12],
        }
    }
}
