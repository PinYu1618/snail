use drivers::display::DisplayResult;
use drivers::display::Display;
use drivers::display::DisplayInfo;
use drivers::display::FrameBuffer;
use drivers::display::Rectangle;

pub mod cmd;
pub mod ctrl_hdr;
pub mod format;
pub mod req;
pub mod virtio_gpu;

pub struct VirtioGpu {
    //
}

impl Display for VirtioGpu {
    fn info(&self) -> DisplayResult<DisplayInfo> {
        todo!()
    }

    fn fb(&self) -> FrameBuffer {
        todo!()
    }

    fn need_flush(&self) -> bool {
        todo!()
    }

    fn flush(&self) -> DisplayResult<()> {
        todo!()
    }
}

pub(crate) use ctrl_hdr::CtrlHeader;