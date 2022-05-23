use x86_64::instructions::interrupts;

pub fn enable() {
    interrupts::enable();
}

pub fn disable() {
    interrupts::disable();
}