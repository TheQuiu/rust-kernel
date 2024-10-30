use volatile::Volatile;
use crate::vga::color::ColorCode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct ScreenChar {
    pub(crate) ascii_character: u8,
    pub(crate) color_code: ColorCode,
}

impl ScreenChar {
    pub fn new(
        ascii_character: u8,
        color_code: ColorCode,
    ) -> Self {
        ScreenChar {
            ascii_character,
            color_code,
        }
    }
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
pub struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

impl Buffer {
    pub fn write_byte(&mut self, row: usize, col: usize, byte: u8, color_code: ColorCode) {
        self.chars[row][col].write(ScreenChar {
            ascii_character: byte,
            color_code,
        });
    }

    pub fn read_byte(&self, row: usize, col: usize) -> u8 {
        self.chars[row][col].read().ascii_character
    }
}