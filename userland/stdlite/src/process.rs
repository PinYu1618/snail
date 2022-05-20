use crate::sys;

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct ExitCode(u8);

pub fn exit(exit_code: i32) -> ! {
    sys::sys_exit(exit_code)
}

pub fn id() -> u32 {
    todo!()
}

pub trait Termination {
    fn report(self) -> ExitCode;
}