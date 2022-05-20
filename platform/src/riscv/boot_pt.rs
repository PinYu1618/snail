#[repr(align(4096))]
pub struct BootPageTable([usize; 512]);

impl BootPageTable {
    pub const ZERO: Self = Self([0; 512]);
}

mod consts {
    pub const MODE_SV39: usize = 8 << 60;
}