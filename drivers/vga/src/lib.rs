use drivers::display::ColorFormat;
use drivers::display::Display;
use drivers::display::DisplayInfo;
use drivers::display::DisplayResult;
use drivers::display::FrameBuffer;

pub struct Vga {
    //
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

impl Display for Vga {
    fn info(&self) -> DisplayResult<DisplayInfo> {
        Ok(
            DisplayInfo {
                width: BUFFER_WIDTH,
                height: BUFFER_HEIGHT,
                format: ColorFormat::Rgb888,
                fb_base_va: BASE_PA,
                fb_size: (BUFFER_WIDTH * BUFFER_HEIGHT) as usize,
            }
        )
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

pub const BUFFER_HEIGHT: u32 = 25;
pub const BUFFER_WIDTH: u32 = 80;
pub const BASE_PA: usize = 0xb8000;