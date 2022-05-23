use crate::{
    fs,
    mm::PageTable,
    task_::TaskCtrller,
    task_::Processor,
    timer::Timer,
};
use alloc::string::String;
use alloc::vec::Vec;
use alloc::sync::Arc;

pub fn sys_exit(_xstate: i32) -> ! {
    todo!()
}

/// func:
///     app yields its rights of using cpu, and switch to next task
/// ret:
///     always 0
/// id: 124
pub fn sys_yield() -> isize {
    Processor::suspend_current_and_run_next();
    0
}

pub fn sys_get_time() -> isize {
    Timer::get_time_ms() as isize
}

pub fn sys_getpid() -> isize {
    Processor::current_process().unwrap().pid.0 as isize
}

/// func:
///     current process fork a child process
/// ret:
///     0 for child process, child PID for current process
/// id: 220
pub fn sys_fork() -> isize {
    let current_task = Processor::current_process().unwrap();
    let new_task = current_task.fork();
    let new_pid = new_task.pid.0;
    // modify trap context of new_task, because it returns immediately after switching
    let trap_cx = new_task.inner_exclusive_access().trap_cx();
    // we do not have to move to next instruction since we have done it before
    // for child process, fork returns 0
    trap_cx.x[10] = 0;
    // add new task to scheduler
    TaskCtrller::add_task(new_task);
    new_pid as isize
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
pub fn sys_exec(path: *const u8, mut args: *const usize) -> isize {
    let token = Processor::current_user_token();
    let path = PageTable::translated_str(token, path);
    let mut args_vec: Vec<String> = Vec::new();
    loop {
        let arg_str_ptr = *PageTable::translated_ref(token, args);
        if arg_str_ptr == 0 {
            break;
        }
        args_vec.push(PageTable::translated_str(token, arg_str_ptr as *const u8));
        unsafe {
            args = args.add(1);
        }
    }
    if let Some(app_inode) = fs::open_file(path.as_str(), fs::OpenFlags::RDONLY) {
        let all_data = app_inode.read_all();
        let task = Processor::current_process().unwrap();
        let argc = args_vec.len();
        task.exec(all_data.as_slice(), args_vec);
        // return argc because cx.x[10] will be covered with it later
        argc as isize
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
pub fn sys_waitpid(pid: isize, exit_code_ptr: *mut i32) -> isize {
    let task = Processor::current_process().unwrap();
    // find a child process

    // ---- access current PCB exclusively
    let mut inner = task.inner_exclusive_access();
    if !inner
        .children
        .iter()
        .any(|p| pid == -1 || pid as usize == p.pid())
    {
        return -1;
        // ---- release current PCB
    }
    let pair = inner.children.iter().enumerate().find(|(_, p)| {
        // ++++ temporarily access child PCB exclusively
        p.inner_exclusive_access().is_zombie() && (pid == -1 || pid as usize == p.pid())
        // ++++ release child PCB
    });
    if let Some((idx, _)) = pair {
        let child = inner.children.remove(idx);
        // confirm that child will be deallocated after being removed from children list
        assert_eq!(Arc::strong_count(&child), 1);
        let found_pid = child.pid();
        // ++++ temporarily access child PCB exclusively
        let exit_code = child.inner_exclusive_access().exit_code;
        // ++++ release child PCB
        *PageTable::translated_refmut(inner.memset.token(), exit_code_ptr) = exit_code;
        found_pid as isize
    } else {
        -2
    }
    // ---- release current PCB automatically
}
