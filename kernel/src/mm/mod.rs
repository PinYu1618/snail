mod heap;
pub mod addr;
mod frame;
pub mod page;
pub mod memset;

pub use addr::{ PhysAddr, VirtPageNr, PhysPageNr };
pub use frame::{ FrameTracker, StackFrameAllocator, alloc_frame };
pub use page::{ PageTable, PageTableEntry, UserBuffer };
pub use memset::{ MapType, MapPermission, MapArea, MemorySet };

pub fn init() {
    heap::init();
    frame::init();
}