use crate::sbi::set_timer;
use riscv::register::time;

const TICKS_PER_SEC: usize = 100;
const K_PER_SEC: usize = 1_000;

pub struct Timer;

impl Timer {
    pub fn get_time() -> usize {
        time::read()
    }
    
    pub fn set_next_trigger() {
        set_timer(Self::get_time() + CLOCK_FREQ / TICKS_PER_SEC);
    }
    
    pub fn get_time_ms() -> usize {
        time::read() / (CLOCK_FREQ / K_PER_SEC)
    }
}

const CLOCK_FREQ: usize = 100;