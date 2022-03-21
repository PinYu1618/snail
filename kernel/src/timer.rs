use riscv::register::time;

pub fn get_time() -> usize {
    time::read()
}