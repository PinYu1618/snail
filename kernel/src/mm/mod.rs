mod heap;
pub mod addr;
pub mod frame;
pub mod page;
pub mod memset;

pub use addr::{ PhysAddr, VirtPageNr, PhysPageNr };
pub use frame::{ FrameTracker, StackFrameAllocator };
pub use page::{ PageTable, PageTableEntry, UserBuffer };
pub use memset::{ MapType, MapPermission, MapArea, MemorySet };

pub fn init() {
    heap::init();
}