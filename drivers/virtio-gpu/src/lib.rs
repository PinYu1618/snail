use drivers::display::DisplayResult;
use drivers::display::Display;
use drivers::display::DisplayInfo;
use drivers::display::FrameBuffer;
use drivers::display::Rectangle;
use enumflags2::bitflags;

pub mod cmd;
pub mod format;
pub mod inner;
pub(crate) mod req;

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

#[bitflags]
#[repr(u64)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Feature {
    Virgl = 1 << 0,
    Edid = 1 << 1,
    ResourceUuid = 1 << 2,
    ResourceBlob = 1 << 3,
    ContextInit = 1 << 4,

    RingIndirectDesc = 1 << 28,
    RingEventIdx = 1 << 29,

    Version1 = 1 << 32,
    AccessPlatform = 1 << 33,
    RingPacked = 1 << 34,
    InOrder = 1 << 35,
    OrderPlatform = 1 << 36,
    SrIov = 1 << 37,
}

#[bitflags]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Flag {
    Fence = 1 << 0,
    InfoRingIdx = 1 << 1,
}