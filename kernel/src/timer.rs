use hal::sbi::set_timer;
use riscv::register::time;

const TICKS_PER_SEC: u64 = 100;
const K_PER_SEC: u64 = 1_000;

pub struct Timer;

impl Timer {
    pub fn get_time() -> u64 {
        time::read() as u64
    }
    
    pub fn set_next_trigger() {
        set_timer(Self::get_time() + CLOCK_FREQ / TICKS_PER_SEC);
    }
    
    pub fn get_time_ms() -> u64 {
        (time::read() as u64) / (CLOCK_FREQ / K_PER_SEC)
    }
}

const CLOCK_FREQ: u64 = 100;