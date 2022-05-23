pub struct SignalAction {
    pub handler: usize,
    pub mask: SignalMask,
}

pub struct SignalMask;

bitflags! {
    pub struct Signal: u32 {
        const SIGINT = 1 << 2;
        const SIGILL = 1 << 4;
        const SIGABRT = 1 << 6;
        const SIGFPE = 1 << 8;
        const SIGSEGV = 1 << 11;
    }
}

pub fn sys_sigaction(_signum: i32, _action: *const SignalAction, _old_action: *mut SignalAction) -> isize {
    todo!()
}

pub fn sys_kill(_pid: usize, _signum: i32) -> isize {
    todo!()
}