mod heap;
pub mod addr;
mod frame;
pub mod page;
pub mod memset;

pub use addr::{ VirtAddr, PhysAddr, VirtPageNr, PhysPageNr, VPNRange };
pub use frame::{ FrameTracker, StackFrameAllocator, alloc_frame };
pub use page::{ PageTable, PageTableEntry, UserBuffer };
pub use memset::{ MapType, MapPermission, MapArea, MemorySet, KERNEL_SPACE };

pub fn init() {
    heap::init();
    frame::init();
    KERNEL_SPACE.exclusive_access().init();
}