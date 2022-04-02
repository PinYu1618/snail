pub mod heap;
pub mod addr;
pub mod frame;
pub mod page;
pub mod memset;

use log::info;
use memset::KERNEL_SPACE;

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