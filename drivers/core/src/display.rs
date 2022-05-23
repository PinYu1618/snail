//! Ref: https://github.com/rcore-os/zCore/zcore/drivers/scheme/display.rs

pub enum DisplayError {
    //
}

pub type DisplayResult<T> = Result<T, DisplayError>;

pub trait Display {
    fn info(&self) -> DisplayResult<DisplayInfo>;

    /// Returns the framebuffer.
    fn fb(&self) -> FrameBuffer;

    /// Whether need to flush the frambuffer to screen.
    #[inline]
    fn need_flush(&self) -> bool {
        false
    }

    /// Flush framebuffer to screen.
    #[inline]
    fn flush(&self) -> DisplayResult<()> {
        Ok(())
    }
}

pub struct DisplayInfo {
    pub width: u32,
    pub height: u32,
    pub format: ColorFormat,
    pub fb_base_va: usize,
    pub fb_size: usize,
}

/// Color format for one pixel. `RGB888` means R in bits 16-23, G in bits 8-15 and B in bits 0-7.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ColorFormat {
    Rgb332,
    Rgb565,
    Rgb888,
    Argb8888,
}

pub struct FrameBuffer<'a> {
    raw: &'a mut [u8],
}

#[derive(Debug, Clone, Copy)]
pub struct Rectangle {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RgbColor(u32);

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArgbColor(u32);

impl DisplayInfo {
    /// Number of bytes between each row of the frame buffer.
    #[inline]
    pub const fn pitch(self) -> u32 {
        self.width * self.format.bytes() as u32
    }
}

impl ColorFormat {
    /// Number of bits per pixel.
    #[inline]
    pub const fn depth(self) -> u8 {
        match self {
            ColorFormat::Rgb332 => 8,
            ColorFormat::Rgb565 => 16,
            ColorFormat::Rgb888 => 24,
            ColorFormat::Argb8888 => 32,
        }
    }

    /// Number of bytes per pixel.
    #[inline]
    pub const fn bytes(self) -> u8 {
        self.depth() / 8
    }
}

impl RgbColor {
    #[inline]
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self(((r as u32) << 16) | ((g as u32) << 8) | b as u32)
    }

    #[inline]
    pub const fn r(self) -> u8 {
        (self.0 >> 16) as u8
    }

    #[inline]
    pub const fn g(self) -> u8 {
        (self.0 >> 8) as u8
    }

    #[inline]
    pub const fn b(self) -> u8 {
        self.0 as u8
    }

    #[inline]
    pub const fn raw_value(self) -> u32 {
        self.0
    }
}

impl<'a> core::ops::Deref for FrameBuffer<'a> {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        self.raw
    }
}