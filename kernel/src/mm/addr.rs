use crate::config::PAGE_SZ_BITS;
use crate::config::PAGE_SZ;

const PA_WIDTH_SV39: usize = 56;
const PPN_WIDTH_SV39: usize = PA_WIDTH_SV39 - PAGE_SZ_BITS;    // 56 - 12
const VA_WIDTH_SV39: usize = 39;
const VPN_WIDTH_SV39: usize = VA_WIDTH_SV39 - PAGE_SZ_BITS;    // 39 - 12

#[repr(C)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct PhysAddr(usize);

#[repr(C)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct PhysPageNr(usize);

#[repr(C)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct VirtAddr(usize);

#[repr(C)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct VirtPageNr(usize);

impl PhysAddr {
    pub fn page_offset(&self) -> usize {
        self.as_usize() & (PAGE_SZ - 1)
    }
    pub fn floor(&self) -> PhysPageNr {
        PhysPageNr(self.as_usize() / PAGE_SZ)
    }
    pub fn ceil(&self) -> PhysPageNr {
        PhysPageNr((self.as_usize() + PAGE_SZ - 1) / PAGE_SZ)
    }
    pub fn as_usize(&self) -> usize { self.0 }
}


// conversions from { pa, ppn, va, vpn } to usize

impl From<PhysAddr> for usize {
    fn from(pa: PhysAddr) -> Self { pa.0 }
}
impl From<PhysPageNr> for usize {
    fn from(ppn: PhysPageNr) -> Self { ppn.0 }
}
impl From<VirtAddr> for usize {
    fn from(va: VirtAddr) -> Self { va.0 }
}
impl From<VirtPageNr> for usize {
    fn from(vpn: VirtPageNr) -> Self { vpn.0 }
}

// conversions from usize to { pa, ppn, va, vpn }

impl From<usize> for PhysAddr {
    fn from(v: usize) -> Self {
        Self(v & ( (1 << PA_WIDTH_SV39) - 1))
    }
}
impl From<usize> for PhysPageNr {
    fn from(v: usize) -> Self {
        Self(v & ( (1 << PPN_WIDTH_SV39) - 1))
    }
}
impl From<usize> for VirtAddr {
    fn from(v: usize) -> Self {
        Self(v & ( (1 << VA_WIDTH_SV39) - 1))
    }
}
impl From<usize> for VirtPageNr {
    fn from(v: usize) -> Self {
        Self(v & ( (1 << VPN_WIDTH_SV39) - 1))
    }
}

// conversions between pa and ppn

impl From<PhysAddr> for PhysPageNr {
    fn from(v: PhysAddr) -> Self {
        assert_eq!(v.page_offset(), 0);
        v.floor()
    }
}
impl From<PhysPageNr> for PhysAddr {
    fn from(v: PhysPageNr) -> Self { Self(v.0 << PAGE_SZ_BITS) }
}


// not sure if this is required

impl PhysPageNr {
    pub fn bytes_arr(&self) -> &'static mut [u8] {
        let pa: PhysAddr = (*self).into();
        unsafe {
            core::slice::from_raw_parts_mut(
                pa.as_usize() as *mut u8,
                PAGE_SZ,
            )
        }
    }
    pub fn as_usize(&self) -> usize { self.0 }
}

impl VirtAddr {
    pub fn as_usize(&self) -> usize { self.0 }
}

impl VirtPageNr {
    pub fn as_usize(&self) -> usize { self.0 }
}