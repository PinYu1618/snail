use core::fmt::Debug;

use crate::config::PAGE_SZ;
use crate::config::PAGE_SZ_BITS;

use super::page::PageTableEntry;

const PA_WIDTH_SV39: usize = 56;
const PPN_WIDTH_SV39: usize = PA_WIDTH_SV39 - PAGE_SZ_BITS; // 56 - 12
const VA_WIDTH_SV39: usize = 39;
const VPN_WIDTH_SV39: usize = VA_WIDTH_SV39 - PAGE_SZ_BITS; // 39 - 12

#[repr(C)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct PhysAddr(usize);

#[repr(C)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct PhysPageNr(usize);

#[repr(C)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct VirtAddr(usize);

#[repr(C)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct VirtPageNr(usize);

#[derive(Copy, Clone, Debug)]
pub struct Range<T>
where
    T: Step + Copy + PartialEq + PartialOrd + Debug,
{
    l: T,
    r: T,
}

pub struct RangeIterator<T>
where
    T: Step + Copy + PartialEq + PartialOrd + Debug,
{
    current: T,
    end: T,
}

pub type VPNRange = Range<VirtPageNr>;

pub trait Step {
    fn step(&mut self);
}

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

    pub fn get_ref<T>(&self) -> &'static T {
        unsafe { (self.0 as *const T).as_ref().unwrap() }
    }

    pub fn get_mut<T>(&self) -> &'static mut T {
        unsafe { (self.0 as *mut T).as_mut().unwrap() }
    }

    pub fn as_usize(&self) -> usize {
        self.0
    }
}

impl PhysPageNr {
    pub fn bytes_arr(&self) -> &'static mut [u8] {
        let pa: PhysAddr = (*self).clone().into();
        unsafe { core::slice::from_raw_parts_mut(pa.as_usize() as *mut u8, PAGE_SZ) }
    }

    pub fn pte_arr(&self) -> &'static mut [PageTableEntry] {
        let pa: PhysAddr = (*self).clone().into();
        unsafe { core::slice::from_raw_parts_mut(pa.as_usize() as *mut PageTableEntry, 512) }
    }

    pub fn get_mut<T>(&self) -> &'static mut T {
        let pa: PhysAddr = self.clone().into();
        unsafe { (pa.as_usize() as *mut T).as_mut().unwrap() }
    }

    pub fn as_usize(&self) -> usize {
        self.0
    }
}

impl VirtAddr {
    pub fn page_offset(&self) -> usize {
        self.as_usize() & (PAGE_SZ - 1)
    }

    pub fn floor(&self) -> VirtPageNr {
        VirtPageNr(self.as_usize() / PAGE_SZ)
    }

    pub fn ceil(&self) -> VirtPageNr {
        VirtPageNr((self.as_usize() + PAGE_SZ - 1) / PAGE_SZ)
    }

    pub fn as_usize(&self) -> usize {
        self.0
    }
}

impl VirtPageNr {
    pub fn indexes(&self) -> [usize; 3] {
        let mut vpn = self.as_usize();
        let mut idxs = [0_usize; 3];
        for i in (0..3).rev() {
            idxs[i] = vpn & 511;
            vpn >>= 9;
        }
        idxs
    }

    pub fn as_usize(&self) -> usize {
        self.0
    }
}

// conversions from { pa, ppn, va, vpn } to usize

impl From<PhysAddr> for usize {
    fn from(pa: PhysAddr) -> Self {
        pa.0
    }
}
impl From<PhysPageNr> for usize {
    fn from(ppn: PhysPageNr) -> Self {
        ppn.0
    }
}
impl From<VirtAddr> for usize {
    fn from(va: VirtAddr) -> Self {
        va.0
    }
}
impl From<VirtPageNr> for usize {
    fn from(vpn: VirtPageNr) -> Self {
        vpn.0
    }
}

// conversions from usize to { pa, ppn, va, vpn }

impl From<usize> for PhysAddr {
    fn from(v: usize) -> Self {
        Self(v & ((1 << PA_WIDTH_SV39) - 1))
    }
}
impl From<usize> for PhysPageNr {
    fn from(v: usize) -> Self {
        Self(v & ((1 << PPN_WIDTH_SV39) - 1))
    }
}
impl From<usize> for VirtAddr {
    fn from(v: usize) -> Self {
        Self(v & ((1 << VA_WIDTH_SV39) - 1))
    }
}
impl From<usize> for VirtPageNr {
    fn from(v: usize) -> Self {
        Self(v & ((1 << VPN_WIDTH_SV39) - 1))
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
    fn from(v: PhysPageNr) -> Self {
        Self(v.0 << PAGE_SZ_BITS)
    }
}

// conversions between va and vpn

impl From<VirtAddr> for VirtPageNr {
    fn from(va: VirtAddr) -> Self {
        assert_eq!(va.page_offset(), 0);
        va.floor()
    }
}
impl From<VirtPageNr> for VirtAddr {
    fn from(v: VirtPageNr) -> Self {
        Self(v.0 << PAGE_SZ_BITS)
    }
}

// iterating

impl Step for VirtPageNr {
    fn step(&mut self) {
        self.0 += 1;
    }
}

impl Step for PhysPageNr {
    fn step(&mut self) {
        self.0 += 1;
    }
}

impl<T> Range<T>
where
    T: Step + Copy + PartialEq + PartialOrd + Debug,
{
    pub fn new(start: T, end: T) -> Self {
        assert!(start <= end, "start {:?} > end {:?}!", start, end);
        Self { l: start, r: end }
    }
    pub fn start(&self) -> T {
        self.l
    }
    pub fn end(&self) -> T {
        self.r
    }
}

impl<T> IntoIterator for Range<T>
where
    T: Step + Copy + PartialEq + PartialOrd + Debug,
{
    type Item = T;
    type IntoIter = RangeIterator<T>;
    fn into_iter(self) -> Self::IntoIter {
        RangeIterator::new(self.l, self.r)
    }
}

impl<T> RangeIterator<T>
where
    T: Step + Copy + PartialEq + PartialOrd + Debug,
{
    pub fn new(l: T, r: T) -> Self {
        Self { current: l, end: r }
    }
}

impl<T> Iterator for RangeIterator<T>
where
    T: Step + Copy + PartialEq + PartialOrd + Debug,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.end {
            None
        } else {
            let t = self.current;
            self.current.step();
            Some(t)
        }
    }
}
