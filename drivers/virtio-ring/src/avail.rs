use enumflags2::bitflags;
use enumflags2::BitFlags;
use volatile::Volatile;

#[repr(C, align(2))]
pub struct Avail {
    pub flags: BitFlags<AvailFlags>,
    pub idx: Volatile<u16>,
    ring: [Volatile<u16>; 32],
    used_event: Volatile<u16>,
}

#[bitflags]
#[repr(u16)]
#[derive(Clone, Copy, PartialEq)]
pub enum AvailFlags {
    NoInterrupt = 1,
}