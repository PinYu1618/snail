use riscv::register::time;
use core::time::Duration;

pub fn get_cycle() -> u64 {
    time::read() as u64
}

pub fn now() -> Duration {
    let time = get_cycle();
    Duration::from_nanos(time * 100)
}