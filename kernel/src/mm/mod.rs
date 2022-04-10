pub mod addr;
pub mod frame;
pub mod heap;
pub mod map;
pub mod memset;
pub mod page;

use log::info;
use memset::KSPACE;

pub fn init() {
    heap::init();
    #[cfg(dbg)]
    heap::heap_test();

    frame::init();
    #[cfg(dbg)]
    frame::test_frame_allocator();

    KSPACE.exclusive_access().init();
    #[cfg(dbg)]
    memset::test_remap();
    info!("mm init done!");
}
