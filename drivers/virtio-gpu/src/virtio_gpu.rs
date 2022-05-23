use crate::Rectangle;

pub struct Inner {
    pub rect: Rectangle,
}

impl Inner {
    pub fn flush(&mut self) {
        self.transfer_to_host2d(self.rect, 0, RESOURCE_ID_FB);
        self.resource_flush(self.rect, RESOURCE_ID_FB);
    }

    pub fn update_cursor() {
        todo!()
    }

    pub fn transfer_to_host2d(&mut self, _rect: Rectangle, _offset: u64, _resource_id: u32) {
        todo!()
    }

    pub fn resource_create_2d() {
        todo!()
    }

    pub fn resource_flush(&mut self, _rect: Rectangle, _resource_id: u32) {
        todo!()
    }
}

pub const RESOURCE_ID_CURSOR: u32 = 0xdade;
const RESOURCE_ID_FB: u32 = 0xbabe;