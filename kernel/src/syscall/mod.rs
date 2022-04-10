pub mod fs;
pub mod process;

use fs::*;
use process::*;

const NR_OPEN: usize = 56;
const NR_CLOSE: usize = 57;
const NR_READ: usize = 63;
const NR_WRITE: usize = 64;
const NR_EXIT: usize = 93;
const NR_GET_TIME: usize = 169;
const NR_YIELD: usize = 124;
const NR_FORK: usize = 220;
const NR_EXEC: usize = 221;
const NR_WAITPID: usize = 260;
const NR_TASK_INFO: usize = 410;
const NR_FSTAT: usize = 80;
const NR_UNLINKAT: usize = 35;
const NR_LINKAT: usize = 37;

pub fn syscall(id: usize, args: [usize; 3]) -> isize {
    match id {
        NR_OPEN => sys_open(args[0] as *const u8, args[1] as u32),
        NR_CLOSE => sys_close(args[0]),
        NR_READ => sys_read(args[0], args[1] as *const u8, args[2]),
        NR_WRITE => sys_write(args[0], args[1] as *const u8, args[2]),
        NR_EXIT => sys_exit(args[0] as i32),
        NR_YIELD => sys_yield(),
        NR_FORK => sys_fork(),
        _ => panic!("[kernel] Unsupported syscall_id: {}", id),
    }
}
