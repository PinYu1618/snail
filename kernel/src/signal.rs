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

pub fn sys_sigaction(signum: i32, action: *const SignalAction, old_action: *mut SignalAction) -> isize {
    todo!()
}

pub fn sys_kill(pid: usize, signum: i32) -> isize {
    todo!()
}