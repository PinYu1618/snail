use crate::{
    fs::inode::{open_file, OpenFlags},
    mm::page::translated_str,
    task::{
        processor::{current_process, current_user_token},
        suspend_current_and_run_next,
    },
};

pub fn sys_exit(xstate: i32) -> ! {
    unimplemented!()
}

/// func:
///     app yields its rights of using cpu, and switch to next task
/// ret:
///     always 0
/// id: 124
pub fn sys_yield() -> isize {
    suspend_current_and_run_next();
    0
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
pub fn sys_exec(path: *const u8) -> isize {
    let token = current_user_token();
    let path = translated_str(token, path);
    if let Some(app_inode) = open_file(path.as_str(), OpenFlags::RDONLY) {
        let all_data = app_inode.read_all();
        let process = current_process().unwrap();
        process.exec(all_data.as_slice());
        0
    } else {
        -1
    }
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
