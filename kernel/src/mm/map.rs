use alloc::collections::BTreeMap;

use crate::{config::PAGE_SZ, mm::addr::Step};

use super::addr::{PhysPageNr, VPNRange, VirtAddr, VirtPageNr};
use super::{
    frame::{alloc_frame, FrameTracker},
    page::{PTEFlags, PageTable},
};

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

#[derive(Clone, Debug)]
pub struct MapArea {
    pub vpn_range: VPNRange,
    data_frames: BTreeMap<VirtPageNr, FrameTracker>,
    map_type: MapType,
    map_perm: MapPermission,
}

impl MapArea {
    pub fn new(sva: VirtAddr, eva: VirtAddr, map_type: MapType, map_perm: MapPermission) -> Self {
        let svpn: VirtPageNr = sva.floor();
        let evpn: VirtPageNr = eva.ceil();
        Self {
            vpn_range: VPNRange::new(svpn, evpn),
            data_frames: BTreeMap::new(),
            map_type,
            map_perm,
        }
    }

    // data: start-aligned but maybe with shorter length
    // assume that all frames were cleared before
    pub fn copy_data(&mut self, pt: &mut PageTable, data: &[u8]) {
        assert_eq!(self.map_type, MapType::Framed);
        let mut start: usize = 0;
        let mut current_vpn = self.vpn_range.start();
        let len = data.len();
        loop {
            let src = &data[start..len.min(start + PAGE_SZ)];
            let dst = &mut pt.translate(current_vpn).unwrap().ppn().bytes_arr()[..src.len()];
            dst.copy_from_slice(src);
            start += PAGE_SZ;
            if start >= len {
                break;
            }
            current_vpn.step();
        }
    }

    pub fn map(&mut self, pt: &mut PageTable) {
        for vpn in self.vpn_range {
            self.map_one(pt, vpn);
        }
    }

    pub fn map_one(&mut self, pt: &mut PageTable, vpn: VirtPageNr) {
        let ppn: PhysPageNr;
        match self.map_type {
            MapType::Identical => {
                ppn = PhysPageNr::from(vpn.as_usize());
            }
            MapType::Framed => {
                let frame = alloc_frame().unwrap();
                ppn = frame.ppn;
                self.data_frames.insert(vpn, frame);
            }
        }
        let pte_flags = PTEFlags::from_bits(self.map_perm.bits()).unwrap();
        pt.map(vpn, ppn, pte_flags);
    }

    pub fn from_another(another: &MapArea) -> Self {
        Self {
            vpn_range: VPNRange::new(another.vpn_range.start(), another.vpn_range.end()),
            data_frames: BTreeMap::new(),
            map_type: another.map_type,
            map_perm: another.map_perm,
        }
    }
}
