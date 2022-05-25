use crate::fs::File;
use crate::mm::UserBuffer;
use hal::sbi::legacy_console_getchar;
use crate::task_::Processor;

/// Stdin.
pub struct Stdin;
pub struct Stderr;

impl File for Stdin {
    fn read(&self, mut buf: UserBuffer) -> usize {
        assert_eq!(buf.len(), 1);
        let mut c: usize;
        loop {
            c = legacy_console_getchar();
            if c == 0 {
                Processor::suspend_current_and_run_next();
                continue;
            } else {
                break;
            }
        }
        let ch = c as u8;
        unsafe { buf.buffers[0].as_mut_ptr().write_volatile(ch); }
        1
    }

    fn write(&self, _buf: UserBuffer) -> usize {
        panic!("Cannot write to Stdin.");
    }

    fn readable(&self) -> bool {
        true
    }

    fn writable(&self) -> bool {
        false
    }
}

/// Stdout.
pub struct Stdout;

impl File for Stdout {
    fn read(&self, _buf: UserBuffer) -> usize {
        panic!("Read is not supported for stdout");
    }

    fn write(&self, buf: UserBuffer) -> usize {
        for buffer in buf.buffers.iter() {
            print!("{}", core::str::from_utf8(*buffer).unwrap());
        }
        buf.len()
    }

    fn readable(&self) -> bool {
        false
    }

    fn writable(&self) -> bool {
        true
    }
}

impl File for Stderr {
    fn read(&self, _buf: UserBuffer) -> usize {
        todo!()
    }

    fn write(&self, _buf: UserBuffer) -> usize {
        todo!()
    }

    fn readable(&self) -> bool {
        todo!()
    }

    fn writable(&self) -> bool {
        todo!()
    }
}
