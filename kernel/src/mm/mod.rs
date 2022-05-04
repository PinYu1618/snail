pub mod addr;
pub mod frame;
pub mod heap;
pub mod map;
pub mod memset;
pub mod page;

// Re-export
pub use addr::{PhysAddr, PhysPageNr, VPNRange, VirtAddr, VirtPageNr};
pub use frame::{FrameAllocator, FrameTracker};
pub use map::{MapArea, MapPermission, MapType};
pub use memset::*;
pub use page::{PageTable, PageTableEntry, UserBuffer};

pub fn init() {
    heap::init();
    #[test_case]
    heap::heap_test();

    FrameAllocator::init();
    #[test_case]
    frame::test_frame_allocator();

    memset::KSPACE.exclusive_access().init();
    #[test_case]
    memset::test_remap();
    info!("mm init done!");
}
