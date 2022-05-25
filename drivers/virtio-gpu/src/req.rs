use crate::{Rectangle, inner::CtrlHeader, format::ColorFormat};

#[repr(C)]
pub struct ResourceAttachBacking {
    header: CtrlHeader,
    resource_id: u32,
    nr_entries: u32, // always 1
    addr: u64,
    length: u32,
    _padding: u32,
}

#[repr(C)]
pub struct ResourceCreate2D {
    header: CtrlHeader,
    resource_id: u32,
    format: ColorFormat,
    width: u32,
    height: u32,
}

#[repr(C)]
pub struct ResourceFlush {
    pub header: CtrlHeader,
    pub rect: Rectangle,
    pub resource_id: u32,
    pub padding: u32,
}

#[repr(C)]
pub struct SetScanout {
    header: CtrlHeader,
    rect: Rectangle,
    scanout_id: u32,
    resource_id: u32,
}

#[repr(C)]
pub struct TransferToHost2D {
    pub header: CtrlHeader,
    pub rect: Rectangle,
    pub offset: u64,
    pub resource_id: u32,
    pub padding: u32,
}

#[repr(C)]
pub struct UpdateCursor {
    pub header: CtrlHeader,
    pub pos: CursorPosition,
    pub resource_id: u32,
    pub hot_x: u32,
    pub hot_y: u32,
    pub padding: u32,
}

#[repr(C)]
pub struct CursorPosition {
    pub scanout_id: u32,
    pub x: u32,
    pub y: u32,
    pub padding: u32,
}

impl TryFrom<Position2D> for CursorPosition {
    type Error = virtio::Error;
    fn try_from(_value: Position2D) -> Result<Self, Self::Error> {
        todo!()
    }
}

pub struct Position2D {
    pub x: u32,
    pub y: u32,
}

pub struct Geometry2D {
    pub width: u32,
    pub height: u32,
}

impl Position2D {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

impl Default for Position2D {
    fn default() -> Self {
        todo!()
    }
}

#[repr(C)]
pub struct Box {
    pub x: u32,
    pub y: u32,
    pub z: u32,
    pub width: u32,
    pub height: u32,
    pub depth: u32,
}

pub struct Position3D {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

#[repr(C)]
pub struct DisplayInfo {
    pub header: CtrlHeader,
    pub rect: Rectangle,
    // Write here !
}

impl DisplayInfo {
    pub fn size(&self) -> usize {
        todo!()
    }
}