const LF: u8 = 0x0au8;    // line flush
const CR: u8 = 0x0du8;    // carriage return
const DL: u8 = 0x7fu8;    // delete
const BS: u8 = 0x08u8;    // backspace

use alloc::string::String;
use snail_user::{console::getchar, fork, waitpid, exec};

#[no_mangle]
pub fn ushell() -> i32 {
    println!("It's snail user shell!");
    let mut line = String::new();
    print!(">> ");
    loop {
        let c = getchar();
        match c {
            LF | CR => {
                println!("");
                if !line.is_empty() {
                    line.push('\0');
                    let pid = fork();
                    if pid == 0 {
                        // child process
                        if exec(line.as_str()) == -1 {
                            println!("[Shell] Error when executing");
                            return -4;
                        }
                        unreachable!()
                    } else {
                        let mut exit_code: i32 = 0;
                        let exit_pid = waitpid(pid as usize, &mut exit_code);
                        assert_eq!(pid, exit_pid);
                        println!(
                            "[Shell] Process {} exited with code {}",
                            pid, exit_code
                        );
                    }
                    line.clear();
                }
            },
            BS | DL => {
                if !line.is_empty() {
                    print!("{}", BS as char);
                    print!(" ");
                    print!("{}", BS as char);
                    line.pop();
                }
            },
            _ => {
                print!("{}", c as char);
                line.push(c as char);
            }
        }
    }
}