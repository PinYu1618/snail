use bitflags::*;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use super::{ VirtPageNr, FrameTracker, PageTable };

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum MapType {
    Identical,
    Framed,
}

bitflags! {
    pub struct MapPermission: u8 {
        const R = 1 << 1;
        const W = 1 << 2;
        const X = 1 << 3;
        const U = 1 << 4;
    }
}

pub struct MapArea {
    vpn_range: VPNRange,
    data_frames: BTreeMap<VirtPageNr, FrameTracker>,
    map_type: MapType,
    map_perm: MapPermission,
}

pub struct MemorySet {
    page_table: PageTable,
    areas: Vec<MapArea>,
}

impl MemorySet {
    pub fn new_bare() -> Self {
        Self {
            page_table: PageTable::new(),
            areas: Vec::new(),
        }
    }

    // ^TODO: pub fn new_kernel() -> Self {...}
    pub fn new_kernel() {}
}

// ^TODO
struct VPNRange;