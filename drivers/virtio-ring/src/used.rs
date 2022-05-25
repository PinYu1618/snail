use enumflags2::bitflags;
use enumflags2::BitFlags;
use volatile::Volatile;

#[repr(C, align(4))]
pub struct Used {
    pub flags: BitFlags<UsedFlags>,
    idx: Volatile<u16>,
    ring: [UsedElem; 32],       // actual size: queue_size
    avail_event: Volatile<u16>, // unused
}

#[repr(C, align(4))]
pub struct UsedElem {
    id: Volatile<u32>,
    len: Volatile<u32>,
}

#[bitflags]
#[repr(u16)]
#[derive(Clone, Copy, PartialEq)]
pub enum UsedFlags {
    NoNotify = 1,
}