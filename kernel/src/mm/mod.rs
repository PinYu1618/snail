mod heap;
mod addr;
mod frame;
pub mod page;
mod memset;


use frame::{ FrameTracker, StackFrameAllocator, alloc_frame };
use log::info;
use page::{ PageTable, PageTableEntry, UserBuffer };
use memset::{ MapType, MapPermission, MapArea, MemorySet, KERNEL_SPACE };

pub fn init() {
    heap::init();
    #[cfg(debug)]
    heap::heap_test();

    frame::init();
    #[cfg(debug)]
    frame::test_frame_allocator();

    KERNEL_SPACE.exclusive_access().init();
    #[cfg(debug)]
    memset::test_remap();
    info!("mm init done!");
}