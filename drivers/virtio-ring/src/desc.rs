use volatile::Volatile;

#[repr(C, align(16))]
pub struct Descriptor {
    addr: Volatile<u64>,
    len: Volatile<u32>,
    flags: Volatile<DescriptorFlags>,
    next: Volatile<u16>,
}

bitflags! {
    struct DescriptorFlags: u16 {
        const NEXT = 1;
        const WRITE = 2;
        const INDIRECT = 4;
    }
}
