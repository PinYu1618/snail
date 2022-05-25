use enumflags2::bitflags;

#[repr(C)]
#[cfg(target_endian = "little")]
pub struct PackedDescEvent {
    pub off_wrap: u16,
    pub flags: u16,
}

#[repr(C)]
#[cfg(target_endian = "little")]
pub struct PackedDesc {
    pub addr: u64,
    pub len: u32,
    pub id: u16,
    pub flags: u16,
}

#[bitflags]
#[repr(u16)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PackedEventFlag {
    Enable = 1 << 0,
    Disable = 1 << 1,
    Desc =  1 << 2,
}

#[bitflags]
#[repr(u64)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PackedEventFeature {
    WrapCtr = 1 << 15,
}