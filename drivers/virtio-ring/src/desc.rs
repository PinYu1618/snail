use enumflags2::bitflags;
use enumflags2::BitFlags;
use volatile::Volatile;

#[repr(C, align(16))]
pub struct Desc {
    pub addr: Volatile<u64>,
    pub len: Volatile<u32>,
    pub flags: BitFlags<DescFlags>,
    pub next: Volatile<u16>,
}

#[bitflags]
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DescFlags {
    Next = 1,
    Write = 2,
    Indirect = 4,
}