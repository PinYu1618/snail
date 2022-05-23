use volatile::Volatile;

#[repr(C)]
pub struct UsedRing {
    flags: Volatile<u16>,
    idx: Volatile<u16>,
    ring: [UsedElem; 32],       // actual size: queue_size
    avail_event: Volatile<u16>, // unused
}

#[repr(C)]
pub struct UsedElem {
    id: Volatile<u32>,
    len: Volatile<u32>,
}
