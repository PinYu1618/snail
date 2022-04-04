pub fn sys_exit(xstate: i32) -> ! {
    unimplemented!()
}

pub fn sys_yield() -> isize {
    unimplemented!()
}

/// func:
///     current process fork a child process
/// ret:
///     0 for child process, child PID for current process
/// id: 220
pub fn sys_fork() -> isize {
    unimplemented!()
}

/// func:
///     clear current process's mem space,
///     load an excecutable file, back to user space then start execute
/// pars:
///     path: name of the excecutable file to load
/// ret:
///     -1 (error)
///     no return (success)
/// id: 221
pub fn sys_exec(path: &str) -> isize {
    unimplemented!()
}

/// func:
///     current process wait for a child process turning to zombie,
///     recycle its resource and get is return
/// pars:
///     pid: child PID to wait for, -1 if wait for any child process
///     exit_code: addr to save child process's return, 0 if don't need to save
/// ret:
///     -1 (child process not exist)
///     -2 (child processes all not finished)
///     child PID (others)
/// id: 260
pub fn sys_waitpid(pid: isize, exit_code: *mut i32) -> isize {
    unimplemented!()
}