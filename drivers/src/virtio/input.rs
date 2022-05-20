mod event {
    pub const SYN: u16 = 0x00;
    pub const KEY: u16 = 0x01;
    pub const MSC: u16 = 0x04;
}

pub enum Event {
    Sync,
    Key,
    Misc,
}

impl Event {
    pub fn as_bits(self) -> u16 {
        match self {
            Event::Sync => event::SYN % 16,
            Event::Key => event::KEY % 16,
            Event::Misc => event::MSC % 16,
        }
    }
}

pub mod key {
    pub const KEY_ESC: u16 = 1;
    pub const KEY_LEFTCTRL: u16 = 29;

    pub const BTN_LEFT: u16 = 0x110;
    pub const BTN_RIGHT: u16 = 0x111;
}

pub enum Key {
    KeyCommon,
    Send,
    Mouse,
}