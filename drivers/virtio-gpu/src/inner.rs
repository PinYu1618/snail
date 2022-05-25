use virtio::Error;
use virtio::Result;
use crate::Rectangle;
use crate::req;

pub struct Inner {
    pub rect: Rectangle,
}

impl Inner {
    pub fn flush(&mut self) -> Result<()> {
        self.transfer_to_host2d(self.rect, 0, RESOURCE_ID_FB)?;
        self.resource_flush(self.rect, RESOURCE_ID_FB)?;
        Ok(())
    }

    pub fn update_cursor(&mut self, ) {
        todo!()
    }

    pub fn transfer_to_host2d(&mut self, rect: Rectangle, offset: u64, resource_id: u32) -> Result<()> {
        let _rsp: CtrlHeader = self.request(
            req::TransferToHost2D {
                header: CtrlHeader::from(Command::TransferToHost2d),
                rect,
                offset,
                resource_id,
                padding: 0,
            }
        )?;
        todo!()
    }

    pub fn resource_create_2d() {
        todo!()
    }

    pub fn set_scanout() -> Result<()> {
        todo!()
    }

    pub fn resource_flush(&mut self, rect: Rectangle, resource_id: u32) -> Result<()> {
        let _rsp: CtrlHeader = self.request(
            req::ResourceFlush {
                header: CtrlHeader::from(Command::ResourceFlush),
                rect,
                resource_id,
                padding: 0,
            }
        )?;
        todo!()
    }

    pub fn resource_attach_backing(&mut self) -> Result<()> {
        todo!()
    }

    fn request<Req, Resp>(&mut self, _request: Req) -> Result<Resp> {
        todo!()
    }
}
/*
bitflags! {
    struct Features: u64 {
        /// virtgl 3D mode is supported.
        const VIRGL                 = 1 << 0;
        /// EDID is supported.
        const EDID                  = 1 << 1;

        // device independent
        const NOTIFY_ON_EMPTY       = 1 << 24; // legacy
        const ANY_LAYOUT            = 1 << 27; // legacy
        const RING_INDIRECT_DESC    = 1 << 28;
        const RING_EVENT_IDX        = 1 << 29;
        const UNUSED                = 1 << 30; // legacy
        const VERSION_1             = 1 << 32; // detect legacy

        // since virtio v1.1
        const ACCESS_PLATFORM       = 1 << 33;
        const RING_PACKED           = 1 << 34;
        const IN_ORDER              = 1 << 35;
        const ORDER_PLATFORM        = 1 << 36;
        const SR_IOV                = 1 << 37;
        const NOTIFICATION_DATA     = 1 << 38;
    }
}
*/

pub struct CtrlHeader {
    pub type_: u32,
    pub flags: u32,
    pub fence_id: u64,
    pub ctx_id: u64,
    pub ring_idx: u8,
    _padding: u32,
}

#[allow(unused)]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Command {
    GetDisplayInfo = 0x100,
    ResourceCreate2d = 0x101,
    ResourceUnref = 0x102,
    SetScanout = 0x103,
    ResourceFlush = 0x104,
    TransferToHost2d = 0x105,
    ResourceAttachBacking = 0x106,
    ResourceDetachBacking = 0x107,
    GetCapsetInfo = 0x108,
    GetCapset = 0x109,
    GetEdid = 0x10a,

    UpdateCursor = 0x300,
    MoveCursor = 0x301,

    OkNodata = 0x1100,
    OkDisplayInfo = 0x1101,
    OkCapsetInfo = 0x1102,
    OkCapset = 0x1103,
    OkEdid = 0x1104,

    ErrUnspec = 0x1200,
    ErrOutOfMemory = 0x1201,
    ErrInvalidScanoutId = 0x1202,
}

impl From<Command> for CtrlHeader {
    fn from(cmd: Command) -> Self {
        CtrlHeader { type_: cmd as u32, flags: 0, fence_id: 0, ctx_id: 0, ring_idx: 0, _padding: 0 }
    }
}

pub const GPU_FLAG_FENCE: u32 = 1 << 0;

/// Display configuration has changed.
pub const EVENT_DISPLAY: u32 = 1 << 0;

pub const QUEUE_TRANSMIT: usize = 0;
pub const QUEUE_CURSOR: usize = 1;

pub const SCANOUT_ID: u32 = 0;
pub const RESOURCE_ID_CURSOR: u32 = 0xdade;
const RESOURCE_ID_FB: u32 = 0xbabe;