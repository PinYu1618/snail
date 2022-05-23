use crate::SbiRet;

pub fn init() {
    let clint = Clint::new(0x2000000 as *mut u8);
    rustsbi::init_ipi(clint);
    let clint = Clint::new(0x2000000 as *mut u8);
    rustsbi::init_timer(clint);
}

pub struct Clint {
    base: usize,
}

impl Clint {
    #[inline]
    pub fn new(base: *mut u8) -> Clint {
        Clint { base: base as usize }
    }

    #[inline]
    pub fn get_mtime(&self) -> u64 {
        unsafe {
            let base = self.base as *mut u8;
            core::ptr::read_volatile(base.add(0xbff8) as *mut u64)
        }
    }

    #[inline]
    pub fn set_timer(&self, hart_id: usize, instant: u64) {
        unsafe {
            let base = self.base as *mut u8;
            core::ptr::write_volatile((base.offset(0x4000) as *mut u64).add(hart_id), instant);
        }
    }

    #[inline]
    pub fn send_soft(&self, hart_id: usize) {
        unsafe {
            let base = self.base as *mut u8;
            core::ptr::write_volatile((base as *mut u32).add(hart_id), 1);
        }
    }

    #[inline]
    pub fn clear_soft(&self, hart_id: usize) {
        unsafe {
            let base = self.base as *mut u8;
            core::ptr::write_volatile((base as *mut u32).add(hart_id), 0);
        }
    }

}

impl rustsbi::Timer for Clint {
    #[inline]
    fn set_timer(&self, stime_value: u64) {
        let this_mhartid = riscv::register::mhartid::read();
        self.set_timer(this_mhartid, stime_value);
    }
}

impl rustsbi::Ipi for Clint {
    #[inline]
    fn send_ipi_many(&self, hart_mask: rustsbi::HartMask) -> rustsbi::SbiRet {
        let num_harts = *crate::hart_count::NUM_HARTS.lock();
        for i in 0..num_harts {
            if hart_mask.has_bit(i) {
                self.send_soft(i);
            }
        }
        SbiRet::ok(0)
    }
}